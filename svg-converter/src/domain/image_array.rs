use ndarray::{Array2};

pub type ImageArray = Array2<u8>;
pub type QuantizationImageArray = Array2<i32>;

// use std::ops::{Index, IndexMut};
// #[derive(Debug)]
// pub struct ImageArray2 {
//     data: Vec<u8>,
//     rows: usize,
//     cols: usize,
// }

// impl ImageArray2 {
//     pub fn new(rows: usize, cols: usize) -> ImageArray2 {
//         let data = Vec::with_capacity(rows * cols);
//         ImageArray2 {
//             data: data,
//             rows: rows,
//             cols: cols,
//         }
//     }
// }

// impl Index<usize> for ImageArray2 {
//     type Output = [u8];
//     fn index(&self, index: usize) -> &Self::Output {
//         &self.data[index * self.cols .. (index + 1) * self.cols]
//     }
// }

// impl IndexMut<usize> for ImageArray2 {
//     fn index_mut(&mut self, index: usize) -> &mut Self::Output {
//         &mut self.data[index * self.cols .. (index + 1) * self.cols]
//     }
// }