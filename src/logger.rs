use lightning::util::logger::{Logger, Record};

pub struct MyLogger;
impl MyLogger {
    pub fn new() -> Self {
        MyLogger
    }
}

impl Logger for MyLogger {
    fn log(&self, record: Record) {
        println!("{}", record.args);
    }
}
