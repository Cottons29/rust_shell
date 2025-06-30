use crate::DEBUG_MODE;
use crate::DebugPrint;
use crate::dlog;
use std::path::Path;
use std::process::{Command, Stdio};
use std::{env, fs};
use getset::{Getters, Setters};

#[derive(Debug, Getters, Setters)]
pub struct ExecutableCmds {
    #[getset(get = "pub")]
    executable_path: String,
    #[getset(get = "pub")]
    executable: String,
}

impl ExecutableCmds {
    pub fn new(cmd: &str) -> Result<Self, Box<dyn std::error::Error>> {
        match Self::find_executable(&cmd) {
            Some(path) => {
                dlog!("found executable: {}", path);
                match Self::is_executable(&Path::new(&path)) {
                    true => Ok(Self { executable_path: path, executable: cmd.into() }),

                    false => Err("command not found".into()),
                }
            }
            None => Err("command not found".into()),
        }
    }

    pub fn is_executable(path: &Path) -> bool {
        fs::metadata(path)
            .map(|metadata| metadata.is_file())
            .unwrap_or(false)
    }

    fn find_executable(cmd: &str) -> Option<String> {
        if let Ok(path) = env::var("PATH") {
            for dir in path.split(":") {
                let path = Path::new(dir).join(&cmd);
                if Self::is_executable(&path) {
                    return Some(path.to_string_lossy().into_owned());
                }
            }
        }
        None
    }

    pub fn execute_cmd(&self, text: &String) -> Result<(), Box<dyn std::error::Error>> {
        let parts: Vec<&str> = text.split_whitespace().collect();
        let cmd = parts[0].trim();
        let args = &parts[1..];

        let _child = Command::new(cmd)
            .args(args)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?
            .wait()?;

        Ok(())
    }
}
