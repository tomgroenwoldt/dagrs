//! yaml configuration file type parser
//!
//! # Config file parser
//!
//! Use yaml configuration files to define a series of tasks, which eliminates the need for users to write code.
//! [`YamlParser`] is responsible for parsing the yaml configuration file into a series of [`YamlTask`].
//! The program specifies the properties of the yaml task configuration file. The basic format of the yaml
//! configuration file is as follows:
//!
//! ```yaml
//! dagrs:
//!   a:
//!     name: "Task 1"
//!     after: [ b, c ]
//!     cmd: echo a
//!   b:
//!     name: "Task 2"
//!     after: [ c, f, g ]
//!     cmd: echo b
//!   c:
//!     name: "Task 3"
//!     after: [ e, g ]
//!     cmd: echo c
//!   d:
//!     name: "Task 4"
//!     after: [ c, e ]
//!     cmd: echo d
//!   e:
//!     name: "Task 5"
//!     after: [ h ]
//!     cmd: echo e
//!   f:
//!     name: "Task 6"
//!     after: [ g ]
//!     cmd: python3 ./tests/config/test.py
//!   g:
//!     name: "Task 7"
//!     after: [ h ]
//!     cmd: node ./tests/config/test.js
//!   h:
//!     name: "Task 8"
//!     cmd: echo h
//! ```
//!
//! Users can read the yaml configuration file programmatically or by using the compiled `dagrs`
//! command line tool. Either way, you need to enable the `yaml` feature.
//!
//! # Example
//!
//! ```rust
//! use dagrs::Dag;
//! let dag = Dag::with_yaml("some_path",std::collections::HashMap::new());
//! ```

mod yaml_parser;
mod yaml_task;

use crate::DagError;

pub use self::yaml_parser::YamlParser;
pub use self::yaml_task::YamlTask;

/// Errors about task configuration items.
#[derive(Debug)]
pub enum YamlTaskError {
    /// The configuration file should start with `dagrs:`.
    StartWordError,
    /// No task name configured.
    NoNameAttr(String),
    /// The specified task predecessor was not found.
    NotFoundPrecursor(String),
    /// `script` is not defined.
    NoScriptAttr(String),
}

/// Error about file information.
#[derive(Debug)]
pub enum FileContentError {
    /// The format of the yaml configuration file is not standardized.
    IllegalYamlContent(yaml_rust::ScanError),
    Empty(String),
}

/// Configuration file not found.
pub struct FileNotFound(pub std::io::Error);

impl From<YamlTaskError> for DagError {
    fn from(value: YamlTaskError) -> Self {
        let error_message = match value {
            YamlTaskError::StartWordError => "File content is not start with 'dagrs'.".to_string(),
            YamlTaskError::NoNameAttr(ref msg) => {
                format!("Task has no name field. [{}]", msg)
            }
            YamlTaskError::NotFoundPrecursor(ref msg) => {
                format!("Task cannot find the specified predecessor. [{}]", msg)
            }
            YamlTaskError::NoScriptAttr(ref msg) => {
                format!("The 'script' attribute is not defined. [{}]", msg).into()
            }
        };
        DagError::ParserError(error_message)
    }
}

impl From<FileContentError> for DagError {
    fn from(value: FileContentError) -> Self {
        let error_message = match value {
            FileContentError::IllegalYamlContent(ref err) => err.to_string(),
            FileContentError::Empty(ref file) => format!("File is empty! [{}]", file),
        };
        DagError::ParserError(error_message)
    }
}

impl From<FileNotFound> for DagError {
    fn from(value: FileNotFound) -> Self {
        DagError::ParserError(format!("File not found. [{}]", value.0))
    }
}
