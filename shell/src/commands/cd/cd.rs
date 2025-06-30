use std::path::PathBuf;
use crate::commands::ls::LsCommand;
use crate::utils::WordSplitter;

pub struct CdCommand{
    current_dir: PathBuf,
    arg: String,
    split_path: Vec<String>,
}

impl CdCommand{
    pub fn new(current_dir: &PathBuf, arg: &Vec<String>) -> Result<Self, (Box<dyn std::error::Error>, PathBuf)>{
        if arg.len() > 1{
            return Err(("cd: too many arguments".into(), current_dir.into()));
        }
        let arg: String = if arg.len() != 0{
            arg.first().unwrap().into()
        }else{
            String::from("")      
        };
        let split_path: Vec<String> = match arg.split_path() {
            Ok(split_path) => split_path,
            Err(_) => return Err(("cd: invalid path".into(), current_dir.into())),
        };
        let current_dir = if arg == "/"{
            PathBuf::from("/")      
        }else{
            current_dir.clone()
        };
        Ok(Self{
            current_dir,
            arg,
            split_path
        })
    }

    pub fn run(mut self) -> Result<PathBuf, Box<dyn std::error::Error>>{
        if self.split_path.len() == 0 {
            return Ok(self.current_dir)
        };
        let first_path = match self.split_path.first(){
            Some(first_path) => first_path,
            None => return Ok(self.current_dir),
        };
        if first_path.is_empty(){
            if self.arg == String::from(""){
                self.current_dir = self.go_upper_dir();
            }
            return Ok(self.current_dir)       
        }
        self.current_dir = match first_path.as_str(){
            "~" => PathBuf::from("/"),
            ".." => self.go_upper_dir(),
            "." => self.current_dir,
            _ => {
                match self.is_valid_dir(first_path){
                    Ok(true) => {
                        self.current_dir.push(first_path.clone());
                        self.current_dir
                    },
                    Ok(false) | Err(_) => return Err(format!("cd: no such file or directory: {}", self.arg).into()),
                }
            },
        };
        self.split_path.remove(0);
        self.run()
    }

    fn go_upper_dir(&self) -> PathBuf{
        let mut current_dir = self.current_dir.clone();
        if current_dir.as_os_str().to_str().unwrap() != "/" {
            let _ = current_dir.pop().to_owned();
        }
        current_dir
    }


    fn is_valid_dir(&self, arg: &String) -> Result<bool, Box<dyn std::error::Error>>{
        LsCommand::new(&self.current_dir, &vec![])?.is_valid_path(arg)
    }
}