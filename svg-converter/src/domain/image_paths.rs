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
    points: Vec<i32>,
}

impl PathPoint {
    pub fn new(row: i32, col: i32, index: i32) -> Self {
        let point = vec![row, col, index];
        Self { points: point }
    }
    pub fn row(&self) -> usize {
        self.points[0] as usize
    }
    pub fn col(&self) -> usize {
        self.points[1] as usize
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
                    let mut px = col;
                    let mut py = row;

                    let mut this_paths = Vec::<PathPoint>::new();

                    // fill paths will be drawn, but hole paths are also required to remove unnecessary edge nodes
                    let mut dir = PATH_SCAN_DIR_LOOKUP[layer[py][px] as usize];
                    let hole_path = PATH_SCAN_HOLE_PATH_LOOKUP[layer[py][px] as usize];

                    // Path points loop
                    loop {
                        // New path point
                        let this_point =
                            PathPoint::new((px - 1) as i32, (py - 1) as i32, layer[py][px]);

                        this_paths.push(this_point);

                        // Next: look up the replacement, direction and coordinate changes = clear this cell, turn if required, walk forward
                        let lookup_row =
                            PATH_SCAN_COMBINED_LOOKUP[layer[py][px] as usize][dir as usize];
                        layer[py][px] = lookup_row[0] as i32;
                        dir = lookup_row[1];
                        px += lookup_row[2] as usize;
                        py += lookup_row[3] as usize;

                        // Close path
                        if ((px - 1) == this_paths[0].row()) && ((py - 1) == this_paths[0].col()) {
                            // Discarding 'hole' type paths and paths shorter than pathOmit
                            if (!hole_path) && (this_paths.len() >= path_omit as usize) {
                                paths[current_idx] = Path { paths: this_paths };
                            }
                            break;
                        }
                    }
                }
            }
        }
        ScanPaths { paths }
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
}
