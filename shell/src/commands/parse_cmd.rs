use crate::commands::commands::Commands;
use crate::commands::cd::CdCommand;
use crate::commands::echo::EchoCommand;
use crate::commands::ls::LsCommand;
use crate::commands::mkdir::MkdirCmd;
use crate::commands::simple::ClearCommand;
use crate::utils::{WordSplitter};
use std::path::PathBuf;
use crate::{print_error, print_success};
use crate::commands::executable_cmds::ExecutableCmds;
use crate::interpreter::Interpreter;

pub struct CmdParser {
    cmd: Commands,
    args: Vec<String>,
    current_dir: PathBuf,
    text_line: String,
}

impl Default for CmdParser {
    fn default() -> Self {
        Self {
            cmd: Commands::EmptyCommand,
            args: vec![],
            current_dir: PathBuf::from("/Users/cottons/Desktop"),
            text_line: String::new(),
        }
    }
}

impl CmdParser {
    pub fn new(
        text_line: &String,
        current_dir: Option<PathBuf>,
    ) -> Result<CmdParser, Box<dyn std::error::Error>> {
        use crate::commands::commands::Commands::*;
        let current_dir = current_dir.unwrap_or_else(|| PathBuf::from("/Users/cottons/Desktop"));
        if text_line.is_empty() {
            return Ok(CmdParser {
                cmd: EmptyCommand,
                args: vec![],
                current_dir,
                text_line: String::new(),
            });
        }
        let parts = text_line.advance_split();

        let cmd = parts[0].to_string();
        let args = parts[1..].to_vec();

        let args = if args.is_empty() { vec![] } else { args };

        let cmd = Commands::new(&cmd)?;
        Ok(CmdParser {
            cmd,
            args,
            current_dir,
            text_line: text_line.clone(),
        })
    }

    pub fn get_current_dir(&self) -> &PathBuf {
        &self.current_dir
    }

    pub fn execute_cmd(mut self, interpret_mode : bool) -> Result<Self, Box<dyn std::error::Error>> {

        if self.text_line.contains(";") || self.text_line.contains("&&") || self.text_line.contains("||") {
            match Interpreter::new_with_lines(&self.text_line).interpret() {
                Ok(_) => {}
                Err(err) => {
                    print_error!("{}", err.to_string());
                }
            }
            return Ok(self);
        }

        use crate::commands::commands::Commands::*;

        match &self.cmd {
            Type(_cmd) | Which(_cmd) => {
                match self.cmd.type_cmd(&self.args[0]) {
                    Ok(res) => print_success!("{}", res),
                    Err(err) => print_error!("{}", err.to_string()),
                };
            }

            Echo(_) => match EchoCommand::new(self.args.clone(), &self.current_dir).run() {
                Ok(_) => {}
                Err(err) => print_error!("echo: {}", err.to_string())
            },

            Clear(_) => match ClearCommand::run() {
                Ok(_) => {}
                Err(err) => {
                    print_error!("{}", err.to_string());
                }
            },

            Ls(_) => match LsCommand::new(&self.current_dir, &self.args) {
                Ok(mut res) => match res.run() {
                    Ok(_) => {}
                    Err(err) => {
                        print_error!("{}", err.to_string());
                    }
                },
                Err(err) => {
                    print_success!("{}", err.to_string());
                }
            },
            Cd(_) => {
                self.current_dir = match CdCommand::new(&self.current_dir, &self.args) {
                    Ok(res) => match res.run() {
                        Ok(res) => res,
                        Err(err) => {
                            print_error!("{}", err.to_string());
                            return Ok(self);
                        }
                    },
                    Err(err) => {
                        print_error!("{}", err.0.to_string());
                        err.1
                    }
                }
            }
            Pwd(_) => println!("{}", self.current_dir.display()),
            Mkdir(_) => match MkdirCmd::new(&self.args, &self.current_dir) {
                Ok(res) => match res.run() {
                    Ok(_) => {}
                    Err(err) => {
                        print_success!("{}", err.to_string());
                    }
                },
                Err(_err) => {}
            },

            NotBuildIn(_) => {
                match ExecutableCmds::new(&self.cmd.get_cmd(), &self.args, &self.current_dir){
                    Ok(res) => match res.execute_cmd() {
                        Ok(_) => {}
                        Err(err) => {
                            print_error!("{}", err.to_string());
                        }
                    },
                    Err(err) => {
                        print_error!("{}", err.to_string());
                    }
                }
            }

            Cotsh(_) => {
                match Interpreter::new_with_lines(&self.text_line).interpret() {
                    Ok(_) => {}
                    Err(err) => {
                        print_error!("{}", err.to_string());
                    }
                }
            }



            _ => {
                print_error!("cotsh: command not found: {}", self.cmd.get_cmd());
            },
        }
        Ok(self)
    }
}
