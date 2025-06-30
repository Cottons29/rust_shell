use crate::commands::cd::CdCommand;
use crate::commands::echo::arg_type::EchoArg;
use crate::utils::{WordSplitter};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

pub struct EchoCommand {
    args: Vec<EchoArg>,
    raw_text: Option<String>,
    output_dir: PathBuf,
    option: Option<EchoArg>,
    current_dir: PathBuf,
    is_redirect: bool,
    redir_option: Option<EchoArg>,
}

impl EchoCommand {
    pub fn new(args: Vec<String>, current_dir: &PathBuf) -> Self {
        let re_dir = current_dir.clone();
        let args = args.join(" ");
        let args = args.advance_split();
        let args: Vec<EchoArg> = args.iter().map(|arg| EchoArg::new(arg)).collect();
        let mut option: Option<EchoArg> = None;
        for arg in args.iter() {
            match arg {
                EchoArg::Flag(_) => {
                    option = Some(arg.clone());
                    break;
                }
                _ => continue,
            }
        }
        Self {
            args,
            raw_text: None,
            output_dir: re_dir,
            option,
            current_dir: current_dir.clone(),
            is_redirect: false,
            redir_option: None,
        }
    }

    fn combine_text(&self) -> String {
        let temp = self
            .args
            .iter()
            .map(|arg| match arg { EchoArg::Flag(_) => None, _ => Some(arg.value())})
            .collect::<Vec<Option<String>>>();

        let temp = temp
            .iter()
            .filter(|arg| arg.is_some())
            .map(|arg| arg.clone().unwrap())
            .collect::<Vec<String>>();
        
        temp.join(" ")
    }

    fn init_work(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let pre_process = format!("{}",self.combine_text());
        self.raw_text = Some(pre_process);
        if self.is_contain_redirection() {
            self.is_redirect = true;
            let temp = self.find_output_dir()?;
            self.output_dir = temp.0;
            self.remove_raw_path(&temp.1);
        }

        Ok(())
    }

    fn remove_raw_path(&mut self, raw_path: &str) {
        for index in 0..self.args.len() {
            let _temp = match self.args.get(index) {
                Some(arg) => arg,
                None => continue,
            };
            if self.args.get(index).unwrap().value() == raw_path {
                self.args.remove(index);
                break;
            }
        }
    }

    fn is_contain_redirection(&self) -> bool {
        for index in 0..self.args.len() {
            let temp = match self.args.get(index) {
                Some(arg) => arg,
                None => continue,
            };
            match temp {
                EchoArg::WriteInto(_) | EchoArg::AppendInto(_) => {
                    return true;
                }
                _ => {}
            }
        }
        false
    }

    fn remove_option(&mut self) {
        for index in 0..self.args.len() {
            let temp = match self.args.get(index) {
                Some(arg) => arg,
                None => continue,
            };
            match temp {
                EchoArg::Flag(_) => {
                    self.args.remove(index);
                }
                _ => {}
            }
        }
    }

    fn find_output_dir(&mut self) -> Result<(PathBuf, String), Box<dyn std::error::Error>> {
        let mut temp_res = String::new();
        let mut raw_path = String::new();
        // Find the redirection operator and get the following argument
        for index in 0..self.args.len() {
            let temp = match self.args.get(index) {
                Some(arg) => arg,
                None => continue,
            };
            match temp {
                EchoArg::WriteInto(_) | EchoArg::AppendInto(_) => {
                    self.redir_option = Some(temp.clone());
                    match self.args.get(index + 1) {
                        Some(arg) => {
                            raw_path = arg.value();
                            temp_res = arg.value();
                            break;
                        }
                        None => {
                            return Err(
                                "No output file specified after redirection operator".into()
                            );
                        }
                    }
                }
                _ => {}
            }
        }

        // Check if we found any redirection
        if temp_res.is_empty() {
            return Err("No redirection operator found".into());
        }

        let mut parts = match temp_res.split_path() {
            Ok(res) => res,
            Err(err) => return Err(err),
        };

        // Handle empty parts
        if parts.is_empty() {
            return Err("Invalid empty path".into());
        }

        // Extract filename and directory path
        let file_name = if parts.len() > 1 {
            let temp = parts.last().unwrap().to_string();
            parts.pop(); // Use pop() instead of remove(parts.len() - 1)
            temp
        } else {
            // Only one component - treat as filename in current directory
            return Ok((self.current_dir.join(&parts[0]), raw_path));
        };

        // If there are directory components, resolve them
        let mut temp_cd = if !parts.is_empty() {
            let full_path = parts.join("/");
            

            match CdCommand::new(&self.current_dir, &vec![full_path]) {
                Ok(cd) => match cd.run() {
                    Ok(res) => {
                        res
                    }
                    Err(err) => return Err(err),
                },
                Err((err, _)) => return Err(err),
            }
        } else {
            self.current_dir.clone()
        };
        temp_cd.push(file_name);

        
        Ok((temp_cd, raw_path))
    }

    pub fn redirect_into(&self) -> Result<(), Box<dyn std::error::Error>> {
        let is_append = match self.redir_option {
            Some(EchoArg::AppendInto(_)) => true,
            _ => false,
        };
        let mut file_option = match fs::OpenOptions::new()
            .write(true)
            .append(is_append)
            .truncate(!is_append)
            .create(true)
            .open(&self.output_dir)
        {
            Ok(res) => res,
            Err(err) => return Err(err.into()),
        };

        let temp = self.extract_text();
        for temp in temp.iter() {
            file_option.write_all(format!("{}\n", temp).as_bytes())?;
        }

        Ok(())
    }

    fn extract_text(&self) -> Vec<String> {
        let mut temp_vec: Vec<String> = Vec::new();
        for arg in self.args.iter() {
            match arg {
                EchoArg::Plain(text) => temp_vec.push(text.into()),
                EchoArg::DoubleQuoteTxt(text) => temp_vec.push(text.split_double_quote().join(" ")),
                EchoArg::QuoteTxt(text) => temp_vec.push(text.split_quote().join(" ")),
                _ => {}
            }
        }
        temp_vec
    }

    fn backslash_parser(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("{}", self.raw_text.as_ref().unwrap());
        Ok(())
    }

    fn inline_echo(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn parse_flags(&self, flag: &str) -> Result<(), Box<dyn std::error::Error>> {
        match flag {
            "-e" => self.backslash_parser(),
            "-n" => self.inline_echo(),
            _ => Err(String::new().into()),
        }
    }

    fn process_escapes(&self, text: &str) -> String {
        text.replace("\\", "\\\\") // Escape literal backslash first
            .replace("\n", "\\n") // Now safely escape newline
            .replace("\t", "\\t") // Tab
            .replace("\r", "\\r") // Carriage return
            .replace("\0", "\\0") // Null byte
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.init_work()?;
        self.remove_option();

        if self.is_redirect {
            self.redirect_into()?;
            return Ok(());
        }

        if self.option.is_some() {
            let temp = self.option.as_ref().unwrap().value();
            self.parse_flags(&temp)?;
            return Ok(());
        }

        match self.args.len() {
            0 => {
                println!();
                Ok(())
            }
            _ => {
                let temp = self.process_escapes(&self.raw_text.as_ref().unwrap());
                println!("{temp}");
                Ok(())
            }
        }
    }
}
