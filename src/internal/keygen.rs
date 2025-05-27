use crate::{
    params::{self, K, L},
    utils::ntt::RejNTTPoly,
};
use ndarray::{Array, Array3};
use ndarray_rand::{RandomExt, rand_distr::Standard};

use sha3::{
    Shake256,
    digest::{ExtendableOutput, Update, XofReader},
};

pub fn keygen_internal(seed: [u8; 4]) {
    let mut hasher = Shake256::default();
    hasher.update(&seed);
    hasher.update(&params::K.to_be_bytes());
    hasher.update(&params::L.to_be_bytes());
    let mut reader = hasher.finalize_xof();
    let mut result = [0u8; 16];
    reader.read(&mut result);
    println!("{:?}", result);
    let mut p = &result[0..4];
    let mut p_prime = &result[4..12];
    let mut k = &result[12..16];

    //println!("{:.4}", Array::<u32, _>::random((usize::from(K), usize::from(L)), Standard));

    let mut A_hat = RejNTTPoly(p_prime);
}

pub fn expand_A(seed: [u8; 4]) {
    let mut p_prime = [0u8; 6];
    let mut A_hat = Array::<u32, _>::zeros((usize::from(K), usize::from(L)));
    for r in 0..K - 1 {
        for s in 0..L - 1 {
            p_prime = [&seed[..], &[r], &[s]].concat().try_into().unwrap();
            A_hat[[usize::from(r), usize::from(s)]] = RejNTTPoly(&p_prime);
        }
    }
}
