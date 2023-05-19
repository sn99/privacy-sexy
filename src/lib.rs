//! - privacy-sexy is a data-driven application where it reads the necessary OS-specific logic from
//!   yaml files in [`collections`](https://github.com/sn99/privacy-sexy/tree/master/collections)
//! - 💡 Best practices
//!   - If you repeat yourself, try to utilize [YAML-defined functions](FunctionData)
//!   - Always try to add documentation and a way to revert a tweak in [scripts](ScriptData)
//! - 📖 Types in code: [`collections.rs`](https://github.com/sn99/privacy-sexy/blob/master/src/collection.rs)

mod collection;
mod util;

pub use collection::{CollectionData, Recommend};

use serde::{Deserialize, Serialize};
use std::{
    env, fs,
    path::PathBuf,
    process::{Command, ExitStatus},
};

/// Allowed values for OS
#[derive(Debug, Serialize, Deserialize)]
pub enum OS {
    /// Apple
    #[serde(rename = "macos")]
    MacOs,
    /// Microsoft
    #[serde(rename = "windows")]
    Windows,
    /// OpenSource 💕
    #[serde(rename = "linux")]
    Linux,
}

/// Main way to get rules in form of [`CollectionData`]
///
/// # Errors
///
/// Refer to [`from_file`](CollectionData)
///
/// # Panics
///
/// Panics for [`OS::Linux`]
pub fn get_collection(os: &OS) -> Result<CollectionData, Box<dyn std::error::Error>> {
    let mut coll_file = PathBuf::from("collections");

    coll_file.push(match os {
        OS::MacOs => "macos.yaml",
        OS::Linux => panic!("No rules yet!"),
        OS::Windows => "windows.yaml",
    });

    CollectionData::from_file(coll_file)
}

/// Runs the script
///
/// # Errors
///
/// Returns [`Err`] if it is unable to:
/// - write to the temp script file OR
/// - change it's permissions (for unix) OR
/// - execute the script
pub fn run_script(
    script_string: &str,
    file_extension: Option<String>,
) -> Result<ExitStatus, Box<dyn std::error::Error>> {
    let mut tmp_file = env::temp_dir();
    tmp_file.push("privacy-sexy");
    if let Some(ext) = file_extension {
        tmp_file.set_extension(ext);
    }

    fs::write(&tmp_file, script_string)?;

    #[cfg(target_family = "unix")]
    {
        use std::os::unix::prelude::PermissionsExt;
        fs::set_permissions(&tmp_file, fs::Permissions::from_mode(0o755))?;
    }

    Ok(Command::new(tmp_file.to_str().unwrap_or_default()).spawn()?.wait()?)
}
