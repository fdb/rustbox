/// Push an i32 value onto the stack.
/// Stack: -> value
pub const OP_CONST_I32: u8 = 0x01;

/// Push a f32 value onto the stack.
/// Stack: -> value
pub const OP_CONST_F32: u8 = 0x02;

/// Duplicate the value on top of the stack
/// Stack: value -> value, value
pub const OP_DUP: u8 = 0x03;

pub const OP_SPREAD_NEW: u8 = 0x04;
pub const OP_SPREAD_STORE: u8 = 0x05;

/// Load a spread with given index from the constant pool and push it onto the stack.
/// Stack: index -> spread
pub const OP_SPREAD_LOAD: u8 = 0x06;

pub const OP_CALL_NODE: u8 = 0x10;

pub const OP_END: u8 = 0xFF;
