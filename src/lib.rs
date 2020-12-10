pub mod exec;

use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Couldn't execute the Python code ({source})")]
    ExecutionFail {
        #[from]
        source: io::Error,
    },
}
