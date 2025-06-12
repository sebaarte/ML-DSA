



use keygen::keygen::keygen_internal;
use ndarray::{array, s, Array, Array2};
use ndarray_rand::{rand_distr::Standard, RandomExt};
use sha3::{
    digest::{generic_array::arr, ExtendableOutput, Update, XofReader}, Shake256
};
use simplelog::{Config, SimpleLogger, TermLogger};

mod params;
mod utils;
mod operation_types;
mod operation_traits;




mod bytes;
pub mod keygen;

fn main() {
    TermLogger::init(simplelog::LevelFilter::Trace, Config::default(),simplelog::TerminalMode::Mixed,simplelog::ColorChoice::Auto);

    keygen_internal(OneDArray::random((64,), Standard));

   
}
