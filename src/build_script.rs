use serde::{Deserialize, Serialize};

/// A build script is a script that is run after the tarball is extracted.
/// It allows you to do things like compile the source code, set PATH variables, etc.
/// None means that there is no build script.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BuildScript {
    /// A python build script. String is the path to the scrip relative to the root of the tarball.
    Python(String),
    /// A shell build script. String is the path to the script relative to the root of the tarball.
    Shell(String),
    /// A batch build script. String is the path to the script relative to the root of the tarball.
    Batch(String),
    /// A rust build script. String is the path to the directory containing the Cargo.toml file
    /// relative to the root of the tarball.
    Rust(String),
    /// No build script.
    None,
}

impl BuildScript {
    pub fn from_str(script: &str) -> Self {
        match script {
            "python" => BuildScript::Python("build.py".to_string()),
            "shell" => BuildScript::Shell("build.sh".to_string()),
            "batch" => BuildScript::Batch("build.bat".to_string()),
            "rust" => BuildScript::Rust(".".to_string()),
            _ => BuildScript::None,
        }
    }

    fn execute_python(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            BuildScript::Python(path) => {
                let mut command = std::process::Command::new("python");
                command.arg(path);
                let output = command.output()?;
                println!("Python build script output: {}", String::from_utf8_lossy(&output.stdout));
                Ok(())
            },
            _ => Err("Not a python build script".into()),
        }
    }

    fn execute_shell(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            BuildScript::Shell(path) => {
                let mut command = std::process::Command::new("sh");
                command.arg(path);
                let output = command.output()?;
                println!("Shell build script output: {}", String::from_utf8_lossy(&output.stdout));
                Ok(())
            },
            _ => Err("Not a shell build script".into()),
        }
    }

    fn execute_batch(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            BuildScript::Batch(path) => {
                let mut command = std::process::Command::new("cmd");
                command.arg("/C");
                command.arg(path);
                let output = command.output()?;
                println!("Batch build script output: {}", String::from_utf8_lossy(&output.stdout));
                Ok(())
            },
            _ => Err("Not a batch build script".into()),
        }
    }

    fn execute_rust(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            BuildScript::Rust(path) => {
                let mut command = std::process::Command::new("cargo");
                command.arg("run");
                command.arg("--release");
                command.current_dir(path);
                let output = command.output()?;
                println!("Rust build script output: {}", String::from_utf8_lossy(&output.stdout));
                Ok(())
            },
            _ => Err("Not a rust build script".into()),
        }
    }

    pub fn exec(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            BuildScript::Python(_) => self.execute_python(),
            BuildScript::Shell(_) => self.execute_shell(),
            BuildScript::Batch(_) => self.execute_batch(),
            BuildScript::Rust(_) => self.execute_rust(),
            BuildScript::None => Ok(()),
        }
    }
}