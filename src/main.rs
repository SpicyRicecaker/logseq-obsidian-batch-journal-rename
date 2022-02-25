use anyhow::{Context, Result};
use std::{env::args, fs, path::PathBuf};

use chrono::NaiveDate;
use regex::Regex;

use log::*;
fn main() -> Result<()> {
    env_logger::init();
    // first take the argument of the directory which we must read
    let mut args = args();
    args.next();

    let logseq = Regex::new(r"(?P<content>\d{4}_\d{2}_\d{2})(?P<extension>[^\\/]*\.md)$")?;
    let obsidian = Regex::new(r"(?P<content>\d{2}\d{2}\d{2})(?P<extension>[^\\/]*\.md)$")?;
    let scuffed = Regex::new(r"(?P<content>\d{2}.\d{2}.\d{2})(?P<extension>[^\\/]*\.md)$")?;

    // copy by default
    if let Some(copy_path) = args.next() {
        // copy to a copy dir
        // if arg == "-copy" {
        // create fake dir instead, right next to copied dir
        let old_path: PathBuf = copy_path.into();

        let new_path: PathBuf = old_path.with_file_name(format!(
            "{}_copy",
            old_path.file_name().unwrap().to_str().unwrap()
        ));
        trace!("new path is {:?} to {:?}", &old_path, &new_path);

        if new_path.is_dir() {
            // actually create new dir
            fs::remove_dir_all(&new_path)?;
        }
        fs::create_dir_all(&new_path)?;
        trace!("created dir at {:?}", &new_path);

        if !old_path.is_dir() {
            fs::create_dir_all(&old_path)
                .with_context(|| format!("error creating dir {:?}", &old_path))?;
        }

        // copy items (non-recursive)
        for entry in fs::read_dir(&old_path)
            .with_context(|| format!("error reading old path, {:?}", old_path))?
        {
            let path = entry?.path();
            let mut t = new_path.clone();
            t.push(path.file_name().unwrap());
            fs::copy(&path, &t)
                .with_context(|| format!("error copying from {:?} to {:?}", path, t))?;
        }

        // read dir (in new copied dir)
        for entry in fs::read_dir(&new_path)
            .with_context(|| format!("error reading old path, {:?}", old_path))?
        {
            let path = entry?.path();

            let file_name = path.file_name().unwrap().to_str().unwrap();

            // we could do loop but we have to then do ew regex stuff
            let (date, extension) = if let Some(captures) = logseq.clone().captures(file_name) {
                (
                    NaiveDate::parse_from_str(&captures["content"], "%Y_%m_%d").with_context(
                        || format!("error converting {} for logseq", &captures["content"]),
                    )?,
                    captures["extension"].to_string(),
                )
            } else if let Some(captures) = obsidian.clone().captures(file_name) {
                (
                    NaiveDate::parse_from_str(&captures["content"], "%m%d%y").with_context(
                        || format!("error converting {} for obsidian", &captures["content"]),
                    )?,
                    captures["extension"].to_string(),
                )
            } else if let Some(captures) = scuffed.clone().captures(file_name) {
                (
                    NaiveDate::parse_from_str(&captures["content"], "%m.%d.%y").with_context(
                        || format!("error converting {} for scuffed", &captures["content"]),
                    )?,
                    captures["extension"].to_string(),
                )
            } else {
                error!("the pattern {} is not supported", file_name);
                panic!();
            };

            let new_file_name = format!("{}{}", date.format("%m.%d.%y"), extension);

            let mut new_path = new_path.clone();
            new_path.push(new_file_name);

            fs::rename(&path, &new_path)?;
            trace!("renamed {:?} to {:?}", &path, &new_path);
        }
    } else {
        error!("no args provided. Usage: [rename] [path_to_folder]");
    }
    Ok(())
}
