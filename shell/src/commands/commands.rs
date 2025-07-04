use std::path::PathBuf;
use crate::commands::executable_cmds::ExecutableCmds;

type CmdName = String;

pub enum Commands {
    Cotsh(CmdName),
    Which(CmdName),
    Type(CmdName),
    Echo(CmdName),
    Cat(CmdName),
    Exit(CmdName),
    Clear(CmdName),
    Pwd(CmdName),
    Cd(CmdName),
    Ls(CmdName),
    NotBuildIn(ExecutableCmds),
    EmptyCommand,
    Mkdir(CmdName),
    InvalidCmd(CmdName),
}

impl Commands {
    pub fn new(cmd: &str) -> Result<Self, Box<dyn std::error::Error>> {
        use crate::commands::commands::Commands::*;
        let res = match cmd {
            "cotsh" => Cotsh("cotsh".into()),
            "type" => Type("type".into()),
            "echo" => Echo("echo".into()),
            "cat" => Cat("cat".into()),
            "exit" => Exit("exit".into()),
            "clear" => Clear("clear".to_string()),
            "pwd" => Pwd("pwd".to_string()),
            "cd" => Cd("cd".to_string()),
            "ls" => Ls("ls".to_string()),
            "mkdir" => Mkdir("mkdir".to_string()),
            "which" => Which("which".to_string()),
            "" => EmptyCommand,
            _ => {
                match ExecutableCmds::new(cmd, &vec![], &PathBuf::new()){
                    Ok(res) => NotBuildIn(res),
                    Err(_) => InvalidCmd(cmd.into())
                }
            },
        };
        Ok(res)
    }

    pub fn get_cmd(&self) -> String {
        use crate::commands::commands::Commands::*;
        match self {
            Cotsh(cmd) => cmd.clone(),
            Which(cmd) => cmd.clone(),
            Type(cmd) => cmd.clone(),
            Echo(cmd) => cmd.clone(),
            Exit(cmd) => cmd.clone(),
            Clear(cmd) => cmd.clone(),
            Cat(cmd) => cmd.clone(),
            Pwd(cmd) => cmd.clone(),
            Cd(cmd) => cmd.clone(),
            Ls(cmd) => cmd.clone(),
            Mkdir(cmd) => cmd.clone(),
            NotBuildIn(cmd) => cmd.executable().into(),
            EmptyCommand => "".to_string(),
            InvalidCmd(cmd) => cmd.clone(),
        }
    }

    pub fn type_cmd(&self, arg: &str) -> Result<String, Box<dyn std::error::Error>> {
        let type_cmd = Self::new(arg)?;
        match type_cmd {
            Commands::EmptyCommand => Ok(String::from("empty command")),
            Commands::InvalidCmd(cmd) => Ok(format!("{} not found", cmd)),
            Commands::NotBuildIn(cmd) => Ok(format!("{} is {}", cmd.executable(), cmd.executable_path())),
            _ => Ok(format!("{} is a shell builtin", type_cmd.get_cmd())),
        }
    }
}
