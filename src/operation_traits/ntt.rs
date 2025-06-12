pub trait NTT{
    type Output;
    fn ntt(&self) -> Self::Output;
}