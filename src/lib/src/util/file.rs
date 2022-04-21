
use std::fs;
use std::path::{Path};
use rayon::prelude::*;
use indicatif::ProgressBar;

pub fn hard_link_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    if dst.exists() {
        fs::remove_dir_all(dst)?;
    }

    fs::create_dir_all(&dst)?;

    let mut paths: Vec<fs::DirEntry> = vec![];
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if !ty.is_dir() {
            paths.push(entry);
        }
    };

    println!("Hard link {} files!", paths.len());
    let size: u64 = unsafe { std::mem::transmute(paths.len()) };
    let bar = ProgressBar::new(size);

    paths.par_iter().for_each(|entry| {
        let result = fs::hard_link(entry.path(), dst.join(entry.file_name()));
        match result {
            Ok(_) => {},
            Err(err) => {
                eprintln!("Err: {}", err);
            }
        };
        bar.inc(1);
    });
    bar.finish();
    
    Ok(())
}