#![warn(rust_2018_idioms, single_use_lifetimes)]

use std::{
    env,
    process::{Command, ExitStatus, Stdio},
};

fn main() {
    if !is_nightly() {
        return;
    }

    let is_ci = env::var_os("CI").map_or(false, |v| v == "true");
    if is_ci {
        println!("cargo:rustc-cfg=ci");
    }

    let cargo_expand = if cfg!(windows) { "cargo-expand.exe" } else { "cargo-expand" };
    if is_ci || has_command(cargo_expand) && has_command("rustfmt") {
        println!("cargo:rustc-cfg=expandtest");
    }
}

fn has_command(command: &str) -> bool {
    Command::new(command)
        .arg("--version")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .as_ref()
        .map(ExitStatus::success)
        .unwrap_or(false)
}

fn is_nightly() -> bool {
    env::var_os("RUSTC")
        .and_then(|rustc| Command::new(rustc).arg("--version").output().ok())
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map_or(false, |version| version.contains("nightly") || version.contains("dev"))
}
