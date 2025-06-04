use std::env::Args;
use std::fs::{ReadDir};
use std::path::PathBuf;
use crate::DEBUG_MODE;
use crate::utils::DebugTool;

pub struct LsCommand {
    format: Vec<LsFormat>,
    dirs: Vec<String>,
    current_dir: PathBuf,
    raw_args : Vec<String>,
}

enum LsFormat {
    LongListing,
    OnePerLine,
    CSVFormat,
    Default,
    OneLine,
    InnerRecursive
}

impl LsFormat {
    pub fn new(arg: String) -> Self {
        match arg.as_str() {
            "-l" => Self::LongListing,
            "-1" => Self::OnePerLine,
            "-F" => Self::CSVFormat,
            "-R" => Self::InnerRecursive,
            "-x" => Self::OneLine,
            _ => Self::Default,
        }
    }
}

impl LsCommand {
    pub fn new(dir: &PathBuf, args: &Vec<String>) -> Result<Self, Box<dyn std::error::Error>> {
        DebugTool::print(format!("dir = {:?}, args = {:?}", dir, args));
        let args = args.clone();
        let temp = Self {
            dirs: Vec::new(),
            format: Vec::new(),
            current_dir: dir.clone(),
            raw_args: args,
        }.parse_args()?;
        Ok(temp)
    }

    pub fn get_dirs(&self) -> &Vec<String> {
        &self.dirs
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.dirs.len() == 0 {
            self.list_dirs_and_files(None)?
        } else {
            self.list_dirs_and_files(Some(&PathBuf::from(self.dirs.first().unwrap())))?
        }
        Ok(())
    }

    fn parse_args(mut self) -> Result<Self, Box<dyn std::error::Error>> {
        for raw_arg in &self.raw_args{
            if raw_arg.starts_with('-') {
                let arg = raw_arg.chars().skip(1).collect::<String>();
                self.format.push(LsFormat::new(arg));
            }else{
                self.dirs.push(raw_arg.to_string());
            }
        }
        Ok(self)
    }


    pub fn is_valid_path(&self, target_file: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let entry = match std::fs::read_dir(&self.current_dir) {
            Ok(dirs) => dirs,
            Err(e) => return Err(e.into()),
        };
        for dir_result in entry {
            if let Ok(dir_entry) = dir_result {
                let file_name = dir_entry.file_name().to_string_lossy().to_string();
                if file_name == target_file {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }

    fn formatted_display(
        &self,
        entry: ReadDir,
        column_count: Option<u32>,
        target_dir: Option<&PathBuf>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (row_len, column_len) = crossterm::terminal::size().unwrap();
        // println!("row = {} | column = {}", row_len, column_len);
        let longest_dir_name = self.find_longest_item_name(target_dir)? + 7;
        // println!("longest_dir_name = {}", longest_dir_name);
        let max_column = row_len / longest_dir_name;
        // println!("max_column = {}", max_column);
        let mut counter = 0;
        let mut line = String::new();
        if column_count.is_some() {
            let max_column = 1;
        }
        if target_dir.is_some() {
            println!("{}:", target_dir.unwrap().to_string_lossy());
        }
        for dir in entry {

            let dir = dir?.file_name().to_string_lossy().to_string();
            if dir.starts_with('.') {
                continue;
            }
            line.push_str(&self.row_item_builder(&dir, longest_dir_name, counter == max_column - 1));
            counter += 1;
            if counter == max_column  {
                println!("{}", line);
                line.clear();
                counter = 0;
            }
        }
        if line.len() > 0 {
            println!("{}", line);
        }
        Ok(())
    }

    fn row_item_builder(&self, dir_r_file: &str, column_len: u16, is_last: bool) -> String {
        if is_last {
            // DebugTool::print(format!("last_item = {}", dir_r_file));
            return dir_r_file.into();
        }
        let mut dir_r_file = dir_r_file.to_string();
        // println!("column_len = {} | dir_file_len = {}", column_len, dir_r_file.len());
        let diff_len = column_len - dir_r_file.len() as u16;
        for _ in 0..diff_len {
            dir_r_file.push(' ');
        }
        dir_r_file
    }

    fn find_longest_item_name(&self, target_dir: Option<&PathBuf>) -> Result<u16, Box<dyn std::error::Error>> {
        let dir_path = if target_dir.is_some() {
            &self.current_dir.join(target_dir.unwrap())
        }else {
            &self.current_dir
        };
        let entry = match std::fs::read_dir(&dir_path) {
            Ok(dirs) => dirs,
            Err(e) => return Err("Error reading directory".into()),
        };
        let mut longest_dir_name = 0;
        // let mut longest_file = String::new();
        for dir_result in entry {
            if let Ok(dir_entry) = dir_result {
                let file_name = dir_entry.file_name().to_string_lossy().to_string();
                let name_len = file_name.len();
                if name_len > longest_dir_name {
                    longest_dir_name = name_len;
                    // longest_file = file_name;
                }
            }
        }
        // DebugTool::print(format!("longest_file = {} : {longest_dir_name}", longest_file));
        Ok(longest_dir_name as u16)
    }

    fn load_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn list_dirs_and_files(
        &self,
        target_dir: Option<&PathBuf>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let current_dir = if target_dir.is_some() {
            &self.current_dir.join(target_dir.unwrap())
        } else {
            &self.current_dir
        };

        let dirs = match std::fs::read_dir(current_dir) {
            Ok(dirs) => dirs,
            Err(e) => return Err(e.into()),
        };
        DebugTool::print(format!("args len = {}", self.format.len()));
        match self.dirs.len() {
            0 => self.formatted_display(dirs, None, None)?,
            1 => self.formatted_display(dirs, None, target_dir)?,
            _ => return Err("Too many arguments".into()),
        }
        Ok(())
    }
}
