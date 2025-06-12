use std::{array::from_ref, ops::Index};


pub struct ByteArray<'a> {
    bytes: &'a [u8], 
    size: usize 
}

impl<'a> Index<usize> for ByteArray<'a> {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.size {
            panic!("Index error on Byte array indexing");
        }
        else {
            &self.bytes[index]
        }
    }
}

impl<'a> ByteArray<'a> {
    pub fn get(&self) -> &[u8]
    {
        &self.bytes
    }    
}