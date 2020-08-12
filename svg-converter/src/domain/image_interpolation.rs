use crate::Path;

#[derive(Debug, Default, Clone)]
pub struct InterpolationNode {
    points: [f64; 3],
}

impl InterpolationNode {
    pub fn new(points: [f64; 3]) -> Self {
        Self { points: points }
    }
}

#[derive(Debug, Default, Clone)]
pub struct InterpolationNodeList {
    inter_nodes: Vec<InterpolationNode>,
}

impl InterpolationNodeList {
    pub fn new(path: &Path) -> Self {
        let path_len = path.len();
        let mut nodes = vec![InterpolationNode::default(); path_len];

        let mut next_point = [0 as f64; 2];

        // pathPoints loop
        for path_index in 0..path_len {
            // interpolate between two path points
            let next_idx = (path_index + 1) % path_len;
            let next_idx2 = (path_index + 2) % path_len;
            let mut current_point = [0 as f64; 3];

            let pp1 = path.index_at(path_index);
            let pp2 = path.index_at(next_idx);
            let pp3 = path.index_at(next_idx2);

            current_point[0] = (pp1.row() + pp2.row()) as f64 / 2.0;
            current_point[1] = (pp1.col() + pp2.col()) as f64 / 2.0;

            next_point[0] = (pp2.row() + pp3.row()) as f64 / 2.0;
            next_point[1] = (pp2.col() + pp3.col()) as f64 / 2.0;

            // line segment direction to the next point
            if current_point[0] < next_point[0] {
                if current_point[1] < next_point[1] {
                    current_point[2] = 1.0;
                }
                // SouthEast
                else if current_point[1] > next_point[1] {
                    current_point[2] = 7.0;
                }
                // NE
                else {
                    current_point[2] = 0.0;
                } // E
            } else if current_point[0] > next_point[0] {
                if current_point[1] < next_point[1] {
                    current_point[2] = 3.0;
                }
                // SW
                else if current_point[1] > next_point[1] {
                    current_point[2] = 5.0;
                }
                // NW
                else {
                    current_point[2] = 4.0;
                } // W
            } else {
                if current_point[1] < next_point[1] {
                    current_point[2] = 2.0;
                }
                // S
                else if current_point[1] > next_point[1] {
                    current_point[2] = 6.0;
                }
                // N
                else {
                    current_point[2] = 8.0;
                } // center, this should not happen
            }

            nodes[path_index] = InterpolationNode::new(current_point);
        }

        Self { inter_nodes: nodes }
    }
}

#[derive(Debug, Default, Clone)]
pub struct BatchInterpolation {
    batch_inter_nodes: Vec<InterpolationNodeList>,
}

impl BatchInterpolation {
    pub fn new(batch_inter_nodes: Vec<InterpolationNodeList>) -> Self {
        Self { batch_inter_nodes }
    }
}

#[derive(Debug, Default, Clone)]
pub struct InterpolationBatchList {
    batch_list: Vec<BatchInterpolation>,
}

impl InterpolationBatchList {
    pub fn new(batch_list: Vec<BatchInterpolation>) -> Self {
        Self { batch_list }
    }
}
