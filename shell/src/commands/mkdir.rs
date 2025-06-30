use std::path::PathBuf;

pub struct MkdirCmd {
    name: String,
    current_dir: PathBuf,
}

impl MkdirCmd {
    pub fn new(
        name: &Vec<String>,
        current_dir: &PathBuf,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let name = match name.first() {
            Some(name) => name.trim(),
            None => return Err(From::from("No name provided")),
        };
        let current_dir = current_dir.to_path_buf();
        Ok(Self {
            name: String::from(name),
            current_dir,
        })
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Create the full path by joining current directory with the new directory name
        let full_path = self.current_dir.join(&self.name);

        match std::fs::create_dir(&full_path) {
            Ok(_) => Ok(()),
            Err(e) => {
                if e.kind() == std::io::ErrorKind::AlreadyExists {
                    Err(format!("mkdir: {}: File exists", &self.name).into())
                } else {
                    Err(format!("mkdir: {}: {}", &self.name, e).into())
                }
            }
        }
    }
}
