/// Constant values that can show up in WebAssembly
#[derive(Debug, Clone, Copy)]
pub enum Value {
    /// A constant 32-bit integer
    I32(i32),
    /// A constant 64-bit integer
    I64(i64),
    /// A constant 32-bit float
    F32(f32),
    /// A constant 64-bit float
    F64(f64),
    /// A constant 128-bit vector register
    V128(u128),
}

impl Value {
    pub(crate) fn emit(&self, encoder: &mut Encoder) {
        match *self {
            Value::I32(n) => {
                encoder.byte(0x41); // i32.const
                encoder.i32(n);
            }
            Value::I64(n) => {
                encoder.byte(0x42); // i64.const
                encoder.i64(n);
            }
            Value::F32(n) => {
                encoder.byte(0x43); // f32.const
                encoder.f32(n);
            }
            Value::F64(n) => {
                encoder.byte(0x44); // f64.const
                encoder.f64(n);
            }
            Value::V128(n) => {
                encoder.raw(&[0xfd, 0x02]); // v128.const
                for i in 0..16 {
                    encoder.byte((n >> (i * 8)) as u8);
                }
            }
        }
    }
}
