use walkdir::WalkDir;

pub fn discover_files() -> Vec<String> {
    let mut files = Vec::new();

    for entry in WalkDir::new("src").into_iter().filter_map(Result::ok) {
        let path = entry.path();

        if path.is_file() {
            files.push(path.display().to_string());
        }
    }

    files.sort();

    files
}
