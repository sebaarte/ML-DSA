use std::iter::{zip, Product};

use generic_array::GenericArray;
use num::PrimInt;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};











pub fn matvecmul<const N: usize>(mat: [[i32;N] ;N],vect : [i32 ; N])
{


    // mat.par_iter().map(| x| VecReduct(x, vect)).collect()


}

fn VecReduct<const N: usize>(vec1: [i32; N],vec2: [i32;N]) -> i32
{
    let tmp = zip(vec1, vec2);
    tmp.into_iter().fold(0, | acc, (x,y)| x+y)
}