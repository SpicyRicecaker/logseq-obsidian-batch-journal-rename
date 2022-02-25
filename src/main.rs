use std::{env::args, fs, path::PathBuf};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // first take the argument of the directory which we must read
    let mut args = args();
    args.next();
    let path = args.next();

    // copy by default
    if let Some(mut arg) = path {
        // copy to a copy dir
        if arg == "-copy" {
            if let Some(copy_path) = args.next() {
                // create fake dir instead, right next to copied dir
                let mut old_path: PathBuf = copy_path.into();

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
                    // set new as actual path
                } else {
                    eprintln!("not dir");
                    std::process::exit(1);
                }
            }
        } 
        todo!()
    } else {
        eprintln!("GG NO PATH ADDED");
    }
    Ok(())
}
