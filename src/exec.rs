use serde::{Deserialize, Serialize};

use std::os::unix::process::ExitStatusExt;
use std::process::{Command, Output};

use crate::error::Error;

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum ExecCode {
    Line(String),
    Multi(Vec<String>),
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExecInput {
    pub code: ExecCode,
}

#[derive(Debug, Clone, Serialize)]
pub struct ExecOutput {
    pub stdout: String,
    pub stderr: String,
    pub code: Option<i32>,
    pub signal: Option<i32>,
}

impl From<Output> for ExecOutput {
    fn from(output: Output) -> Self {
        ExecOutput {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            code: output.status.code(),
            signal: output.status.signal(),
        }
    }
}

pub fn python(code: &str) -> Result<ExecOutput, Error> {
    let output = Command::new("python").arg("-c").arg(code).output()?;
    Ok(output.into())
}
