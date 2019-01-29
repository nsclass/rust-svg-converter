
#[derive(Debug)]
pub struct ImageData {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u8>,
}

impl ImageData {
    pub fn new(width: usize, height: usize, data: Vec<u8>) -> ImageData {
        ImageData {
            width: width,
            height: height,
            data: data,
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn index_at(&self, idx: usize) -> u8 {
        self.data[idx]
    }
}