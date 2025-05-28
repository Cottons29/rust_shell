pub struct EchoCommand{
    pub args: Vec<String>
}


impl EchoCommand{
    pub fn new(args: Vec<String>) -> Self{
        Self{
            args
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>>{
        match self.args.len(){
            0 => {
                println!();
                Ok(())
            }
            _ => {
                println!("{}", self.args[0..self.args.len()].join(" "));
                Ok(())
            }
        }
    }
}