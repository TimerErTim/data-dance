#![feature(exit_status_error)]

use std::path::Path;
use std::{fs, io, process};

fn main() {
    build_frontend();
}

fn build_frontend() {
    println!("building frontend...");

    process::Command::new("npm")
        .args(["install"])
        .current_dir(fs::canonicalize("frontend/").unwrap())
        .status()
        .expect("failed to execute process")
        .exit_ok()
        .expect("failed to run npm install");

    process::Command::new("npm")
        .args(["run", "build"])
        .current_dir(fs::canonicalize("frontend/").unwrap())
        .status()
        .expect("failed to execute process")
        .exit_ok()
        .expect("failed to run npm build");

    let _ = fs::remove_dir_all("target/site/");
    copy_dir_all("frontend/out/", "target/site/").unwrap();
    println!("cargo:rerun-if-changed=frontend/");
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
