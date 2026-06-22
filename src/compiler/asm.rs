use std::process::Command;

pub fn generate_asm_for_function(function_name: &str) -> Result<String, String> {
    let symbol = find_symbol(function_name)?;

    let output = Command::new("cargo")
        .args(["asm", &symbol])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

pub fn list_asm_symbols() -> Result<String, String> {
    let output = Command::new("cargo")
        .arg("asm")
        .output()
        .map_err(|e| e.to_string())?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn find_symbol(function_name: &str) -> Result<String, String> {
    let symbols = list_asm_symbols()?;

    for line in symbols.lines() {
        if line.contains(function_name)
            && let Some(start) = line.find('"')
            && let Some(end) = line[start + 1..].find('"')
        {
            return Ok(line[start + 1..start + 1 + end].to_string());
        }
    }

    Err(format!("ASM symbol not found: {}", function_name))
}
