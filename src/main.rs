use git2::Repository;
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    let dir = match std::env::args().nth(1) {
        Some(d) => d,
        None => {
            println!("Usage:\n  gitice <dir>\n");
            return;
        }
    };
    let mut items: Vec<String> = Vec::new();
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_dir() {
            let path = format!("{}/.git", entry.path().display());
            let git_dir = Path::new(&path);
            if git_dir.exists() {
                let repo = match Repository::open(git_dir) {
                    Ok(repo) => repo,
                    Err(e) => {
                        println!("Failed to open repository: {}", e);
                        return;
                    }
                };
                let head = match repo.head() {
                    Ok(head) => head,
                    Err(e) => {
                        println!("Failed getting repository head: {}", e);
                        return;
                    }
                };
                items.push(format!(
                    "{} = {}",
                    entry.path().to_string_lossy().to_string(),
                    head.name().unwrap()
                ));
            }
        };
    }
    println!("{:#x?}", items);
}
