use git2::{BlameHunk, Repository};
use nvim_oxi::api::{
    self, Buffer,
    opts::CreateCommandOpts,
    types::{CommandArgs, CommandNArgs},
};
use nvim_oxi::{Dictionary, Function, Result, print as vim_print};
use std::{
    ops::Range,
    path::{Path, PathBuf},
};

#[nvim_oxi::plugin]
fn blme() -> nvim_oxi::Result<Dictionary> {
    let opts = CreateCommandOpts::builder()
        .bang(true)
        .desc("shows a greetings message")
        .nargs(CommandNArgs::ZeroOrOne)
        .build();

    let blame_command = |_: CommandArgs| {
        let buffer = Buffer::current();

        match buffer.get_name() {
            Err(_) => vim_print!("Couldn't get buffer name"),
            Ok(name) => match blame(&name) {
                Ok(blames) => {
                    vim_print!("{:#?}", blames);
                }
                Err(error) => vim_print!("{}", error),
            },
        }
    };

    api::create_user_command("BlameFile", blame_command, &opts)?;

    let say_hello = Function::from_fn(move |()| {
        vim_print!("Hello from rust");
    });

    let api = Dictionary::from_iter([("say_hello", say_hello)]);

    Ok(api)
}

// Provide git blame
#[derive(Debug)]
pub struct BlameRange {
    pub name: String,

    pub email: String,

    /// The line ranges for this blame hunk.
    /// The start index is inclusive and the end index is **exclusive**
    pub range: Range<usize>,

    pub commit: String,
}

impl From<BlameHunk<'_>> for BlameRange {
    fn from(value: BlameHunk) -> Self {
        let sig = value.final_signature();
        let name = sig.name().expect("could not get name").to_owned();
        let email = sig.email().expect("could not get email").to_owned();
        let commit = value.final_commit_id().to_string();

        let start_line = value.final_start_line();
        let end_line = value.final_start_line() + value.lines_in_hunk();
        let range = start_line..end_line;
        Self {
            name,
            email,
            range,
            commit,
        }
    }
}
fn blame(buffer_name: &PathBuf) -> anyhow::Result<Vec<BlameRange>> {
    // open as repository
    let repo = Repository::open_from_env()
    .map_err(|_| anyhow::anyhow!("The buffer is not located in a git repository, or there was some issue opening the repository. Most likely its the first one, though."))?;

    // here we need to strip the .git folder
    // whenever you get the directory from the env variables the path points to the .git file, this
    // makes it difficult to the the diffing afterwards, as we need a relative path to the buffer
    // from the root of the repo
    let repo_path = repo.path().parent().ok_or(anyhow::anyhow!(
        "No parent directory for this repo. Something went horribly wrong."
    ))?;

    let absolute_buffer_path = Path::new(&buffer_name);
    let rel_file_path = absolute_buffer_path.strip_prefix(repo_path).map_err(|_| {
        anyhow::anyhow!(
            "Failed to strip prefix. Repo path:{}, Buffer path:{}",
            repo_path.to_string_lossy(),
            absolute_buffer_path.to_string_lossy()
        )
    })?;

    let blame = repo.blame_file(rel_file_path, None)?;

    println!("Detected Hunks: {}", blame.len());

    let blame_info: Vec<BlameRange> = blame
        .iter()
        .map(|a| Into::<BlameRange>::into(a))
        .collect::<Vec<BlameRange>>();

    return Ok(blame_info);
}

#[cfg(test)]
mod tests {
    use git2::Repository;
    use std::path::{Path, PathBuf};

    use crate::BlameRange;

    #[test]
    fn it_works() {
        // Get path
        let repository_path = Path::new(r"C:\Users\Sergio.Lopes\proteus-now");
        // open as repository
        let repo = Repository::open(repository_path).expect("could not open repository path");

        let mut rel_file_path = PathBuf::new();
        rel_file_path.push("src/suite/Ngb.ProteusNow.Suite.WebApi/Hubs/Pod/PodHub.cs");

        let blame = repo
            .blame_file(rel_file_path.as_path(), None)
            .expect("failed to execute blame");

        println!("Detected Hunks: {}", blame.len());

        let names = blame.iter().map(|a| Into::<BlameRange>::into(a));

        for name in names {
            println!("{:#?}", name);
        }
    }

    #[nvim_oxi::test]
    fn test() {
        // Get current puffer path
        // print path to nvim console
        super::blme();
    }
}
