use dialoguer::Password;
use std::process::{Command, Stdio};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Prompt for passphrase/secret key securely before any confirmation step.
    // This ensures it never sits in an environment variable or command argument where `ps aux` can view it.
    let secret_key = Password::new()
        .with_prompt("Enter your STELLAR_SECRET_KEY")
        .interact()?;

    println!("\n[Verification] Secret key accepted securely.");

    // 2. Clear and sanitize the subprocess environment when invoking an external workflow
    let mut child_cmd = Command::new("stellar");

    child_cmd
        .arg("contract")
        .arg("deploy")
        .env_clear() // Destroys the current environment for the child process to avoid any accidental leak
        .env("PATH", std::env::var("PATH").unwrap_or_default()) // Only bring back absolutely necessary variables
        .stdin(Stdio::piped())
        .stdout(Stdio::piped());

    // You can now safely pass `secret_key` via standard input (stdin) to the child process if needed.
    println!("Subprocess environment sanitized. Workflows ready.");
    Ok(())
}
