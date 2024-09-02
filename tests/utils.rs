use anyhow::{Context, Result};

use redis::Connection;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;

const VALKEY_PORT: &str = "6379";
const MODULE_NAME: &str = "rustmod";

/// Ensure child process is killed both on normal exit and when panicking due to a failed test.
pub struct ChildGuard {
    name: &'static str,
    child: std::process::Child,
}

impl Drop for ChildGuard {
    fn drop(&mut self) {
        if let Err(e) = self.child.kill() {
            println!("Could not kill {}: {e}", self.name);
        }
        if let Err(e) = self.child.wait() {
            println!("Could not wait for {}: {e}", self.name);
        }
    }
}

pub fn start_valkey_server_with_module() -> Result<ChildGuard> {
    let module_path = get_module_path()?;

    let args = &[
        "--port",
        &VALKEY_PORT,
        "--loadmodule",
        module_path.as_str(),
        "--enable-debug-command",
        "yes",
    ];

    let valkey_server = Command::new("valkey-server")
        .args(args)
        .spawn()
        .map(|c| ChildGuard {
            name: "valkey-server",
            child: c,
        })?;

    Ok(valkey_server)
}

pub fn get_valkey_connection() -> Result<Connection> {
    let client = redis::Client::open(format!("redis://127.0.0.1:{VALKEY_PORT}/"))?;
    loop {
        let res = client.get_connection();
        match res {
            Ok(con) => return Ok(con),
            Err(e) => {
                if e.is_connection_refusal() {
                    // Valkey not ready yet, sleep and retry
                    std::thread::sleep(Duration::from_millis(50));
                } else {
                    return Err(e.into());
                }
            }
        }
    }
}

fn get_module_path() -> Result<String> {
    let extension = if cfg!(target_os = "macos") {
        "dylib"
    } else {
        "so"
    };

    let profile = if cfg!(not(debug_assertions)) {
        "release"
    } else {
        "debug"
    };

    let module_path: PathBuf = [
        std::env::current_dir()?,
        PathBuf::from(format!("target/{profile}/lib{MODULE_NAME}.{extension}")),
    ]
    .iter()
    .collect();

    assert!(fs::metadata(&module_path)
        .with_context(|| format!("Loading valkey module: {}", module_path.display()))?
        .is_file());

    let module_path = format!("{}", module_path.display());
    Ok(module_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_module_path() {
        let path = get_module_path().unwrap();
        assert!(path.contains(format!("{}/target/debug/lib{}", MODULE_NAME, MODULE_NAME).as_str()));
    }
}
