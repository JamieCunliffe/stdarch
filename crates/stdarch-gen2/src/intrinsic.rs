use itertools::Itertools;
use proc_macro2::{Punct, Spacing, TokenStream};
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use serde::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::ops::RangeInclusive;
use std::str::FromStr;

use crate::assert_instr::InstructionAssertionsForBaseType;
use crate::context::{GlobalContext, GroupContext};
use crate::input::{InputSet, InputSetEntry};
use crate::predicate_forms::{DontCareMethod, PredicateForm, PredicationMask, ZeroingMethod};
use crate::{
    assert_instr::InstructionAssertionMethod,
    context::{self, ArchitectureSettings, Context, LocalContext, VariableType},
    expression::{Expression, FnCall, IdentifierType},
    input::IntrinsicInput,
    matching::{KindMatchable, SizeMatchable},
    typekinds::*,
    wildcards::Wildcard,
    wildstring::WildString,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SubstitutionType {
    MatchSize(SizeMatchable<WildString>),
    MatchKind(KindMatchable<WildString>),
}

impl SubstitutionType {
    pub fn get(&mut self, ctx: &LocalContext) -> context::Result<WildString> {
        match self {
            Self::MatchSize(smws) => {
                smws.perform_match(ctx)?;
                Ok(smws.as_ref().clone())
            }
            Self::MatchKind(kmws) => {
                kmws.perform_match(ctx)?;
                Ok(kmws.as_ref().clone())
            }
        }
    }
}

/// Mutability level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AccessLevel {
    /// Immutable
    R,
    /// Mutable
    RW,
}

/// Function signature argument.
///
/// Prepend the `mut` keyword for a mutable argument. Separate argument name
/// and type with a semicolon `:`. Usage examples:
/// - Mutable argument: `mut arg1: *u64`
/// - Immutable argument: `arg2: u32`
#[derive(Debug, Clone, SerializeDisplay, DeserializeFromStr)]
pub struct Argument {
    /// Argument name
    pub name: WildString,
    /// Mutability level
    pub rw: AccessLevel,
    /// Argument type
    pub kind: TypeKind,
}

impl Argument {
    pub fn populate_variables(&self, vars: &mut HashMap<String, (TypeKind, VariableType)>) {
        vars.insert(
            self.name.to_string(),
            (self.kind.clone(), VariableType::Argument),
        );
    }
}

impl FromStr for Argument {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.splitn(2, ':').map(<str>::trim);
        if let Some(mut lhs) = it.next().map(|s| s.split_whitespace()) {
            let lhs_len = lhs.clone().count();
            match (lhs_len, lhs.next(), it.next()) {
                (2, Some("mut"), Some(kind)) => Ok(Argument {
                    name: lhs.next().unwrap().parse()?,
                    rw: AccessLevel::RW,
                    kind: kind.parse()?,
                }),
                (2, Some(ident), _) => Err(format!("invalid {ident:#?} keyword")),
                (1, Some(name), Some(kind)) => Ok(Argument {
                    name: name.parse()?,
                    rw: AccessLevel::R,
                    kind: kind.parse()?,
                }),
                _ => Err(format!("invalid argument `{s}` provided")),
            }
        } else {
            Err(format!("invalid argument `{s}` provided"))
        }
    }
}

impl fmt::Display for Argument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let AccessLevel::RW = &self.rw {
            write!(f, "mut ")?;
        }

        write!(f, "{}: {}", self.name, self.kind)
    }
}

impl ToTokens for Argument {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if let AccessLevel::RW = &self.rw {
            tokens.append(format_ident!("mut"))
        }

        let (name, kind) = (format_ident!("{}", self.name.to_string()), &self.kind);
        tokens.append_all(quote! { #name: #kind })
    }
}

/// Static definition part of the signature. It may evaluate to a constant
/// expression with e.g. `const imm: u64`, or a generic `T: Into<u64>`.
#[derive(Debug, Clone, SerializeDisplay, DeserializeFromStr)]
pub enum StaticDefinition {
    /// Constant expression
    Constant(Argument),
    /// Generic type
    Generic(String),
}

impl StaticDefinition {
    pub fn as_variable(&self) -> Option<(String, (TypeKind, VariableType))> {
        match self {
            StaticDefinition::Constant(arg) => Some((
                arg.name.to_string(),
                (arg.kind.clone(), VariableType::Argument),
            )),
            StaticDefinition::Generic(..) => None,
        }
    }
}

impl FromStr for StaticDefinition {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            s if s.starts_with("const ") => Ok(StaticDefinition::Constant(s[6..].trim().parse()?)),
            s => Ok(StaticDefinition::Generic(s.to_string())),
        }
    }
}

impl fmt::Display for StaticDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StaticDefinition::Constant(arg) => write!(f, "const {arg}"),
            StaticDefinition::Generic(generic) => write!(f, "{generic}"),
        }
    }
}

impl ToTokens for StaticDefinition {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(match self {
            StaticDefinition::Constant(arg) => quote! { const #arg },
            StaticDefinition::Generic(generic) => {
                let generic: TokenStream = generic.parse().expect("invalid Rust code");
                quote! { #generic }
            }
        })
    }
}

/// Function constraints
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Constraint {
    /// Asserts that the given variable equals to any of the given integer values
    AnyI32 {
        variable: String,
        any_values: Vec<i32>,
    },
    /// WildString version of RangeI32. If the string values given for the range
    /// are valid, this gets built into a RangeI32.
    RangeWildstring {
        variable: String,
        range: (WildString, WildString),
    },
    /// Asserts that the given variable's value falls in the specified range
    RangeI32 {
        variable: String,
        range: SizeMatchable<RangeInclusive<i32>>,
    },
    /// Asserts that the number of elements/lanes does not exceed the 2048-bit SVE constraint
    SVEMaxElems {
        variable: String,
        sve_max_elems_type: TypeKind,
    },
    /// Asserts that the number of elements/lanes does not exceed the 128-bit register constraint
    VecMaxElems {
        variable: String,
        vec_max_elems_type: TypeKind,
    },
}

impl Constraint {
    fn variable(&self) -> &str {
        match self {
            Constraint::AnyI32 { variable, .. }
            | Constraint::RangeWildstring { variable, .. }
            | Constraint::RangeI32 { variable, .. }
            | Constraint::SVEMaxElems { variable, .. }
            | Constraint::VecMaxElems { variable, .. } => variable,
        }
    }
    pub fn build(&mut self, ctx: &Context) -> context::Result {
        if let Self::RangeWildstring {
            variable,
            range: (min, max),
        } = self
        {
            min.build_acle(ctx.local)?;
            max.build_acle(ctx.local)?;
            let min = min.to_string();
            let max = max.to_string();
            let min: i32 = min
                .parse()
                .map_err(|_| format!("the minimum value `{min}` is not a valid number"))?;
            let max: i32 = max
                .parse()
                .map_err(|_| format!("the maximum value `{max}` is not a valid number"))?;
            *self = Self::RangeI32 {
                variable: variable.to_owned(),
                range: SizeMatchable::Matched(RangeInclusive::new(min, max)),
            }
        }

        if let Self::SVEMaxElems {
            sve_max_elems_type: ty,
            ..
        }
        | Self::VecMaxElems {
            vec_max_elems_type: ty,
            ..
        } = self
        {
            if let Some(w) = ty.wildcard() {
                ty.populate_wildcard(ctx.local.provide_type_wildcard(w)?)?;
            }
        }

        if let Self::RangeI32 { range, .. } = self {
            range.perform_match(ctx.local)?;
        }

        let variable = self.variable();
        ctx.local
            .variables
            .contains_key(variable)
            .then_some(())
            .ok_or_else(|| format!("cannot build constraint, could not find variable {variable}"))
    }
}

/// Function signature
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Signature {
    /// Function name
    pub name: WildString,
    /// List of function arguments, leave unset or empty for no arguments
    pub arguments: Vec<Argument>,
    /// Function return type, leave unset for void
    pub return_type: Option<TypeKind>,

    /// List of static definitions, leave unset of empty if not required
    #[serde(default)]
    pub static_defs: Vec<StaticDefinition>,

    /// **Internal use only.**
    /// Condition for which the ultimate function is specific to predicates.
    #[serde(skip)]
    pub is_predicate_specific: bool,

    /// **Internal use only.**
    /// Setting this property will trigger the signature builder to convert any `svbool*_t` to `svbool_t` in the input and output.
    #[serde(skip)]
    pub predicate_needs_conversion: bool,
}

impl Signature {
    pub fn drop_argument(&mut self, arg_name: &WildString) -> Result<(), String> {
        if let Some(idx) = self
            .arguments
            .iter()
            .position(|arg| arg.name.to_string() == arg_name.to_string())
        {
            self.arguments.remove(idx);
            Ok(())
        } else {
            Err(format!("no argument {arg_name} found to drop"))
        }
    }

    pub fn build(&mut self, ctx: &LocalContext) -> context::Result {
        self.name.build_acle(ctx)?;

        if let Some(ref mut return_type) = self.return_type {
            if let Some(w) = return_type.clone().wildcard() {
                return_type.populate_wildcard(ctx.provide_type_wildcard(w)?)?;
            }
        }

        self.arguments
            .iter_mut()
            .try_for_each(|arg| arg.name.build_acle(ctx))?;

        self.arguments
            .iter_mut()
            .filter_map(|arg| {
                arg.kind
                    .clone()
                    .wildcard()
                    .map(|w| (&mut arg.kind, w.clone()))
            })
            .try_for_each(|(ty, w)| ty.populate_wildcard(ctx.provide_type_wildcard(&w)?))
    }

    pub fn fn_name(&self) -> WildString {
        self.name.replace(['[', ']'], "")
    }

    pub fn doc_name(&self) -> String {
        self.name.to_string()
    }
}

impl ToTokens for Signature {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name_ident = format_ident!("{}", self.fn_name().to_string());
        let arguments = self
            .arguments
            .clone()
            .into_iter()
            .map(|mut arg| {
                if arg
                    .kind
                    .vector()
                    .map_or(false, |ty| ty.base_type().is_bool())
                    && self.predicate_needs_conversion
                {
                    arg.kind = TypeKind::Vector(VectorType::make_predicate_from_bitsize(8))
                }
                arg
            })
            .collect_vec();
        let static_defs = &self.static_defs;
        tokens.append_all(quote! { fn #name_ident<#(#static_defs),*>(#(#arguments),*) });

        if let Some(ref return_type) = self.return_type {
            if return_type
                .vector()
                .map_or(false, |ty| ty.base_type().is_bool())
                && self.predicate_needs_conversion
            {
                tokens.append_all(quote! { -> svbool_t })
            } else {
                tokens.append_all(quote! { -> #return_type })
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct LLVMLinkAttribute {
    pub arch: String,
    pub link: String,
}

impl ToTokens for LLVMLinkAttribute {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let LLVMLinkAttribute { arch, link } = self;
        tokens.append_all(quote! {
            #[cfg_attr(target_arch = #arch, link_name = #link)]
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLVMLink {
    /// LLVM link function name without namespace and types,
    /// e.g. `st1` in `llvm.aarch64.sve.st1.nxv4i32`
    pub name: WildString,

    /// LLVM link signature arguments, leave unset if it inherits from intrinsic's signature
    pub arguments: Option<Vec<Argument>>,
    /// LLVM link signature return type, leave unset if it inherits from intrinsic's signature
    pub return_type: Option<TypeKind>,

    /// **Internal use only. Do not set.**
    /// Attribute LLVM links for the function. First element is the architecture it targets,
    /// second element is the LLVM link itself.
    #[serde(skip)]
    pub links: Option<Vec<LLVMLinkAttribute>>,

    /// **Internal use only. Do not set.**
    /// Generated signature from these `arguments` and/or `return_type` if set, and the intrinsic's signature.
    #[serde(skip)]
    pub signature: Option<Box<Signature>>,
}

impl LLVMLink {
    pub fn resolve(&self, cfg: &ArchitectureSettings) -> String {
        self.name
            .starts_with("llvm")
            .then(|| self.name.to_string())
            .unwrap_or_else(|| format!("{}.{}", cfg.llvm_link_prefix, self.name))
    }

    pub fn build_and_save(&mut self, ctx: &mut Context) -> context::Result {
        self.build(ctx)?;

        // Save LLVM link to the group context
        ctx.global.arch_cfgs.iter().for_each(|cfg| {
            ctx.group
                .links
                .insert(self.resolve(cfg), ctx.local.input.clone());
        });

        Ok(())
    }

    pub fn build(&mut self, ctx: &mut Context) -> context::Result {
        let mut sig_name = ctx.local.signature.name.clone();
        sig_name.prepend_str("_");

        let mut sig = Signature {
            name: sig_name,
            arguments: self
                .arguments
                .clone()
                .unwrap_or_else(|| ctx.local.signature.arguments.clone()),
            return_type: self
                .return_type
                .clone()
                .or_else(|| ctx.local.signature.return_type.clone()),
            static_defs: vec![],
            is_predicate_specific: ctx.local.signature.is_predicate_specific,
            predicate_needs_conversion: false,
        };

        sig.build(ctx.local)?;
        self.name.build(ctx.local, TypeRepr::LLVMMachine)?;

        // Add link function name to context
        ctx.local
            .substitutions
            .insert(Wildcard::LLVMLink, sig.fn_name().to_string());

        self.signature = Some(Box::new(sig));
        self.links = Some(
            ctx.global
                .arch_cfgs
                .iter()
                .map(|cfg| LLVMLinkAttribute {
                    arch: cfg.arch_name.to_owned(),
                    link: self.resolve(cfg),
                })
                .collect_vec(),
        );

        Ok(())
    }

    /// Alters all the unsigned types from the signature, as unsupported by LLVM.
    pub fn sanitise_uints(&mut self) {
        let transform = |tk: &mut TypeKind| {
            if let Some(BaseType::Sized(BaseTypeKind::UInt, size)) = tk.base_type() {
                *tk.base_type_mut().unwrap() = BaseType::Sized(BaseTypeKind::Int, *size)
            }
        };

        if let Some(sig) = self.signature.as_mut() {
            for arg in sig.arguments.iter_mut() {
                transform(&mut arg.kind);
            }

            sig.return_type.as_mut().map(transform);
        }
    }

    /// Make a function call to the LLVM link
    pub fn make_fn_call(&self, intrinsic_sig: &Signature) -> context::Result<Expression> {
        let link_sig = self.signature.as_ref().ok_or_else(|| {
            "cannot derive the LLVM link call, as it does not hold a valid function signature"
                .to_string()
        })?;

        if intrinsic_sig.arguments.len() != link_sig.arguments.len() {
            return Err(
                "cannot derive the LLVM link call, the number of arguments does not match"
                    .to_string(),
            );
        }

        let call_args = intrinsic_sig
            .arguments
            .iter()
            .zip(link_sig.arguments.iter())
            .map(|(intrinsic_arg, link_arg)| {
                // Could also add a type check...
                if intrinsic_arg.name == link_arg.name {
                    Ok(Expression::Identifier(
                        intrinsic_arg.name.to_owned(),
                        IdentifierType::Variable,
                    ))
                } else {
                    Err("cannot derive the LLVM link call, the arguments do not match".to_string())
                }
            })
            .try_collect()?;

        Ok(FnCall::new_expression(link_sig.fn_name().into(), call_args))
    }

    /// Given a FnCall, apply all the predicate and unsigned conversions as required.
    pub fn apply_conversions_to_call(
        &self,
        mut fn_call: FnCall,
        ctx: &LocalContext,
    ) -> context::Result<Expression> {
        use BaseType::{Sized, Unsized};
        use BaseTypeKind::{Bool, UInt};
        use VariableType::Argument;

        let convert =
            |method: &str, ex| Expression::MethodCall(Box::new(ex), method.to_string(), vec![]);

        fn_call.1 = fn_call
            .1
            .into_iter()
            .map(|arg| -> context::Result<Expression> {
                if let Expression::Identifier(ref var_name, IdentifierType::Variable) = arg {
                    let (kind, scope) = ctx
                        .variables
                        .get(&var_name.to_string())
                        .ok_or_else(|| format!("invalid variable {var_name:?} being referenced"))?;

                    match (scope, kind.base_type()) {
                        (Argument, Some(Sized(Bool, bitsize))) if *bitsize != 8 => {
                            Ok(convert("into", arg))
                        }
                        (Argument, Some(Sized(UInt, _) | Unsized(UInt))) => {
                            Ok(convert("as_signed", arg))
                        }
                        _ => Ok(arg),
                    }
                } else {
                    Ok(arg)
                }
            })
            .try_collect()?;

        let return_type_requires_conversion = self
            .signature
            .as_ref()
            .and_then(|sig| sig.return_type.as_ref())
            .and_then(|ty| {
                if let Some(Sized(Bool, bitsize)) = ty.base_type() {
                    (*bitsize != 8).then_some(Bool)
                } else if let Some(Sized(UInt, _) | Unsized(UInt)) = ty.base_type() {
                    Some(UInt)
                } else {
                    None
                }
            });

        let fn_call = Expression::FnCall(fn_call);
        match return_type_requires_conversion {
            Some(Bool) => Ok(convert("into", fn_call)),
            Some(UInt) => Ok(convert("as_unsigned", fn_call)),
            _ => Ok(fn_call),
        }
    }
}

impl ToTokens for LLVMLink {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        assert!(
            self.signature.is_some() && self.links.is_some(),
            "expression {self:#?} was not built before calling to_tokens"
        );

        let signature = self.signature.as_ref().unwrap();
        let links = self.links.as_ref().unwrap();
        tokens.append_all(quote! {
            extern "C" {
                #(#links),*
                #signature;
            }
        })
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FunctionVisibility {
    #[default]
    Public,
    Private,
}

/// Whether to generate a load/store test, and which typeset index
/// represents the data type of the load/store target address
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Test {
    #[default]
    #[serde(skip)]
    None, // Covered by `intrinsic-test`
    Load(usize),
    Store(usize),
}

impl Test {
    pub fn get_typeset_index(&self) -> Option<usize> {
        match *self {
            Test::Load(n) => Some(n),
            Test::Store(n) => Some(n),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Safety {
    Safe,
    Unsafe(Vec<UnsafetyComment>),
}

impl Safety {
    /// Return `Ok(Safety::Safe)` if safety appears reasonable for the given `intrinsic`'s name and
    /// prototype. Otherwise, return `Err()` with a suitable diagnostic.
    fn safe_checked(intrinsic: &Intrinsic) -> Result<Self, String> {
        let name = intrinsic.signature.doc_name();
        if name.starts_with("sv") {
            let handles_pointers = intrinsic
                .signature
                .arguments
                .iter()
                .any(|arg| matches!(arg.kind, TypeKind::Pointer(..)));
            if name.starts_with("svld")
                || name.starts_with("svst")
                || name.starts_with("svprf")
                || name.starts_with("svundef")
                || handles_pointers
            {
                let doc = intrinsic.doc.as_ref().map(|s| s.to_string());
                let doc = doc.as_deref().unwrap_or("...");
                Err(format!(
                    "`{name}` has no safety specification, but it looks like it should be unsafe. \
                Consider specifying (un)safety explicitly:

  - name: {name}
    doc: {doc}
    safety:
      unsafe:
        - ...
    ...
"
                ))
            } else {
                Ok(Self::Safe)
            }
        } else {
            Err(format!(
                "Safety::safe_checked() for non-SVE intrinsic: {name}"
            ))
        }
    }

    fn is_safe(&self) -> bool {
        match self {
            Self::Safe => true,
            Self::Unsafe(..) => false,
        }
    }

    fn is_unsafe(&self) -> bool {
        !self.is_safe()
    }

    fn has_doc_comments(&self) -> bool {
        match self {
            Self::Safe => false,
            Self::Unsafe(v) => !v.is_empty(),
        }
    }

    fn doc_comments(&self) -> &[UnsafetyComment] {
        match self {
            Self::Safe => &[],
            Self::Unsafe(v) => v.as_slice(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnsafetyComment {
    Custom(String),
    Uninitialized,
    PointerOffset(GovernedBy),
    PointerOffsetVnum(GovernedBy),
    Dereference(GovernedBy),
    UnpredictableOnFault,
    NonTemporal,
    NoProvenance(String),
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernedBy {
    #[default]
    Predicated,
    PredicatedNonFaulting,
    PredicatedFirstFaulting,
}

impl fmt::Display for GovernedBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Predicated => write!(f, " (governed by `pg`)"),
            Self::PredicatedNonFaulting => write!(
                f,
                " (governed by `pg`, the first-fault register (`FFR`) \
                and non-faulting behaviour)"
            ),
            Self::PredicatedFirstFaulting => write!(
                f,
                " (governed by `pg`, the first-fault register (`FFR`) \
                and first-faulting behaviour)"
            ),
        }
    }
}

impl fmt::Display for UnsafetyComment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Custom(s) => s.fmt(f),
            Self::Uninitialized => write!(
                f,
                "This creates an uninitialized value, and may be unsound (like \
                [`core::mem::uninitialized`])."
            ),
            Self::PointerOffset(gov) => write!(
                f,
                "[`pointer::offset`](pointer#method.offset) safety constraints must \
                be met for the address calculation for each active element{gov}."
            ),
            Self::PointerOffsetVnum(gov) => write!(
                f,
                "[`pointer::offset`](pointer#method.offset) safety constraints must \
                be met for the address calculation for each active element{gov}. \
                In particular, note that `vnum` is scaled by the vector \
                length, `VL`, which is not known at compile time."
            ),
            Self::Dereference(gov) => write!(
                f,
                "This dereferences and accesses the calculated address for each \
                active element{gov}."
            ),
            Self::NonTemporal => write!(
                f,
                "Non-temporal accesses have special memory ordering rules, and \
                [explicit barriers may be required for some applications]\
                (https://developer.arm.com/documentation/den0024/a/Memory-Ordering/Barriers/Non-temporal-load-and-store-pair?lang=en)."
            ),
            Self::NoProvenance(arg) => write!(
                f,
                "Addresses passed in `{arg}` lack provenance, so this is similar to using a \
                `usize as ptr` cast (or [`core::ptr::from_exposed_addr`]) on each lane before \
                using it."
            ),
            Self::UnpredictableOnFault => write!(
                f,
                "Result lanes corresponding to inactive FFR lanes (either before or as a result \
                of this intrinsic) have \"CONSTRAINED UNPREDICTABLE\" values, irrespective of \
                predication. Refer to architectural documentation for details."
            ),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Intrinsic {
    #[serde(default)]
    pub visibility: FunctionVisibility,
    #[serde(default)]
    pub doc: Option<WildString>,
    #[serde(default)]
    pub url: Option<WildString>,
    #[serde(flatten)]
    pub signature: Signature,
    /// Function sequential composition
    pub compose: Vec<Expression>,
    /// Input to generate the intrinsic against. Leave empty if the intrinsic
    /// does not have any variants.
    /// Specific variants contain one InputSet
    #[serde(flatten, default)]
    pub input: IntrinsicInput,
    #[serde(default)]
    pub constraints: Vec<Constraint>,
    /// Additional target features to add to the global settings
    #[serde(default)]
    pub target_features: Vec<String>,
    /// Should the intrinsic be `unsafe`? By default, the generator will try to guess from the
    /// prototype, but it errs on the side of `unsafe`, and prints a warning in that case.
    #[serde(default)]
    pub safety: Option<Safety>,
    #[serde(default)]
    pub substitutions: HashMap<String, SubstitutionType>,
    /// List of the only indices in a typeset that require conversion to signed
    /// when deferring unsigned intrinsics to signed. (optional, default
    /// behaviour is all unsigned types are converted to signed)
    #[serde(default)]
    pub defer_to_signed_only_indices: HashSet<usize>,
    pub assert_instr: Vec<InstructionAssertionMethod>,
    /// Whether we should generate a test for this intrinsic
    #[serde(default)]
    pub test: Test,
    /// Primary base type, used for instruction assertion.
    #[serde(skip)]
    pub base_type: Option<BaseType>,
}

impl Intrinsic {
    pub fn llvm_link(&self) -> Option<&LLVMLink> {
        self.compose.iter().find_map(|ex| {
            if let Expression::LLVMLink(llvm_link) = ex {
                Some(llvm_link)
            } else {
                None
            }
        })
    }

    pub fn llvm_link_mut(&mut self) -> Option<&mut LLVMLink> {
        self.compose.iter_mut().find_map(|ex| {
            if let Expression::LLVMLink(llvm_link) = ex {
                Some(llvm_link)
            } else {
                None
            }
        })
    }

    pub fn generate_variants(&self, global_ctx: &GlobalContext) -> context::Result<Vec<Intrinsic>> {
        let wrap_err = |err| format!("{}: {err}", self.signature.name);

        let mut group_ctx = GroupContext::default();
        self.input
            .variants(self)
            .map_err(wrap_err)?
            .map(|input| {
                self.generate_variant(input.clone(), &mut group_ctx, global_ctx)
                    .map_err(wrap_err)
                    .map(|variant| (variant, input))
            })
            .collect::<context::Result<Vec<_>>>()
            .and_then(|mut variants| {
                variants.sort_by_cached_key(|(_, input)| input.to_owned());

                if variants.is_empty() {
                    let standalone_variant = self
                        .generate_variant(InputSet::default(), &mut group_ctx, global_ctx)
                        .map_err(wrap_err)?;

                    Ok(vec![standalone_variant])
                } else {
                    Ok(variants
                        .into_iter()
                        .map(|(variant, _)| variant)
                        .collect_vec())
                }
            })
    }

    pub fn generate_variant(
        &self,
        input: InputSet,
        group_ctx: &mut GroupContext,
        global_ctx: &GlobalContext,
    ) -> context::Result<Intrinsic> {
        let mut variant = self.clone();

        variant.input.types = vec![InputSetEntry::new(vec![input.clone()])];

        let mut local_ctx = LocalContext::new(input, self);
        let mut ctx = Context {
            local: &mut local_ctx,
            group: group_ctx,
            global: global_ctx,
        };

        variant.pre_build(&mut ctx)?;

        match ctx.local.predicate_form().cloned() {
            Some(PredicateForm::DontCare(method)) => {
                variant.compose = variant.generate_dont_care_pass_through(&mut ctx, method)?
            }
            Some(PredicateForm::Zeroing(method)) => {
                variant.compose = variant.generate_zeroing_pass_through(&mut ctx, method)?
            }
            _ => {
                for idx in 0..variant.compose.len() {
                    let mut ex = variant.compose[idx].clone();
                    ex.build(&variant, &mut ctx)?;
                    variant.compose[idx] = ex;
                }
            }
        };

        variant.post_build(&mut ctx)?;

        if let Some(n_variant_op) = ctx.local.n_variant_op().cloned() {
            variant.generate_n_variant(n_variant_op, &mut ctx)
        } else {
            Ok(variant)
        }
    }

    /// Implement a "zeroing" (_z) method by calling an existing "merging" (_m) method, as required.
    fn generate_zeroing_pass_through(
        &mut self,
        ctx: &mut Context,
        method: ZeroingMethod,
    ) -> context::Result<Vec<Expression>> {
        PredicationMask::try_from(&ctx.local.signature.name)
            .ok()
            .filter(|mask| mask.has_merging())
            .ok_or_else(|| format!("cannot generate zeroing passthrough for {}, no merging predicate form is specified", self.signature.name))?;

        // Determine the function to pass through to.
        let mut target_ctx = ctx.local.clone();
        // Change target function predicate form to merging
        *target_ctx.input.iter_mut()
            .find_map(|arg| arg.predicate_form_mut())
            .expect("failed to generate zeroing pass through, could not find predicate form in the InputSet") = PredicateForm::Merging;

        let mut sig = target_ctx.signature.clone();
        sig.build(&target_ctx)?;

        let args_as_expressions = |arg: &Argument| -> context::Result<Expression> {
            let arg_name = arg.name.to_string();
            match &method {
                ZeroingMethod::Drop { drop } if arg_name == drop.to_string() => {
                    Ok(PredicateForm::make_zeroinitializer(&arg.kind))
                }
                ZeroingMethod::Select { select } if arg_name == select.to_string() => {
                    let pg = sig
                        .arguments
                        .iter()
                        .find_map(|arg| match arg.kind.vector() {
                            Some(ty) if ty.base_type().is_bool() => Some(arg.name.clone()),
                            _ => None,
                        })
                        .ok_or_else(|| {
                            format!("cannot generate zeroing passthrough for {}, no predicate found in the signature for zero selection", self.signature.name)
                        })?;
                    Ok(PredicateForm::make_zeroselector(
                        pg,
                        select.clone(),
                        &arg.kind,
                    ))
                }
                _ => Ok(arg.into()),
            }
        };

        let name: Expression = sig.fn_name().into();
        let args: Vec<Expression> = sig
            .arguments
            .iter()
            .map(args_as_expressions)
            .try_collect()?;
        let statics: Vec<Expression> = sig
            .static_defs
            .iter()
            .map(|sd| sd.try_into())
            .try_collect()?;
        let mut call: Expression = FnCall(Box::new(name), args, statics).into();
        call.build(self, ctx)?;
        Ok(vec![call])
    }

    /// Implement a "don't care" (_x) method by calling an existing "merging" (_m).
    fn generate_dont_care_pass_through(
        &mut self,
        ctx: &mut Context,
        method: DontCareMethod,
    ) -> context::Result<Vec<Expression>> {
        PredicationMask::try_from(&ctx.local.signature.name).and_then(|mask| match method {
            DontCareMethod::AsMerging if mask.has_merging() => Ok(()),
            DontCareMethod::AsZeroing if mask.has_zeroing() => Ok(()),
            _ => Err(format!(
                "cannot generate don't care passthrough for {}, no {} predicate form is specified",
                self.signature.name,
                match method {
                    DontCareMethod::AsMerging => "merging",
                    DontCareMethod::AsZeroing => "zeroing",
                    _ => unreachable!(),
                }
            )),
        })?;

        // Determine the function to pass through to.
        let mut target_ctx = ctx.local.clone();
        // Change target function predicate form to merging
        *target_ctx.input.iter_mut()
            .find_map(|arg| arg.predicate_form_mut())
            .expect("failed to generate don't care passthrough, could not find predicate form in the InputSet") = PredicateForm::Merging;

        let mut sig = target_ctx.signature.clone();
        sig.build(&target_ctx)?;

        // We might need to drop an argument for a zeroing pass-through.
        let drop = match (method, &self.input.predication_methods.zeroing_method) {
            (DontCareMethod::AsZeroing, Some(ZeroingMethod::Drop { drop })) => Some(drop),
            _ => None,
        };

        let name: Expression = sig.fn_name().into();
        let args: Vec<Expression> = sig
            .arguments
            .iter()
            .map(|arg| {
                if Some(arg.name.to_string()) == drop.as_ref().map(|v| v.to_string()) {
                    // This argument is present in the _m form, but missing from the _x form. Clang
                    // typically replaces these with an uninitialised vector, but to avoid
                    // materialising uninitialised values in Rust, we instead merge with a known
                    // vector. This usually results in the same code generation.
                    // TODO: In many cases, it'll be better to use an unpredicated (or zeroing) form.
                    sig.arguments
                        .iter()
                        .filter(|&other| arg.name.to_string() != other.name.to_string())
                        .find_map(|other| {
                            arg.kind.express_reinterpretation_from(&other.kind, other)
                        })
                        .unwrap_or_else(|| PredicateForm::make_zeroinitializer(&arg.kind))
                } else {
                    arg.into()
                }
            })
            .collect();
        let statics: Vec<Expression> = sig
            .static_defs
            .iter()
            .map(|sd| sd.try_into())
            .try_collect()?;
        let mut call: Expression = FnCall(Box::new(name), args, statics).into();
        call.build(self, ctx)?;
        Ok(vec![call])
    }

    /// Implement a "_n" variant based on the given operand
    fn generate_n_variant(
        &self,
        mut n_variant_op: WildString,
        ctx: &mut Context,
    ) -> context::Result<Intrinsic> {
        let mut variant = self.clone();

        n_variant_op.build_acle(ctx.local)?;

        let n_op_arg_idx = variant
            .signature
            .arguments
            .iter_mut()
            .position(|arg| arg.name.to_string() == n_variant_op.to_string())
            .ok_or_else(|| {
                format!(
                    "cannot generate `_n` variant for {}, operand `{n_variant_op}` not found",
                    variant.signature.name
                )
            })?;

        let has_n_wildcard = ctx
            .local
            .signature
            .name
            .wildcards()
            .any(|w| matches!(w, Wildcard::NVariant));

        if !has_n_wildcard {
            return Err(format!("cannot generate `_n` variant for {}, no wildcard {{_n}} was specified in the intrinsic's name", variant.signature.name));
        }

        // Build signature
        variant.signature = ctx.local.signature.clone();
        if let Some(pf) = ctx.local.predicate_form() {
            // WARN: this may break in the future according to the underlying implementation
            // Drops unwanted arguments if needed (required for the collection of arguments to pass to the function)
            pf.post_build(&mut variant)?;
        }

        let sig = &mut variant.signature;

        ctx.local
            .substitutions
            .insert(Wildcard::NVariant, "_n".to_owned());

        let arg_kind = &mut sig.arguments.get_mut(n_op_arg_idx).unwrap().kind;
        *arg_kind = match arg_kind {
            TypeKind::Wildcard(Wildcard::SVEType(idx, None)) => {
                TypeKind::Wildcard(Wildcard::Type(*idx))
            }
            _ => {
                return Err(format!(
                "cannot generate `_n` variant for {}, the given operand is not a valid SVE type",
                variant.signature.name
            ))
            }
        };

        sig.build(ctx.local)?;

        // Build compose
        let name: Expression = self.signature.fn_name().into();
        let args: Vec<Expression> = sig
            .arguments
            .iter()
            .enumerate()
            .map(|(idx, arg)| {
                let ty = arg.kind.acle_notation_repr();
                if idx == n_op_arg_idx {
                    FnCall::new_expression(
                        WildString::from(format!("svdup_n_{ty}")).into(),
                        vec![arg.into()],
                    )
                } else {
                    arg.into()
                }
            })
            .collect();
        let statics: Vec<Expression> = sig
            .static_defs
            .iter()
            .map(|sd| sd.try_into())
            .try_collect()?;
        let mut call: Expression = FnCall(Box::new(name), args, statics).into();
        call.build(self, ctx)?;

        variant.compose = vec![call];
        variant.signature.predicate_needs_conversion = true;

        Ok(variant)
    }

    fn pre_build(&mut self, ctx: &mut Context) -> context::Result {
        self.substitutions
            .iter_mut()
            .try_for_each(|(k, v)| -> context::Result {
                let mut ws = v.get(ctx.local)?;
                ws.build_acle(ctx.local)?;
                ctx.local
                    .substitutions
                    .insert(Wildcard::Custom(k.to_owned()), ws.to_string());
                Ok(())
            })?;

        self.signature.build(ctx.local)?;

        if self.safety.is_none() {
            self.safety = match Safety::safe_checked(self) {
                Ok(safe) => Some(safe),
                Err(err) => {
                    eprintln!("{err}");
                    return Err(format!(
                        "Refusing to infer unsafety for {name}",
                        name = self.signature.doc_name()
                    ));
                }
            }
        }

        if let Some(doc) = &mut self.doc {
            doc.build_acle(ctx.local)?
        }

        // Add arguments to variable tracking
        self.signature
            .arguments
            .iter()
            .for_each(|arg| arg.populate_variables(&mut ctx.local.variables));

        // Add constant expressions to variable tracking
        self.signature
            .static_defs
            .iter()
            .filter_map(StaticDefinition::as_variable)
            .for_each(|(var_name, var_properties)| {
                ctx.local.variables.insert(var_name, var_properties);
            });

        // Pre-build compose expressions
        for idx in 0..self.compose.len() {
            let mut ex = self.compose[idx].clone();
            ex.pre_build(ctx)?;
            self.compose[idx] = ex;
        }

        if !ctx.local.input.is_empty() {
            // We simplify the LLVM link transmute logic by deferring to a variant employing the same LLVM link where possible
            if let Some(link) = self.compose.iter().find_map(|ex| match ex {
                Expression::LLVMLink(link) => Some(link),
                _ => None,
            }) {
                let mut link = link.clone();
                link.build(ctx)?;

                for cfg in ctx.global.arch_cfgs.iter() {
                    let expected_link = link.resolve(cfg);
                    if let Some(target_inputset) = ctx.group.links.get(&expected_link) {
                        self.defer_to_existing_llvm_link(ctx.local, target_inputset)?;
                        break;
                    }
                }
            }
        }

        self.assert_instr
            .iter_mut()
            .try_for_each(|ai| ai.build(ctx))?;

        // Prepend constraint assertions
        self.constraints.iter_mut().try_for_each(|c| c.build(ctx))?;
        let assertions: Vec<_> = self
            .constraints
            .iter()
            .map(|c| ctx.local.make_assertion_from_constraint(c))
            .try_collect()?;
        self.compose.splice(0..0, assertions);

        Ok(())
    }

    fn post_build(&mut self, ctx: &mut Context) -> context::Result {
        if let Some(Expression::LLVMLink(link)) = self.compose.last() {
            let mut fn_call = link.make_fn_call(&self.signature)?;
            // Required to inject conversions
            fn_call.build(self, ctx)?;
            self.compose.push(fn_call)
        }

        if let Some(llvm_link) = self.llvm_link_mut() {
            // Turn all Rust unsigned types into signed
            llvm_link.sanitise_uints();
        }

        if let Some(predicate_form) = ctx.local.predicate_form() {
            predicate_form.post_build(self)?
        }

        // Set for ToTokens<Signature> to display a generic svbool_t
        self.signature.predicate_needs_conversion = true;

        // Set base type kind for instruction assertion
        self.base_type = ctx
            .local
            .input
            .get(0)
            .and_then(|arg| arg.typekind())
            .and_then(|ty| ty.base_type())
            .map(BaseType::clone);

        // Add global target features
        self.target_features = ctx
            .global
            .arch_cfgs
            .iter()
            .flat_map(|cfg| cfg.target_feature.clone())
            .chain(self.target_features.clone())
            .collect_vec();

        Ok(())
    }

    fn defer_to_existing_llvm_link(
        &mut self,
        ctx: &LocalContext,
        target_inputset: &InputSet,
    ) -> context::Result {
        let mut target_ctx = ctx.clone();
        target_ctx.input = target_inputset.clone();

        let mut target_signature = target_ctx.signature.clone();
        target_signature.build(&target_ctx)?;

        let drop_var = if let Some(pred) = ctx.predicate_form().cloned() {
            match pred {
                PredicateForm::Zeroing(ZeroingMethod::Drop { drop }) => Some(drop),
                PredicateForm::DontCare(DontCareMethod::AsZeroing) => {
                    if let Some(ZeroingMethod::Drop { drop }) =
                        self.input.predication_methods.zeroing_method.to_owned()
                    {
                        Some(drop)
                    } else {
                        None
                    }
                }
                _ => None,
            }
        } else {
            None
        };

        let call_method =
            |ex, method: &str| Expression::MethodCall(Box::new(ex), method.to_string(), vec![]);
        let as_unsigned = |ex| call_method(ex, "as_unsigned");
        let as_signed = |ex| call_method(ex, "as_signed");
        let convert_if_required = |w: Option<&Wildcard>, from: &InputSet, to: &InputSet, ex| {
            if let Some(w) = w {
                if let Some(dest_idx) = w.get_typeset_index() {
                    let from_type = from.get(dest_idx);
                    let to_type = to.get(dest_idx);

                    if from_type != to_type {
                        let from_base_type = from_type
                            .and_then(|in_arg| in_arg.typekind())
                            .and_then(|ty| ty.base_type())
                            .map(|bt| bt.kind());
                        let to_base_type = to_type
                            .and_then(|in_arg| in_arg.typekind())
                            .and_then(|ty| ty.base_type())
                            .map(|bt| bt.kind());

                        match (from_base_type, to_base_type) {
                            // Use AsSigned for uint -> int
                            (Some(BaseTypeKind::UInt), Some(BaseTypeKind::Int)) => as_signed(ex),
                            // Use AsUnsigned for int -> uint
                            (Some(BaseTypeKind::Int), Some(BaseTypeKind::UInt)) => as_unsigned(ex),
                            (None, None) => ex,
                            _ => unreachable!("unsupported conversion case from {from_base_type:?} to {to_base_type:?} hit"),
                        }
                    } else {
                        ex
                    }
                } else {
                    ex
                }
            } else {
                ex
            }
        };

        let args = ctx
            .signature
            .arguments
            .iter()
            .filter_map(|arg| {
                let var = Expression::Identifier(arg.name.to_owned(), IdentifierType::Variable);
                if drop_var.as_ref().map(|v| v.to_string()) != Some(arg.name.to_string()) {
                    Some(convert_if_required(
                        arg.kind.wildcard(),
                        &ctx.input,
                        target_inputset,
                        var,
                    ))
                } else {
                    None
                }
            })
            .collect_vec();

        let turbofish = self
            .signature
            .static_defs
            .iter()
            .map(|def| {
                let name = match def {
                    StaticDefinition::Constant(Argument { name, .. }) => name.to_string(),
                    StaticDefinition::Generic(name) => name.to_string(),
                };
                Expression::Identifier(name.into(), IdentifierType::Symbol)
            })
            .collect_vec();

        let ret_wildcard = ctx
            .signature
            .return_type
            .as_ref()
            .and_then(|t| t.wildcard());
        let call = FnCall(Box::new(target_signature.fn_name().into()), args, turbofish).into();

        self.compose = vec![convert_if_required(
            ret_wildcard,
            target_inputset,
            &ctx.input,
            call,
        )];

        Ok(())
    }
}

impl ToTokens for Intrinsic {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let signature = &self.signature;
        let fn_name = signature.fn_name().to_string();
        let target_feature = self.target_features.join(",");
        let safety = self
            .safety
            .as_ref()
            .expect("safety should be determined during `pre_build`");

        if let Some(doc) = &self.doc {
            let mut doc = vec![doc.to_string()];
            let url: String = self.url.as_ref().map_or_else(
                || {
                    format!(
                        "https://developer.arm.com/architectures/instruction-sets/intrinsics/{}",
                        &signature.doc_name()
                    )
                },
                |url| url.to_string(),
            );
            doc.push(String::new());
            doc.push(format!("[Arm's documentation]({})", url));

            if safety.has_doc_comments() {
                doc.push(String::new());
                doc.push("## Safety".to_string());
                for comment in safety.doc_comments() {
                    doc.push(format!("  * {comment}"));
                }
            } else {
                assert!(
                    safety.is_safe(),
                    "{fn_name} is both public and unsafe, and so needs safety documentation"
                );
            }

            tokens.append_all(quote! { #(#[doc = #doc])* });
        } else {
            assert!(
                matches!(self.visibility, FunctionVisibility::Private),
                "{fn_name} needs to be private, or to have documentation."
            );
            assert!(
                !safety.has_doc_comments(),
                "{fn_name} needs a documentation section for its safety comments."
            );
        }

        tokens.append_all(quote! {
            #[inline]
            #[target_feature(enable = #target_feature)]
        });

        if !self.assert_instr.is_empty() {
            InstructionAssertionsForBaseType(&self.assert_instr, &self.base_type.as_ref())
                .to_tokens(tokens)
        }

        match &self.visibility {
            FunctionVisibility::Public => tokens.append_all(quote! { pub }),
            FunctionVisibility::Private => {}
        }
        if safety.is_unsafe() {
            tokens.append_all(quote! { unsafe });
        }
        tokens.append_all(quote! { #signature });
        tokens.append(Punct::new('{', Spacing::Alone));

        let mut body_unsafe = false;
        let mut expressions = self.compose.iter().peekable();
        while let Some(ex) = expressions.next() {
            if !body_unsafe && safety.is_safe() && ex.requires_unsafe_wrapper(&fn_name) {
                body_unsafe = true;
                tokens.append_all(quote! { unsafe });
                tokens.append(Punct::new('{', Spacing::Alone));
            }
            // If it's not the last and not a LLVM link, add a trailing semicolon
            if expressions.peek().is_some() && !matches!(ex, Expression::LLVMLink(_)) {
                tokens.append_all(quote! { #ex; })
            } else {
                ex.to_tokens(tokens)
            }
        }
        if body_unsafe {
            tokens.append(Punct::new('}', Spacing::Alone));
        }

        tokens.append(Punct::new('}', Spacing::Alone));
    }
}