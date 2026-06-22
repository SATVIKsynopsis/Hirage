use std::process::Command;

pub fn generate_mir() -> Result<String, String> {
    let output = Command::new("cargo")
        .args(["rustc", "--", "-Zunpretty=mir"])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}
