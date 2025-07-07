use crate::print_success;

pub struct ExitCommand;

impl ExitCommand{

    pub fn exit(code: Option<&String>) -> Result<(), Box<dyn std::error::Error>> {
        if code.is_some(){
            let regex = regex::Regex::new(r"^\d+$").unwrap();
            match regex.is_match(&code.as_ref().unwrap()) {
                true => {},
                false => return Err("Invalid exit code".into())
            }
        }
        let code = code.as_ref().unwrap().parse::<i32>()?;
        print_success!("Exiting with code {}", code);
        std::process::exit(code);
    }
}