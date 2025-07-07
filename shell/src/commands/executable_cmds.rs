use crate::DEBUG_MODE;
use crate::DebugPrint;
use crate::dlog;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::{env, fs};
use getset::{Getters, Setters};

#[derive(Debug, Getters, Setters)]
pub struct ExecutableCmds {
    #[getset(get = "pub")]
    executable_path: String,
    #[getset(get = "pub")]
    executable: String,
    #[getset(get = "pub")]
    args: Vec<String>,
    #[getset(get = "pub")]
    current_path: PathBuf,
}

impl ExecutableCmds {
    pub fn new(cmd: &str, args: &Vec<String>, current_path: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        match Self::find_executable(&cmd) {
            Some(path) => {
                dlog!("found executable: {}", path);
                match Self::is_executable(&Path::new(&path)) {
                    true => Ok(Self {
                        executable_path: path,
                        executable: cmd.into(),
                        args: args.clone(),
                        current_path: current_path.clone()
                    }),

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

    pub fn execute_cmd(&self) -> Result<(), Box<dyn std::error::Error>> {
        let _child = Command::new(&self.executable)
            .current_dir(&self.current_path)
            .args(&self.args)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?
            .wait()?;

        Ok(())
    }
}
