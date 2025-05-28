

type MultiArgs = Vec<String>;
type SingleArg = String;
type CmdName = String;

pub enum Commands {
    Type(CmdName),
    Echo(CmdName),
    Cat(CmdName),
    Exit(CmdName),
    Clear(CmdName),
    Pwd(CmdName),
    Cd(CmdName),
    Ls(CmdName),
    NotBuildIn(CmdName),
    EmptyCommand,
}

impl Commands {
    pub fn new(cmd: &str) -> Result<Self, Box<dyn std::error::Error>> {
        use crate::commands::build_in::Commands::*;
        let res = match cmd {
            "type" => Type("type".into()),
            "echo" => Echo("echo".into()),
            "cat" => Cat("cat".into()),
            "exit" => Exit("exit".into()),
            "clear" => Clear("clear".to_string()),
            "pwd" => Pwd("pwd".to_string()),
            "cd" => Cd("cd".to_string()),
            "ls" => Ls("ls".to_string()),       
            "" => EmptyCommand,
            _ => NotBuildIn(cmd.to_string())
        };
        Ok(res)
    }

    pub fn get_cmd(&self) -> String {
        use crate::commands::build_in::Commands::*;
        match self {
            Type(cmd) => cmd.clone(),
            Echo(cmd) => cmd.clone(),
            Exit(cmd) => cmd.clone(),
            Clear(cmd) => cmd.clone(),
            Cat(cmd) => cmd.clone(),
            Pwd(cmd) => cmd.clone(),
            Cd(cmd) => cmd.clone(), 
            Ls(cmd) => cmd.clone(),
            NotBuildIn(cmd) => cmd.clone(),
            EmptyCommand => "".to_string(),
            _ => {"".to_string()}
        }
    }

    pub fn type_cmd(&self, arg : &str) -> Result<String, Box<dyn std::error::Error>> {
        let type_cmd = Self::new(arg)?;
        match type_cmd {
            Commands::EmptyCommand => Ok(String::from("empty command")),
            Commands::NotBuildIn(cmd) => Ok(format!("{} is not a shell builtin", cmd)),
            _ => Ok(format!("{} is a shell builtin", type_cmd.get_cmd()))
        }

    }
}