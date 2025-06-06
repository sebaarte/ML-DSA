//use crate::internal::keygen::hello;

use internal::keygen::keygen_internal;
use sha3::{
    Shake256,
    digest::{ExtendableOutput, Update, XofReader},
};
use simplelog::{Config, SimpleLogger, TermLogger};
mod params;
mod utils;


mod bytes;
pub mod internal;

fn main() {
    TermLogger::init(simplelog::LevelFilter::Info, Config::default(),simplelog::TerminalMode::Mixed,simplelog::ColorChoice::Auto);

    keygen_internal([0u8; 4]);
}
