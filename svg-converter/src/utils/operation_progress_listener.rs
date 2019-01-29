
pub trait OperationProgressListener {
    fn on_progress(&self, desc: &str, cur: usize, total: usize);
}