#[derive(Debug)]
pub struct ImageData {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u8>,
}

impl ImageData {
    pub fn new(width: usize, height: usize, data: Vec<u8>) -> Self {
        Self {
            width,
            height,
            data,
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn index_at(&self, idx: usize) -> u8 {
        self.data[idx]
    }

    pub fn index_at_row_col(&self, row: usize, col: usize) -> u8 {
        self.data[row * self.width + col]
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_index_at() {
        // 2 x 4 matrix
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let image_data = ImageData::new(4, 2, data);
        assert_eq!(image_data.index_at_row_col(0, 1), 2);
        assert_eq!(image_data.index_at_row_col(1, 2), 7);
    }
}
