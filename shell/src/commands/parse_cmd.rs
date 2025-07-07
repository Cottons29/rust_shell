use regex::Regex;
use crate::commands::commands::Commands;
use crate::commands::cd::CdCommand;
use crate::commands::echo::EchoCommand;
use crate::commands::ls::LsCommand;
use crate::commands::mkdir::MkdirCmd;
use crate::commands::simple::ClearCommand;
use crate::utils::{WordSplitter};
use crate::{print_error, print_success, CURRENT_DIR};
use crate::commands::executable_cmds::ExecutableCmds;
use crate::commands::ExitCommand;
use crate::interpreter::{eval, tokenize, Interpreter, Parser};



pub struct CmdParser {
    cmd: Commands,
    args: Vec<String>,
    script_line: String,
    is_expression: bool,
}

impl Default for CmdParser {
    fn default() -> Self {
        Self {
            is_expression: false,
            cmd: Commands::EmptyCommand,
            args: vec![],
            script_line: String::new(),
        }
    }
}

impl CmdParser {
    pub fn new(
        text_line: &String,
    ) -> Result<CmdParser, Box<dyn std::error::Error>> {
        use crate::commands::commands::Commands::*;
        if text_line.is_empty() {
            return Ok(CmdParser {
                is_expression: false,
                cmd: EmptyCommand,
                args: vec![],
                script_line: String::new(),
            });
        }
        let parts = text_line.advance_split();
        let cmd = parts[0].to_string();
        let args = parts[1..].to_vec().iter().filter(|x| !x.is_empty()).map(|x| x.to_string()).collect::<Vec<String>>();
        let cmd = Commands::new(&cmd)?;

        let regex = Regex::new(r"-?\d+\.\d+|-?\d+|[+\-*/()]").unwrap();

        if regex.is_match(&cmd.get_cmd()) {
            return Ok(CmdParser {
                is_expression: true,
                cmd,
                args,
                script_line: text_line.clone(),
            });
        }

        Ok(CmdParser {
            is_expression: false,
            cmd,
            args,
            script_line: text_line.clone(),
        })
    }

    pub fn execute_cmd(self) -> Result<(Self), Box<dyn std::error::Error>> {
        let mut current_dir = CURRENT_DIR.lock().unwrap();
        use crate::commands::commands::Commands::*;

        if self.is_expression{
            let res = Parser::new(&self.script_line).parse_expression(0) ;
            print_success!("Result: {}", eval(&res));
            return Ok(self);
        }



        if self.script_line.contains(";") || self.script_line.contains("&&") || self.script_line.contains("||") {
            match Interpreter::new_with_lines(&self.script_line).interpret() {
                Ok(_) => {}
                Err(err) => {
                    print_error!("{}", err.to_string());
                }
            }
            return Ok(self);
        }

        match &self.cmd {

            Exit(_) => {
                match ExitCommand::exit(self.args.first()){
                    Ok(_) => {}
                    Err(err) => {
                        print_error!("{}", err.to_string());
                    }
                };
            },

            Type(_cmd) | Which(_cmd) => {
                match self.cmd.type_cmd(&self.args[0]) {
                    Ok(res) => print_success!("{}", res),
                    Err(err) => print_error!("{}", err.to_string()),
                };
            }

            Echo(_) => match EchoCommand::new(self.args.clone(), &current_dir).run() {
                Ok(_) => {}
                Err(err) => print_error!("echo: {}", err.to_string())
            },

            Clear(_) => match ClearCommand::run() {
                Ok(_) => {}
                Err(err) => {
                    print_error!("{}", err.to_string());
                }
            },

            Ls(_) => match LsCommand::new(&current_dir, &self.args) {
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
                *current_dir = match CdCommand::new(&current_dir, &self.args) {
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
            Pwd(_) => print_success!("{}", &current_dir.display()),
            Mkdir(_) => match MkdirCmd::new(&self.args, &current_dir) {
                Ok(res) => match res.run() {
                    Ok(_) => {}
                    Err(err) => {
                        print_success!("{}", err.to_string());
                    }
                },
                Err(_err) => {}
            },

            NotBuildIn(_) => {
                match ExecutableCmds::new(&self.cmd.get_cmd(), &self.args, &current_dir){
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
                match Interpreter::new_with_lines(&self.script_line).interpret() {
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
