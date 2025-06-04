use crate::commands::build_in::Commands;
use crate::commands::cd::CdCommand;
use crate::commands::echo::EchoCommand;
use crate::commands::ls::LsCommand;
use crate::commands::mkdir::MkdirCmd;
use crate::commands::simple::ClearCommand;
use crate::utils::{DebugTool, ResultPrinter, WordSplitter};
use std::path::PathBuf;

pub struct CmdParser {
    cmd: Commands,
    args: Vec<String>,
    current_dir: PathBuf,
}

impl Default for CmdParser {
    fn default() -> Self {
        Self {
            cmd: Commands::EmptyCommand,
            args: vec![],
            current_dir: PathBuf::from("/Users/cottons/Desktop"),
        }
    }
}

impl CmdParser {
    pub fn new(
        text_line: String,
        current_dir: Option<PathBuf>,
    ) -> Result<CmdParser, Box<dyn std::error::Error>> {
        use crate::commands::build_in::Commands::*;
        let current_dir = current_dir.unwrap_or_else(|| PathBuf::from("/Users/cottons/Desktop"));
        if text_line.is_empty() {
            return Ok(CmdParser {
                cmd: EmptyCommand,
                args: vec![],
                current_dir,
            });
        }
        let parts = text_line.advance_split();
        DebugTool::print(format!("parts: {:?}", parts));
        let cmd = parts[0].to_string();
        let args = parts[1..].to_vec();

        let args = if args.is_empty() { vec![] } else { args };

        let cmd = Commands::new(&cmd)?;
        Ok(CmdParser {
            cmd,
            args,
            current_dir,
        })
    }

    pub fn get_current_dir(&self) -> &PathBuf {
        &self.current_dir
    }

    pub fn execute_cmd(mut self) -> Result<Self, Box<dyn std::error::Error>> {
        use crate::commands::build_in::Commands::*;

        match &self.cmd {
            Type(_cmd) => {
                let arg = self.args[0].to_string();
                let result = match self.cmd.type_cmd(&arg) {
                    Ok(res) => res,
                    Err(err) => {
                        ResultPrinter::error(format!("{}", err.to_string()));
                        return Ok(self);
                    }
                };
                println!("{result}");
            }

            Echo(_) => match EchoCommand::new(self.args.clone(), &self.current_dir).run() {
                Ok(_) => {}
                Err(err) => {
                    ResultPrinter::error(format!("echo: {}", err.to_string()));
                }
            },

            Clear(_) => match ClearCommand::run() {
                Ok(_) => {}
                Err(err) => {
                    ResultPrinter::error(format!("{}", err.to_string()));
                }
            },

            Ls(_) => match LsCommand::new(&self.current_dir, &self.args) {
                Ok(mut res) => match res.run() {
                    Ok(_) => {}
                    Err(err) => {
                        ResultPrinter::error(format!("{}", err.to_string()));
                    }
                },
                Err(err) => {
                    ResultPrinter::error(format!("{}", err.to_string()));
                }
            },
            Cd(_) => {
                self.current_dir = match CdCommand::new(&self.current_dir, &self.args) {
                    Ok(res) => match res.run() {
                        Ok(res) => res,
                        Err(err) => {
                            ResultPrinter::error(format!("{}", err.to_string()));
                            return Ok(self);
                        }
                    },
                    Err(err) => {
                        ResultPrinter::error(format!("{}", err.0.to_string()));
                        err.1
                    }
                }
            }
            Pwd(_) => println!("{}", self.current_dir.display()),
            Mkdir(_) => match MkdirCmd::new(&self.args, &self.current_dir) {
                Ok(res) => match res.run() {
                    Ok(_) => {}
                    Err(err) => {
                        ResultPrinter::error(format!("{}", err.to_string()));
                    }
                },
                Err(_err) => {}
            },

            _ => ResultPrinter::error(format!("cotsh: command not found: {}", self.cmd.get_cmd())),
        }
        Ok(self)
    }
}
