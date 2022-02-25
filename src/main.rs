use std::{env::args, fs, path::PathBuf};

use chrono::NaiveDate;
use regex::Regex;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // first take the argument of the directory which we must read
    let mut args = args();
    args.next();

    let logseq = Regex::new(r"(?P<content>\d{2}_\d{2}_\d{2})(?P<extension>[^\\/]*\.md)$")?;
    let obsidian = Regex::new(r"(?P<content>\d{2}\d{2}\d{2})(?P<extension>[^\\/]*\.md)$")?;
    let scuffed = Regex::new(r"(?P<content>\d{2}.\d{2}.\d{2})(?P<extension>[^\\/]*\.md)$")?;

    // copy by default
    if let Some(copy_path) = args.next() {
        // copy to a copy dir
        // if arg == "-copy" {
        // create fake dir instead, right next to copied dir
        let old_path: PathBuf = copy_path.into();

        let mut new: PathBuf = old_path.clone();

        new.set_file_name(format!(
            "{}_copy",
            new.file_name().unwrap().to_str().unwrap()
        ));

        // remove this dir if it already exists
        if old_path.is_dir() {
            fs::remove_dir_all(&old_path)?;
            // create new dir
            fs::create_dir(&old_path)?;

            // copy items (non-recursive)
            for entry in fs::read_dir(old_path)? {
                let path = entry?.path();
                let mut t = new.clone();
                t.push(path.file_name().unwrap());
                fs::copy(path, t)?;
            }
            let folder_to_rename = new;

            // read dir
            for entry in fs::read_dir(folder_to_rename)? {
                let path = entry?.path();

                let file_name = path.file_stem().unwrap().to_str().unwrap();

                // we could do loop but we have to then do ew regex stuff
                let (date, extension) = if let Some(captures) = logseq.clone().captures(file_name) {
                    (
                        NaiveDate::parse_from_str(&captures["content"], "%Y_%m_%d")?,
                        captures["extension"].to_string(),
                    )
                } else if let Some(captures) = obsidian.clone().captures(file_name) {
                    (
                        NaiveDate::parse_from_str(&captures["content"], "%m%d%y")?,
                        captures["extension"].to_string(),
                    )
                } else if let Some(captures) = scuffed.clone().captures(file_name) {
                    (
                        NaiveDate::parse_from_str(&captures["content"], "%m.%d.%y")?,
                        captures["extension"].to_string(),
                    )
                } else {
                    panic!("pattern not supported");
                };

                let new_file_name = format!("{}{}", date.format("%m.%d.%y"), extension);

                fs::rename(&path, path.with_file_name(new_file_name))?;
            }
        }
    }
    Ok(())
}
