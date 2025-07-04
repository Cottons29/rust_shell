use std::path::PathBuf;
use crate::commands::CmdParser;
use crate::{print_error, print_success};

pub struct Interpreter{
    script_lines: Vec<String>,
}

impl Interpreter{
    pub fn new(script_file: PathBuf) -> Result<Self, Box<dyn std::error::Error>>{
        println!("{:?}", script_file);
        match script_file.exists(){
            true => {},
            false => return Err("File not found".into())
        };

        let script_lines = std::fs::read_to_string(&script_file)?
            .split('\n')
            .map(|line| line.trim().to_string())
            .collect::<Vec<String>>();
        Ok(Self{script_lines})
    }

    pub fn new_with_lines(script_lines: &String) -> Self{
        let script_lines = script_lines.split(';')
            .map(|line| line.trim().to_string())
            .collect::<Vec<String>>();
        Self{script_lines}
    }

    pub fn interpret(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd_parser = CmdParser::default();

        let mut counter: u32 = 1;
        for line in &self.script_lines{
            cmd_parser = match  CmdParser::new(line, Some(cmd_parser.get_current_dir().into())){
                Ok(cmd_parser) => {
                    match cmd_parser.execute_cmd(true){
                        Ok(res) => res,
                        Err(err) => {
                            print_success!("Error on line {}: {}", counter, err);
                            return Err(err.into());
                        }   
                    }
                }
                Err(err) => {
                    print_error!("Error on line {}: {}", counter, err);
                    return Err(err.into());
                }
            };
            counter += 1;
        }
        Ok(())
    }
}