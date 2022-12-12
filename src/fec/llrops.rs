use fastfec::Llr;

pub trait LlrMul {
    fn mul(self, rhs: Llr) -> Llr;
}

impl LlrMul for bool {
    fn mul(self, rhs: Llr) -> Llr {
        if self {
            rhs
        } else {
            -rhs
        }
    }
}
