/// Push an i32 value onto the stack.
/// Stack: -> Value
pub const OP_CONST_I32: u8 = 0x01;

/// Push a f32 value onto the stack.
/// Stack: -> Value
pub const OP_CONST_F32: u8 = 0x02;

/// Duplicate the Value on top of the stack
/// Stack: Value -> Value, Value
pub const OP_DUP: u8 = 0x03;

/// Discard the Value on top of the stack.
/// Stack: Value ->
pub const OP_POP: u8 = 0x04;

/// Unconditional jump.
/// Stack: ->
/// Instruction bytes: 1: index byte 1, 2: index byte 2
pub const OP_JMP: u8 = 0x99;

/// Compare two values on the stack and jump to the given instruction label.
/// Stack: Value, Value ->
/// Instruction bytes: 1: index byte 1, 2: index byte 2
pub const OP_IF_EQ_I32: u8 = 0x9F;

// pub const OP_SPREAD_NEW: u8 = 0x04;
// pub const OP_SPREAD_STORE: u8 = 0x05;

/// Load a Value with given index from the constant pool and push it onto the stack.
/// Stack: index -> Value
pub const OP_VALUE_LOAD: u8 = 0x06;

pub const OP_CALL_NODE: u8 = 0x10;

pub const OP_END: u8 = 0xFF;
