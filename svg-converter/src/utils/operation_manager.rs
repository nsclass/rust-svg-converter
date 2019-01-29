
type Operation<T> = Box<Fn(T) -> Result<T, String>>;

struct OperationItem<T> {
    name: String,
    operation: Operation<T>,
}

impl<T> OperationItem<T> {
    fn new(name: &str, operation: Operation<T>) -> OperationItem<T> {
        OperationItem {
            name: String::from(name),
            operation: operation,
        }
    }
}

pub struct OperationManager<'a, T> {
    progress_listener: &'a OperationProgressListener,
    operation_list: Vec<OperationItem<T>>,
}

impl<'a, T> OperationManager<'a, T> {
    pub fn new(listener: &'a OperationProgressListener) -> OperationManager<'a, T> {
        OperationManager {
            progress_listener: listener,
            operation_list: Vec::with_capacity(5),
        }
    }

    pub fn add_opeation<F: 'static>(&mut self, desc: &str, f: F)
            where F: Fn(T) -> Result<T, String> {
        self.operation_list.push(OperationItem::new(desc, Box::new(f)));
    }

    pub fn execute(&self, mut ctx: T) -> Result<T, String> {
        let size = self.operation_list.len();
        for (idx, item) in self.operation_list.iter().enumerate() {
            ctx = (*item.operation)(ctx)?;
            self.progress_listener.on_progress(&item.name, idx, size);
        }

        Result::Ok(ctx)
    }
}
