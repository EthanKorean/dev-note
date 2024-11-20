
pub trait ExampleExecutor{
    fn execute(&self);
}
impl<F> ExampleExecutor for F
where
    F: Fn() + 'static, //
{
    fn execute(&self) {
        self(); //
    }
}

pub struct Example<'a> {
    name :  & 'a str,
    executor: Box<dyn ExampleExecutor>,
}

impl Example{
    pub fn new(name: &str, executor: Box<dyn ExampleExecutor>) -> Self {
        Self { name, executor }
    }

    pub fn execute(self){
        println!("## [{}] START ==========================================\n", self.name);
        self.executor.execute();
        println!("\n## END ");
    }
}

