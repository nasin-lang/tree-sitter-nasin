use cranelift_codegen::ir::Value;
use cranelift_frontend::FunctionBuilder;

// Cranelift's variables are for mutable primitives, immutable primitive can just use values.
// Aggregate values can be either stack slots, if they have known length and are never moved, or
// heap allocated if else, regardless of mutability
/// Holds a Crenelift representation of a value, which can be different depending on type,
/// mutability, scope and sharing
pub enum VariableRef {
    ImmPrimitive(Value),
    // TODO: mutable and aggregate types
}

impl VariableRef {
    pub fn get_value(&self, _builder: &FunctionBuilder) -> Value {
        match self {
            Self::ImmPrimitive(value) => value.clone(),
        }
    }
}
