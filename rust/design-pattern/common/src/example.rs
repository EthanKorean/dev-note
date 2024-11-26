
pub trait ExampleExecutor{
    fn execute(&self);
}//ExampleExecutor
impl<F> ExampleExecutor for F
where
    F: Fn() + 'static, //
{
    fn execute(&self) {
        self(); //
    }
}//ExampleExecutor

pub struct Example {
    name : String,
    executor: Box<dyn ExampleExecutor>,
}//Example

impl Example{
    pub fn new(name: &str, executor: Box<dyn ExampleExecutor>) -> Self {
        Self { name : name.to_string(),  executor }
    }//new

    pub fn execute(self){
        println!("\x1b[32m## [{}] START ==========================================\n\x1b[0m", self.name);
        self.executor.execute();
        println!("\x1b[31m\n## END\x1b[0m");
    }//execute
}//Example

