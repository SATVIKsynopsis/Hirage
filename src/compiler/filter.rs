pub fn extract_function_mir(mir: &str, fn_name: &str) -> String {
    let mut result = String::new();
    let mut capture = false;

    for line in mir.lines() {
        if line.trim_start().starts_with("fn ")
            && (line.contains(&format!("::{}", fn_name))
                || line.contains(&format!("fn {}", fn_name)))
        {
            capture = true;
        }

        if capture {
            result.push_str(line);
            result.push('\n');

            if line.trim() == "}" {
                break;
            }
        }
    }

    if result.is_empty() {
        format!("MIR not found for {}", fn_name)
    } else {
        result
    }
}
