use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};

use std::os::unix::process::ExitStatusExt;
use std::process::{Command, Output};

use crate::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExecCode {
    Line(String),
    Multi(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecRequest {
    pub code: ExecCode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecResponse {
    pub stdout: String,
    pub stderr: String,
    pub code: Option<i32>,
    pub signal: Option<i32>,
}

impl From<Output> for ExecResponse {
    fn from(output: Output) -> Self {
        ExecResponse {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            code: output.status.code(),
            signal: output.status.signal(),
        }
    }
}

pub fn exec(req: &ExecRequest) -> Result<ExecResponse, Error> {
    let code = match &req.code {
        ExecCode::Line(line) => line.to_owned(),
        ExecCode::Multi(lines) => {
            let mut code = String::new();
            for line in lines {
                code.push_str(&line);
                code.push('\n');
            }

            code
        }
    };

    cfg_if! {
        if #[cfg(feature = "lambda")] {
            let output = Command::new("/usr/bin/python3.8").arg("-c").arg(code).output()?;
        } else {
            let output = Command::new("/usr/bin/python").arg("-c").arg(code).output()?;
        }
    }

    Ok(output.into())
}
