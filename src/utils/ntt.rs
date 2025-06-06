use crate::params::Q;
use sha3::{
    Shake256,
    digest::{ExtendableOutput, Update, XofReader},
};
type Byte = u8;

pub fn RejNTTPoly(seed: &[Byte]) -> [u32 ; 256]
{
    let mut j: usize = 0;
    let mut hasher = Shake256::default();
    let mut a_hat = [0u32 ; 256];
    
    let mut tmp: Option<u32>;
    hasher.update(&seed);
    let mut reader: sha3::digest::core_api::XofReaderCoreWrapper<sha3::Shake256ReaderCore>;
    let mut res = [0u8 ; 3];
    while j < 256 {
        reader = hasher.clone().finalize_xof();
        reader.read(&mut res);
        if let Some(x) = coeffFromThreeBytes(res[0], res[1], res[1]) {
            a_hat[j] = x;
            j += 1;
        };
    
    }
    
    a_hat

}



fn coeffFromThreeBytes(b0: Byte, b1: Byte, b2: Byte) -> Option<u32>
{
    let mut b2prime = b2;
    if b2prime > 127 {
        b2prime -= 128;
    }
    let mut z = u32::from(b2prime) * u32::pow(2, 16) + u32::from(b1) * u32::pow(2,8) + u32::from(b0);
    if z < Q {
        Some(z)
    }
    else {
        None
    }
}


pub fn RejBoundedPoly()

