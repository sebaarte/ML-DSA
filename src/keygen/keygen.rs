use crate::{
    params::{self, K, KL, L},
    utils::ntt::{NTT, NTT_inv, RejBoundedPoly, RejNTTPoly},
};
use log::{debug, error, info, log, trace};
use ndarray::{Array, Array2, Array3, Shape, arr2};
use ndarray_rand::{RandomExt, rand_distr::Standard};

use sha3::{
    Shake256,
    digest::{ExtendableOutput, Update, XofReader},
};

type OneDArray<T> = ndarray::ArrayBase<ndarray::OwnedRepr<T>, ndarray::Dim<[usize; 1]>>;
type TwoDArray<T> = ndarray::ArrayBase<ndarray::OwnedRepr<T>, ndarray::Dim<[usize; 2]>>;

pub fn keygen_internal(seed: OneDArray<u8>) {
    trace!("Entered key generation");
    let mut hasher = Shake256::default();

    // compute H(seed||k||l) to expand seed
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

    // generate A and store it in NTT representation
    let mut A_hat = expand_A(p);

    let (mut s1, mut s2) = expand_S(p_prime);

    //let mut t = NTT_inv(A_hat * NTT(s1)) + s2;
}

fn expand_A(seed: [u8; 4]) -> TwoDArray<i32> {
    trace!("Entered expand A");
    let mut p_prime = [0u8; 6];
    let mut A_hat = [[0i32; 256]; KL];
    for r in 0..K - 1 {
        for s in 0..L - 1 {
            p_prime[0..4].copy_from_slice(&seed);
            p_prime[8..10].copy_from_slice(&r.to_be_bytes()[6..8]);
            A_hat[usize::from(r) * usize::from(K) + usize::from(s)] = RejNTTPoly(&p_prime);
        }
    }
    let res = arr2(&A_hat);

    res
}

fn expand_S(seed: [u8; 8]) -> (TwoDArray<i32>, TwoDArray<i32>) {
    trace!("Entered expand S");
    let mut s1 = [[0i32; 256]; L];
    let mut s2 = [[0i32; 256]; K];
    let mut p_prime = [0u8; 10];
    for r in 0..(L - 1) {
        p_prime[0..8].copy_from_slice(&seed);
        p_prime[8..10].copy_from_slice(&r.to_be_bytes()[6..8]);
        s1[r] = RejBoundedPoly(&p_prime);
    }

    for r in 0..(K - 1) {
        p_prime[0..8].copy_from_slice(&seed);
        p_prime[8..10].copy_from_slice(&r.to_be_bytes()[6..8]);
        s2[r] = RejBoundedPoly(&p_prime);
    }
    (arr2(&s1), arr2(&s2))
}
