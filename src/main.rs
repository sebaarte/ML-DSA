//use crate::internal::keygen::hello;

use internal::keygen::keygen_internal;
use sha3::{
    Shake256,
    digest::{ExtendableOutput, Update, XofReader},
};
mod params;
mod utils;


mod bytes;
pub mod internal;

fn main() {
    keygen_internal([0u8; 4]);
}
