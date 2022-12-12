mod encodertermination;
mod llrops;
mod turbodecoderinput;
mod turboencoderoutput;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CodeRate {
    OneThird,
    OneHalf,
}

impl From<CodeRate> for fastfec::CodeRate {
    fn from(value: CodeRate) -> Self {
        match value {
            CodeRate::OneThird => fastfec::CodeRate { k: 1, n: 3 },
            CodeRate::OneHalf => fastfec::CodeRate { k: 1, n: 2 },
        }
    }
}

pub(crate) use encodertermination::EncoderTermination;
pub use llrops::LlrMul;
pub(crate) use turbodecoderinput::TurboDecoderInput;
pub(crate) use turboencoderoutput::TurboEncoderOutput;
