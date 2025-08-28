use crate::parser::ast::Type;

#[derive(Debug)]
pub struct TypeSystem;

impl TypeSystem {
    pub fn new() -> Self {
        TypeSystem
    }

    pub fn is_compatible(&self, from: &Type, to: &Type) -> bool {
        match (from, to) {
            (Type::Int, Type::Int) => true,
            (Type::Bool, Type::Bool) => true,
            (Type::String, Type::String) => true,
            (Type::Array(a), Type::Array(b)) => self.is_compatible(a, b),
            (Type::Void, Type::Void) => true,
            _ => false,
        }
    }

    pub fn is_comparable(&self, left: &Type, right: &Type) -> bool {
        match (left, right) {
            (Type::Int, Type::Int) => true,
            (Type::Bool, Type::Bool) => true,
            (Type::String, Type::String) => true,
            _ => false,
        }
    }

    pub fn get_default_value(&self, type_: &Type) -> String {
        match type_ {
            Type::Int => "0".to_string(),
            Type::Bool => "0".to_string(), // false
            Type::String => "\"\"".to_string(),
            Type::Array(_) => "[]".to_string(),
            Type::Void => "void".to_string(),
        }
    }
}
