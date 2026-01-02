//! Type AST nodes

use super::{Spanned, Expr};
use serde::{Deserialize, Serialize};

/// Type representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Type {
    /// 32-bit signed integer
    I32,
    /// 64-bit signed integer
    I64,
    /// 64-bit floating point
    F64,
    /// Boolean
    Bool,
    /// Unit type ()
    Unit,
    /// String type (v0.5 Phase 2)
    String,
    /// Range type (v0.5 Phase 3) - represents start..end
    Range(Box<Type>),
    /// Named type (struct or enum)
    Named(String),
    /// Struct type with fields (resolved after type checking)
    Struct {
        name: String,
        fields: Vec<(String, Box<Type>)>,
    },
    /// Enum type with variants (resolved after type checking)
    Enum {
        name: String,
        variants: Vec<(String, Vec<Box<Type>>)>,
    },
    /// Reference type (v0.5 Phase 5): &T
    Ref(Box<Type>),
    /// Mutable reference type (v0.5 Phase 5): &mut T
    RefMut(Box<Type>),
    /// Fixed-size array type (v0.5 Phase 6): [T; N]
    Array(Box<Type>, usize),
    /// Inline refinement type (v0.2): T{constraints}
    /// e.g., i64{!= 0}, i64{>= lo, <= hi}
    /// The constraints are expressions relative to the refined value
    Refined {
        base: Box<Type>,
        constraints: Vec<Spanned<Expr>>,
    },
}

/// Manual PartialEq implementation for Type
/// Ignores refinement constraints for type equality checks (structural equality)
impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Type::I32, Type::I32) => true,
            (Type::I64, Type::I64) => true,
            (Type::F64, Type::F64) => true,
            (Type::Bool, Type::Bool) => true,
            (Type::Unit, Type::Unit) => true,
            (Type::String, Type::String) => true,
            (Type::Range(a), Type::Range(b)) => a == b,
            (Type::Named(a), Type::Named(b)) => a == b,
            (Type::Struct { name: n1, fields: f1 }, Type::Struct { name: n2, fields: f2 }) => {
                n1 == n2 && f1 == f2
            }
            (Type::Enum { name: n1, variants: v1 }, Type::Enum { name: n2, variants: v2 }) => {
                n1 == n2 && v1 == v2
            }
            (Type::Ref(a), Type::Ref(b)) => a == b,
            (Type::RefMut(a), Type::RefMut(b)) => a == b,
            (Type::Array(t1, s1), Type::Array(t2, s2)) => t1 == t2 && s1 == s2,
            // Refined types are equal if base types are equal
            // (constraints are semantic, not structural)
            (Type::Refined { base: b1, .. }, Type::Refined { base: b2, .. }) => b1 == b2,
            (Type::Refined { base, .. }, other) | (other, Type::Refined { base, .. }) => {
                base.as_ref() == other
            }
            _ => false,
        }
    }
}

impl Eq for Type {}

impl Type {
    /// Get the base type for refined types, or self for non-refined types.
    /// This is useful for type checking arithmetic/comparison operations.
    /// e.g., i64{it > 0}.base_type() returns &Type::I64
    pub fn base_type(&self) -> &Type {
        match self {
            Type::Refined { base, .. } => base.base_type(),
            _ => self,
        }
    }

    /// Check if this type is numeric (i32, i64, f64) including refined numeric types
    pub fn is_numeric(&self) -> bool {
        matches!(self.base_type(), Type::I32 | Type::I64 | Type::F64)
    }

    /// Check if this type is comparable (numeric, bool, string) including refined types
    pub fn is_comparable(&self) -> bool {
        matches!(
            self.base_type(),
            Type::I32 | Type::I64 | Type::F64 | Type::Bool | Type::String
        )
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::I32 => write!(f, "i32"),
            Type::I64 => write!(f, "i64"),
            Type::F64 => write!(f, "f64"),
            Type::Bool => write!(f, "bool"),
            Type::Unit => write!(f, "()"),
            Type::String => write!(f, "String"),
            Type::Range(elem_ty) => write!(f, "Range<{elem_ty}>"),
            Type::Named(name) => write!(f, "{name}"),
            Type::Struct { name, .. } => write!(f, "{name}"),
            Type::Enum { name, .. } => write!(f, "{name}"),
            Type::Ref(inner) => write!(f, "&{inner}"),
            Type::RefMut(inner) => write!(f, "&mut {inner}"),
            Type::Array(elem, size) => write!(f, "[{elem}; {size}]"),
            Type::Refined { base, constraints } => {
                write!(f, "{}{{", base)?;
                for (i, _) in constraints.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "<constraint>")?;
                }
                write!(f, "}}")
            }
        }
    }
}
