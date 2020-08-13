use crate::{BatchInterpolation, InterpolationNodeList};

pub struct ImagePathTrace {
    trace_paths: Vec<Vec<f64>>,
}

impl ImagePathTrace {
    pub fn new(path: &InterpolationNodeList, l_threshold: f32, q_threshold: f32) -> Self {
        let mut path_index = 0;
        let mut sequence_end = 0;
        let mut sequence_type1: f64 = 0.;
        let mut sequence_type2: f64 = 0.;

        let mut smp = Vec::<Vec<f64>>::new();
        //Double [] thissegment;
        let path_length = path.len();

        while path_index < path_length {
            // 5.1. Find sequences of points with only 2 segment types
            sequence_type1 = path.index_at(path_index).point_at(2);
            sequence_type2 = -1.;
            sequence_end = path_index + 1;
            while ((path.index_at(sequence_end).point_at(2) == sequence_type1)
                || (path.index_at(sequence_end).point_at(2) == sequence_type2)
                || (sequence_type2 == -1.))
                && (sequence_end < (path_length - 1))
            {
                if (path.index_at(sequence_end).point_at(2) != sequence_type1)
                    && (sequence_type2 == -1.)
                {
                    sequence_type2 = path.index_at(sequence_end).point_at(2);
                }
                sequence_end = sequence_end + 1;
            }
            if sequence_end == (path_length - 1) {
                sequence_end = 0;
            }

            // 5.2. - 5.6. Split sequence and recursively apply 5.2. - 5.6. to startPoint-splitPoint and splitPoint-endPoint sequences
            let mut path_sequence =
                fit_sequence(path, l_threshold, q_threshold, path_index, sequence_end);
            smp.append(&mut path_sequence);
            // 5.7. TODO? If splitPoint-endPoint is a spline, try to add new points from the next sequence

            // forward pathIndex;
            if sequence_end > 0 {
                path_index = sequence_end;
            } else {
                path_index = path_length;
            }
        }
        Self { trace_paths: smp }
    }
}

fn fit_sequence(
    path: &InterpolationNodeList,
    l_threshold: f32,
    q_threshold: f32,
    seq_start: usize,
    seq_end: usize,
) -> Vec<Vec<f64>> {
    let mut segment = Vec::<Vec<f64>>::new();
    let mut thisSegment = Vec::<f64>::new();
    let path_length = path.len();

    // return if invalid seqEnd
    if (seq_end > path_length) || (seq_end < 0) {
        return segment;
    }

    let mut error_point = seq_start;
    let mut curve_pass = true;
    let mut px = 0.;
    let mut py = 0.;
    let mut error_val = 0.;
    let mut tl = (seq_end - seq_start) as f64;
    if tl < 0. {
        tl += path_length as f64;
    }
    let vx = (path.point_at_seq_idx(seq_end, 0) - path.point_at_seq_idx(seq_start, 0)) / tl;
    let vy = (path.point_at_seq_idx(seq_end, 1) - path.point_at_seq_idx(seq_start, 1)) / tl;

    // 5.2. Fit a straight line on the sequence
    let mut path_index = (seq_start + 1) % path_length;
    while path_index != seq_end {
        let mut pl = (path_index - seq_start) as f64;
        if pl < 0. {
            pl += path_length as f64;
        }
        px = path.point_at_seq_idx(seq_start, 0) + (vx * pl as f64);
        py = path.point_at_seq_idx(seq_start, 1) + (vy * pl as f64);
        let dist2 = ((path.point_at_seq_idx(path_index, 0) - px)
            * (path.point_at_seq_idx(path_index, 0) - px))
            + ((path.point_at_seq_idx(path_index, 1) - py)
                * (path.point_at_seq_idx(path_index, 1) - py));
        if dist2 > l_threshold as f64 {
            curve_pass = false;
        }
        if dist2 > error_val {
            error_point = path_index;
            error_val = dist2;
        }
        path_index = (path_index + 1) % path_length;
    }

    // return straight line if fits
    if curve_pass {
        let mut current_segment = vec![0 as f64; 7];
        current_segment[0] = 1.0;
        current_segment[1] = path.point_at_seq_idx(seq_start, 0);
        current_segment[2] = path.point_at_seq_idx(seq_start, 1);
        current_segment[3] = path.point_at_seq_idx(seq_end, 0);
        current_segment[4] = path.point_at_seq_idx(seq_end, 1);
        current_segment[5] = 0.0;
        current_segment[6] = 0.0;
        segment.push(current_segment);
        return segment;
    }

    // 5.3. If the straight line fails (an error>lThreshold), find the point with the biggest error
    let fit_point = error_point;
    curve_pass = true;
    error_val = 0.;

    // 5.4. Fit a quadratic spline through this point, measure errors on every point in the sequence
    // helpers and projecting to get control point
    let mut t = (fit_point - seq_start) as f64 / tl as f64;
    let mut t1 = (1.0 - t) * (1.0 - t);
    let mut t2 = 2. * (1. - t) * t;
    let mut t3 = t * t;
    let cpx = (((t1 * path.point_at_seq_idx(seq_start, 0))
        + (t3 * path.point_at_seq_idx(seq_end, 0)))
        - path.point_at_seq_idx(fit_point, 0))
        / -t2;
    let cpy = (((t1 * path.point_at_seq_idx(seq_start, 1))
        + (t3 * path.point_at_seq_idx(seq_end, 1)))
        - path.point_at_seq_idx(fit_point, 1))
        / -t2;

    // Check every point
    path_index = seq_start + 1;
    while path_index != seq_end {
        t = (path_index - seq_start) as f64 / tl;
        t1 = (1.0 - t) * (1.0 - t);
        t2 = 2.0 * (1.0 - t) * t;
        t3 = t * t;
        px = (t1 * path.point_at_seq_idx(seq_start, 0))
            + (t2 * cpx)
            + (t3 * path.point_at_seq_idx(seq_end, 0));
        py = (t1 * path.point_at_seq_idx(seq_start, 1))
            + (t2 * cpy)
            + (t3 * path.point_at_seq_idx(seq_end, 1));

        let dist2 = ((path.point_at_seq_idx(path_index, 0) - px)
            * (path.point_at_seq_idx(path_index, 0) - px))
            + ((path.point_at_seq_idx(path_index, 1) - py)
                * (path.point_at_seq_idx(path_index, 1) - py));

        if dist2 > q_threshold as f64 {
            curve_pass = false;
        }
        if dist2 > error_val {
            error_point = path_index;
            error_val = dist2;
        }
        path_index = (path_index + 1) % path_length;
    }

    // return spline if fits
    if curve_pass {
        let mut current_segment = vec![0 as f64; 7];
        current_segment[0] = 2.0;
        current_segment[1] = path.point_at_seq_idx(seq_start, 0);
        current_segment[2] = path.point_at_seq_idx(seq_start, 1);
        current_segment[3] = cpx;
        current_segment[4] = cpy;
        current_segment[5] = path.point_at_seq_idx(seq_end, 0);
        current_segment[6] = path.point_at_seq_idx(seq_end, 1);
        segment.push(current_segment);
        return segment;
    }

    // 5.5. If the spline fails (an error>qThreshold), find the point with the biggest error,
    // set splitPoint = (fitting point + errorPoint)/2
    let splitPoint = (fit_point + error_point) / 2;

    // 5.6. Split sequence and recursively apply 5.2. - 5.6. to startPoint-splitPoint and splitPoint-endpoint sequences
    let mut segment_created = fit_sequence(path, l_threshold, q_threshold, seq_start, splitPoint);
    segment = segment_created;

    let mut segment_another = fit_sequence(path, l_threshold, q_threshold, splitPoint, seq_end);
    segment.append(&mut segment_another);
    return segment;
}

pub struct ImagePathTraceList {
    trace_paths: Vec<ImagePathTrace>,
}

impl ImagePathTraceList {
    pub fn new(batch: &BatchInterpolation, l_threshold: f32, q_threshold: f32) -> Self {
        let mut trace_paths = Vec::<ImagePathTrace>::new();

        for idx in 0..batch.len() {
            let trace_path = ImagePathTrace::new(batch.index_at(idx), l_threshold, q_threshold);
            trace_paths.push(trace_path)
        }
        Self { trace_paths }
    }
}

pub struct ImagePathTraceLayers {
    trace_path_layers: Vec<ImagePathTraceList>,
}

impl ImagePathTraceLayers {
    pub fn new(layers: Vec<ImagePathTraceList>) -> Self {
        Self {
            trace_path_layers: layers,
        }
    }
}
