



use keygen::keygen::keygen_internal;
use ndarray::{s, Array, Array2};
use sha3::{
    digest::{generic_array::arr, ExtendableOutput, Update, XofReader}, Shake256
};
use simplelog::{Config, SimpleLogger, TermLogger};

mod params;
mod utils;


mod bytes;
pub mod keygen;

fn main() {
    TermLogger::init(simplelog::LevelFilter::Info, Config::default(),simplelog::TerminalMode::Mixed,simplelog::ColorChoice::Auto);

    //keygen_internal([0u8; 4]);

    let mut array: ndarray::ArrayBase<ndarray::OwnedRepr<i32>, ndarray::Dim<[usize; 1]>> = Array::zeros((4));
    
    let sl = array.as_slice().unwrap();

    println!("{:?}",sl)


}
