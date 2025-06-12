use std::ops::{Index, Mul, Rem};

use crate::{operation_traits::ntt::{self, NTT}, params::{Q_i32, ZETA}};
use bit_reverse::BitwiseReverse;



pub type  Polynomial<T> = ndarray::ArrayBase<ndarray::OwnedRepr<T>, ndarray::Dim<[usize; 1]>>;



pub type  NTTPolynomial<T> = ndarray::ArrayBase<ndarray::OwnedRepr<T>, ndarray::Dim<[usize; 1]>>;





impl NTT for Polynomial<i32> 
{
    type Output = NTTPolynomial<i32>;

    fn ntt(&self) -> Self::Output {
        // copy contents of w into w_hat
        let mut w_hat = self.clone();

        let mut m = 0;
        let mut len = 128;

        while len >= 1 {
            let mut start = 0;
            while start < 256 {
                m += 1;
                let mut z = i32::pow(ZETA, m.swap_bits()) % Q_i32;
                for j in start..start + len - 1 {
                    let mut t:i32  = z * w_hat[j + len] % Q_i32;
                    w_hat[j + len] = (w_hat[j] - t) % Q_i32;
                    w_hat[j] = (w_hat[j] + t) % Q_i32;
                }
                start = start + 2 * len;
            }
            len = len / 2;
        }
        w_hat
    }
}
