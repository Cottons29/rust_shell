use std::path::PathBuf;
use crate::commands::cd::CdCommand;
use crate::utils::{DebugTool, WordSplitter};

pub struct MkdirCmd {
    name: String,
    current_dir: PathBuf,
}

impl MkdirCmd {
    pub fn new(name: &Vec<String>, current_dir: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let name = match name.first(){
            Some(name) => name.trim(),
            None => return Err(From::from("No name provided"))
        };
        let current_dir = current_dir.to_path_buf();
        Ok(Self {
            name: String::from(name),
            current_dir
        })
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut splitter = self.name.split_path()?;
        let dir_name = splitter.last().unwrap().to_string();
        splitter.pop();
        let full_path = splitter.join("/");
        let temp = match CdCommand::new(&self.current_dir, &vec![full_path]){
            Ok(temp) =>{
                match temp.run(){
                    Ok(res) => res,
                    Err(_) => return Err("mkdir : No such file or directory".into())
                }
            },
            Err(_) => return Ok(())
        };

        let path = temp.join(&dir_name);
        DebugTool::print(&format!("mkdir : Creating directory {}", path.display()));
        match std::fs::create_dir(path){
            Ok(_) => Ok(()),
            Err(_) => return Err(format!("mkdir: {}: File exists", dir_name).into())       
        }
    }
}