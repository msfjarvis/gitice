use git2::Repository;
use std::path::Path;
use walkdir::WalkDir;

fn main() -> anyhow::Result<()> {
    let dir = match std::env::args().nth(1) {
        Some(d) => d,
        None => {
            println!("Usage:\n  gitice <dir>\n");
            return Ok(());
        }
    };
    let mut items: Vec<String> = Vec::new();
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_dir() {
            let path = format!("{}/.git", entry.path().display());
            let git_dir = Path::new(&path);
            if git_dir.exists() {
                let repo = Repository::open(git_dir)?;
                if repo.is_empty()? {
                    continue;
                }

                items.push(format!(
                    "{} = {}",
                    entry.path().to_string_lossy().to_string(),
                    repo.head()?.name().unwrap_or("None")
                ));
            }
        };
    }
    println!("{:#x?}", items);
    Ok(())
}
