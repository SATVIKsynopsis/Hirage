use std::{fs, process::Command};

pub fn generate_llvm() -> Result<String, String> {
    let output = Command::new("cargo")
        .args(["rustc", "--", "--emit=llvm-ir"])
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    let target_dir = "target/debug/deps";

    let entry = fs::read_dir(target_dir)
        .map_err(|e| e.to_string())?
        .filter_map(Result::ok)
        .find(|e| e.path().extension().map(|x| x == "ll").unwrap_or(false));

    let file = entry.ok_or("No LLVM file found")?;

    fs::read_to_string(file.path()).map_err(|e| e.to_string())
}
