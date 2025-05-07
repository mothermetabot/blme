// Provide git blame

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use git2::{BlameOptions, Repository};

    #[test]
    fn it_works() {
        let repo = Repository::open(".").expect("hi");

        let mut cwd = PathBuf::new();
        cwd.push("file.txt");

        println!("{}", cwd.to_string_lossy());

        assert!(cwd.exists());
        let blame = repo.blame_file(cwd.as_path(), None).expect("hi");

        for line in 1..blame.len() {
            println!("{}", blame.get_line(line).expect("hi").lines_in_hunk());
        }
    }
}
