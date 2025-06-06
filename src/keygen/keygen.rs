use crate::{
    bytes::Bytes, params::{self, K, KL, L}, utils::ntt::{NTT_inv, RejBoundedPoly, RejNTTPoly}
};
use log::{debug, error, info, log, trace};
use ndarray::{Array, Array3};
use ndarray_rand::{RandomExt, rand_distr::Standard};

use sha3::{
    Shake256,
    digest::{ExtendableOutput, Update, XofReader},
};


type OneDArray<T> = ndarray::ArrayBase<ndarray::OwnedRepr<T>, ndarray::Dim<[usize; 1]>>;

pub fn keygen_internal(seed: OneDArray<u8>) {

    
    trace!("Entered key generation");
    let mut hasher = Shake256::default();
    hasher.update(seed.as_slice().unwrap());
    hasher.update(&params::K.to_be_bytes());
    hasher.update(&params::L.to_be_bytes());

    let mut reader = hasher.finalize_xof();
    let mut result = [0u8; 16];
    reader.read(&mut result);
    let mut p = [0u8; 4];
    p.clone_from_slice(&result[0..4]);
    let mut p_prime = [0u8; 8];
    p_prime.clone_from_slice(&result[4..12]);
    let mut k = [0u8; 4];
    k.clone_from_slice(&result[12..16]);

    //println!("{:.4}", Array::<u32, _>::random((usize::from(K), usize::from(L)), Standard));

    let mut A_hat = expand_A(p);
    let (mut s1, mut s2) = expand_S(p_prime);

    
    

}

fn expand_A(seed: [u8; 4]) -> [[i32; 256]; KL] {
    let mut p_prime = [0u8; 6];
    let mut A_hat = [[0i32; 256]; KL];
    for r in 0..K - 1 {
        for s in 0..L - 1 {
            p_prime = [&seed[..], &[r.try_into().unwrap()], &[s.try_into().unwrap()]].concat().try_into().unwrap();
            A_hat[usize::from(r) * usize::from(K) + usize::from(s)] = RejNTTPoly(&p_prime);
        }
    }
    A_hat
}

fn expand_S(seed: [u8; 8]) -> ([[i32; 256]; L], [[i32; 256]; K]) {
    let mut s1 = [[0i32; 256]; L];
    let mut s2 = [[0i32; 256]; K];
    let mut p_prime = [0u8; 10];
    for r in 0..L - 1 {
        p_prime = [&seed[..], &[r.try_into().unwrap()]].concat().try_into().unwrap();
        s1[r] = RejBoundedPoly(&p_prime);
    }

    for r in 0..K-1 {
        p_prime = [&seed[..], &[r.try_into().unwrap()]].concat().try_into().unwrap();
        s2[r] = RejBoundedPoly(&p_prime);
    }
    (s1, s2)
}
