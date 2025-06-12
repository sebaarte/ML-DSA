use crate::{
    bytes::Bytes,
    params::{ETA, Q, Q_i32, ZETA},
};
use bit_reverse::BitwiseReverse;
use log::trace;
use ndarray_rand::rand_distr::Zeta;
use sha3::{
    digest::{ExtendableOutput, Update, XofReader}, Shake128, Shake256
};
type Byte = u8;

type OneDArray<T> = ndarray::ArrayBase<ndarray::OwnedRepr<T>, ndarray::Dim<[usize; 1]>>;
type TwoDArray<T> = ndarray::ArrayBase<ndarray::OwnedRepr<T>, ndarray::Dim<[usize; 2]>>;

pub fn RejNTTPoly(seed: &[Byte]) -> [i32; 256] {
    let mut j: usize = 0;
    let mut hasher = Shake256::default();
    let mut a_hat = [0i32; 256];

    let mut tmp: Option<u32>;
    hasher.update(&seed);
    let mut reader: sha3::digest::core_api::XofReaderCoreWrapper<sha3::Shake256ReaderCore>;
    let mut res = [0u8; 3];
    while j < 256 {
        reader = hasher.clone().finalize_xof();
        reader.read(&mut res);
        if let Some(x) = coeffFromThreeBytes(res[0], res[1], res[1]) {
            a_hat[j] = x.try_into().unwrap();
            j += 1;
        };
    }

    a_hat
}

fn coeffFromThreeBytes(b0: Byte, b1: Byte, b2: Byte) -> Option<u32> {
    let mut b2prime = b2;
    // set biggest bit of b2 to 0
    if b2prime > 127 {
        b2prime -= 128;
    }
    // compute b2 * 2^16 + b1 * 2^8 + b0 -> b2||b1||b0
    let mut z =
        u32::from(b2prime) * u32::pow(2, 16) + u32::from(b1) * u32::pow(2, 8) + u32::from(b0);
    if z < Q { Some(z) } else { None }
}



pub fn RejBoundedPoly(seed: &[Byte]) -> [i32; 256] {
    //trace!("Entered RejBoundedPoly");
    let mut j: usize = 0;
    let mut hasher = Shake128::default();
    let mut a: [i32; 256] = [0i32; 256];

    let mut tmp: Option<u32>;
    hasher.update(&seed);
    let mut reader: sha3::digest::core_api::XofReaderCoreWrapper<sha3::Shake128ReaderCore>;
    let mut res = [0u8; 1];
    while j < 256 {
        reader = hasher.clone().finalize_xof();
        reader.read(&mut res);
        let mut z0 = CoeffFromHalfByte((u8::from_be_bytes(res) % 16));
        let mut z1 = CoeffFromHalfByte((u8::from_be_bytes(res) / 16).try_into().unwrap());
        if let Some(x) = z0 {
            a[j] = x.try_into().unwrap();
            j += 1;
        };
        if z1.is_some() && j < 256 {
            a[j] = z1.unwrap().try_into().unwrap();
            j += 1;
        }
    }

    a
}

pub fn CoeffFromHalfByte(halfbyte: u8) -> Option<i8> {
    let halfbyte: i16 = halfbyte.try_into().unwrap();
    if ETA == 2 && halfbyte < 15 {
        Some((2 - (halfbyte % 5)).try_into().unwrap())
    } else {
        if ETA == 4 && halfbyte < 9 {
            Some((4 - halfbyte).try_into().unwrap())
        } else {
            None
        }
    }
}

pub fn NTT(w: [i32; 256]) -> [i32; 256] {
    // copy contents of w into w_hat
    let mut w_hat = w;

    let mut m = 0;
    let mut len = 128;

    while len >= 1 {
        let mut start = 0;
        while start < 256 {
            m += 1;
            let mut z = i32::pow(ZETA, m.swap_bits()) % Q_i32;
            for j in start..start + len - 1 {
                let mut t = z * w_hat[j + len] % Q_i32;
                w_hat[j + len] = (w_hat[j] - t) % Q_i32;
                w_hat[j] = (w_hat[j] + t) % Q_i32;
            }
            start = start + 2 * len;
        }
        len = len / 2;
    }
    w_hat
}

pub fn NTT_inv(w_hat: [i32; 256]) -> [i32; 256] {
    let mut w = w_hat;
    let mut m = 256;
    let mut len = 1;

    while len >= 1 {
        let mut start = 0;
        while start < 256 {
            m -= 1;
            let mut z = -i32::pow(ZETA, m.swap_bits()) % Q_i32;
            for j in start..start + len - 1 {
                let mut t = z * w[j];
                w[j] = (t + w[j + len]) % Q_i32;
                w[j + len] = (t - w[j + len]) % Q_i32;
                w[j + len] = (z * w[j + len]) % Q_i32;
            }
            start = start + 2 * len;
        }
        len = len * 2;
    }
    let mut f = 8347681;
    for j in 0..255 {
        w[j] = f * w[j] % Q_i32;
    }

    w
}
