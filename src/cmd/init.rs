
pub use std::path::{PathBuf, Path};
pub use std::fs;
use super::types::*;

const SETTINGS_RSK: &'static str = r#"\
# This is an artifacts file for the command line tool rsk
# files (like this one) that are in {repo}/.rsk are automatically loaded
# for a tutorial on how to use rsk type `rsk tutorial`

[settings]
artifact_paths = ['{repo}/docs']

[globals]
# This is where you define variables
# There are two variables that can be used anywhere:
# - {cwd}: the path to the directory of the file using it
# - {repo}: the path to the current repository, which is the closest
#    directory (searching down) that contains a ".rsk" folder
"#;

pub fn get_subcommand<'a, 'b>() -> App<'a, 'b> {
    // [SPC-ui-cmdline-cmd-init-interface]
    SubCommand::with_name("init")
        .about("initiailze the repository in the cwd")
        .settings(&[AS::DeriveDisplayOrder, AS::ColoredHelp])
}

pub fn do_init(path: &Path, tutorial: bool) -> io::Result<()> {
    let mut read_dir = try!(fs::read_dir(path));
    let exists = read_dir.any(|e|
        match e {
            Err(_) => false,
            Ok(e) => {
                if !e.file_type().unwrap().is_dir() {
                    false
                } else {
                    let p = e.path();
                    let fname = p.file_name().unwrap().to_str().unwrap();
                    if fname == ".rsk" {
                        true
                    } else {
                        false
                    }
                }
            }
        });
    let repo = path.join(".rsk");
    if !exists {
        try!(fs::create_dir(&repo));

        // create settings
        let settings = repo.join("settings.rsk");
        let mut f = try!(fs::File::create(settings));
        f.write_all(SETTINGS_RSK.as_ref()).unwrap();
        println!("rsk already initialized at {0}", repo.display());
    } else {
        println!("rsk initialized at {0}", repo.display());
    }
    Ok(())
}
