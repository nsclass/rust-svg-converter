use rayon::{prelude::*, slice::Iter};

// static NUMBERS: &'static [i32] = &[1, 2, 3, 4, 5];
static PATH_SCAN_DIR_LOOKUP: &'static [i8] = &[0, 0, 3, 0, 1, 0, 3, 0, 0, 3, 3, 1, 0, 3, 0, 0];
static PATH_SCAN_HOLE_PATH_LOOKUP: &'static [bool] = &[
    false, false, false, false, false, false, false, true, false, false, false, true, false, true,
    true, false,
];
// PATH_SCAN_COMBINED_LOOKUP[ arr[py][px] ][ dir ] = [nextArrayPyPx, nextDir, deltaPx, deltaPy];
static PATH_SCAN_COMBINED_LOOKUP: &'static [[[i8; 4]; 4]; 16] = &[
    [
        [-1, -1, -1, -1],
        [-1, -1, -1, -1],
        [-1, -1, -1, -1],
        [-1, -1, -1, -1],
    ], // arr[py][px]==0 is invalid
    [
        [0, 1, 0, -1],
        [-1, -1, -1, -1],
        [-1, -1, -1, -1],
        [0, 2, -1, 0],
    ],
    [
        [-1, -1, -1, -1],
        [-1, -1, -1, -1],
        [0, 1, 0, -1],
        [0, 0, 1, 0],
    ],
    [
        [0, 0, 1, 0],
        [-1, -1, -1, -1],
        [0, 2, -1, 0],
        [-1, -1, -1, -1],
    ],
    [
        [-1, -1, -1, -1],
        [0, 0, 1, 0],
        [0, 3, 0, 1],
        [-1, -1, -1, -1],
    ],
    [[13, 3, 0, 1], [13, 2, -1, 0], [7, 1, 0, -1], [7, 0, 1, 0]],
    [
        [-1, -1, -1, -1],
        [0, 1, 0, -1],
        [-1, -1, -1, -1],
        [0, 3, 0, 1],
    ],
    [
        [0, 3, 0, 1],
        [0, 2, -1, 0],
        [-1, -1, -1, -1],
        [-1, -1, -1, -1],
    ],
    [
        [0, 3, 0, 1],
        [0, 2, -1, 0],
        [-1, -1, -1, -1],
        [-1, -1, -1, -1],
    ],
    [
        [-1, -1, -1, -1],
        [0, 1, 0, -1],
        [-1, -1, -1, -1],
        [0, 3, 0, 1],
    ],
    [[11, 1, 0, -1], [14, 0, 1, 0], [14, 3, 0, 1], [11, 2, -1, 0]],
    [
        [-1, -1, -1, -1],
        [0, 0, 1, 0],
        [0, 3, 0, 1],
        [-1, -1, -1, -1],
    ],
    [
        [0, 0, 1, 0],
        [-1, -1, -1, -1],
        [0, 2, -1, 0],
        [-1, -1, -1, -1],
    ],
    [
        [-1, -1, -1, -1],
        [-1, -1, -1, -1],
        [0, 1, 0, -1],
        [0, 0, 1, 0],
    ],
    [
        [0, 1, 0, -1],
        [-1, -1, -1, -1],
        [-1, -1, -1, -1],
        [0, 2, -1, 0],
    ],
    [
        [-1, -1, -1, -1],
        [-1, -1, -1, -1],
        [-1, -1, -1, -1],
        [-1, -1, -1, -1],
    ], // arr[py][px]==15 is invalid
];

#[derive(Debug, Default, Clone)]
pub struct PathPoint {
    row: i32,
    col: i32,
    image_index: i32,
}

impl PathPoint {
    pub fn new(row: i32, col: i32, index: i32) -> Self {
        Self {
            row,
            col,
            image_index: index,
        }
    }
    pub fn row(&self) -> usize {
        self.row as usize
    }
    pub fn col(&self) -> usize {
        self.col as usize
    }
}

#[derive(Debug, Default, Clone)]
pub struct Path {
    paths: Vec<PathPoint>,
}

impl Path {
    pub fn new() -> Self {
        Self { paths: Vec::new() }
    }

    pub fn add_path(&mut self, point: PathPoint) {
        self.paths.push(point)
    }

    pub fn len(&self) -> usize {
        self.paths.len()
    }

    pub fn index_at(&self, idx: usize) -> &PathPoint {
        &self.paths[idx]
    }
}

#[derive(Debug, Default, Clone)]
pub struct ScanPaths {
    paths: Vec<Path>,
}

impl ScanPaths {
    pub fn new(layer: &mut Vec<Vec<i32>>, path_omit: u32) -> Self {
        let height = layer.len();
        let width = layer[0].len();
        let mut paths = vec![Path::default(); width * height];
        for row in 0..height {
            for col in 0..width {
                if (layer[row][col] != 0) && (layer[row][col] != 15) {
                    let current_idx = row * width + col;

                    // Init
                    let mut px = col as i32;
                    let mut py = row as i32;

                    let mut current_paths = Vec::<PathPoint>::new();

                    // fill paths will be drawn, but hole paths are also required to remove unnecessary edge nodes
                    let mut dir = PATH_SCAN_DIR_LOOKUP[layer[py as usize][px as usize] as usize];
                    let hole_path =
                        PATH_SCAN_HOLE_PATH_LOOKUP[layer[py as usize][px as usize] as usize];

                    // Path points loop
                    loop {
                        // New path point
                        let current_point =
                            PathPoint::new(px - 1, py - 1, layer[py as usize][px as usize]);

                        current_paths.push(current_point);

                        // Next: look up the replacement, direction and coordinate changes = clear this cell, turn if required, walk forward
                        let lookup_row = PATH_SCAN_COMBINED_LOOKUP
                            [layer[py as usize][px as usize] as usize][dir as usize];
                        layer[py as usize][px as usize] = lookup_row[0] as i32;
                        dir = lookup_row[1];
                        px += lookup_row[2] as i32;
                        py += lookup_row[3] as i32;

                        // Close path
                        if ((px - 1) == current_paths[0].row() as i32)
                            && ((py - 1) == current_paths[0].col() as i32)
                        {
                            // Discarding 'hole' type paths and paths shorter than pathOmit
                            if (!hole_path) && (current_paths.len() >= path_omit as usize) {
                                paths[current_idx] = Path {
                                    paths: current_paths,
                                };
                            }
                            break;
                        }
                    }
                }
            }
        }
        let paths = paths.into_iter().filter(|path| path.len() > 0).collect();
        ScanPaths { paths }
    }

    pub fn par_values(&self) -> Iter<Path> {
        self.paths.par_iter()
    }
}

#[derive(Debug, Default, Clone)]
pub struct ScanPathList {
    scan_paths: Vec<ScanPaths>,
}

impl ScanPathList {
    pub fn new(scan_paths: Vec<ScanPaths>) -> Self {
        Self { scan_paths }
    }
    pub fn par_values(&self) -> Iter<ScanPaths> {
        self.scan_paths.par_iter()
    }
}
