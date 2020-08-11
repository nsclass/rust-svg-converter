use exoquant::Color;
use std::ops::{Index, IndexMut};
#[derive(Debug)]
pub struct ImageData {
    pub height: usize,
    pub width: usize,
    pub data: Vec<u8>,
}

impl Index<usize> for ImageData {
    type Output = [u8];
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index * self.width..(index + 1) * self.width]
    }
}

impl IndexMut<usize> for ImageData {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index * self.width..(index + 1) * self.width]
    }
}

impl ImageData {
    pub fn new(height: usize, width: usize, data: Vec<u8>) -> Self {
        assert!(data.len() % 4 == 0);
        Self {
            height,
            width,
            data,
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn index_at(&self, idx: usize) -> u8 {
        self.data[idx]
    }

    pub fn index_row_col(&self, row: u32, col: u32) -> u8 {
        if 0 > row || row >= self.height as u32 {
            return 0xff;
        }

        if 0 > col || col >= self.width as u32 {
            return 0xff;
        }

        let idx = row as usize * self.height + col as usize;
        return self.data[idx];
    }

    pub fn into_color(self) -> ImageColorData {
        let color_data = self
            .data
            .chunks(4)
            .map(|chunk| Color::new(chunk[0], chunk[1], chunk[2], chunk[3]))
            .collect();
        ImageColorData {
            pixels: color_data,
            height: self.height,
            width: self.width,
        }
    }
}

pub struct ImageColorData {
    pub pixels: Vec<Color>,
    pub height: usize,
    pub width: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_ops() {
        // 2 x 4 matrix
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let image_data = ImageData::new(2, 4, data);
        assert_eq!(image_data[0][1], 2);
        assert_eq!(image_data[1][2], 7);
    }

    #[test]
    fn test_index_mut_ops() {
        // 2 x 4 matrix
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let mut image_data = ImageData::new(2, 4, data);
        image_data[0][1] = 30;
        assert_eq!(image_data[0][1], 30);
        image_data[1][2] = 100;
        assert_eq!(image_data[1][2], 100);
    }
}
