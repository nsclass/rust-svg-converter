// Copyright (c) 2020 Nam Seob Seo
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::ops::{Index, IndexMut};
#[derive(Debug)]
pub struct ImageDataArray<T> {
    data: Vec<T>,
    height: usize,
    width: usize,
}

impl<T> ImageDataArray<T> {
    pub fn new(height: usize, width: usize) -> Self {
        let data = Vec::with_capacity(height * width);
        ImageDataArray {
            data,
            height,
            width,
        }
    }
}

impl<T> Index<usize> for ImageDataArray<T> {
    type Output = [T];
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index * self.width..(index + 1) * self.width]
    }
}

impl<T> IndexMut<usize> for ImageDataArray<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index * self.width..(index + 1) * self.width]
    }
}
