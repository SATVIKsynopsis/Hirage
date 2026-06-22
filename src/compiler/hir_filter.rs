pub fn extract_function_hir(hir: &str, fn_name: &str) -> String {
    let start_pattern = format!("fn {}", fn_name);

    let start = match hir.find(&start_pattern) {
        Some(pos) => pos,
        None => {
            return format!("HIR not found: {}", fn_name);
        }
    };

    let remaining = &hir[start..];

    let end = remaining
        .find("\n        fn ")
        .filter(|&i| i > 0)
        .unwrap_or(remaining.len());

    remaining[..end].to_string()
}
