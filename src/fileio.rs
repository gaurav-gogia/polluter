use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::process;
use std::{sync::mpsc, thread};

use super::constant;
use super::util;

pub fn generate_files(root_path: PathBuf, limit: u32, depth: u32) -> Result<(), Box<dyn Error>> {
    if depth == 0 {
        return Ok(());
    }

    let max = constant::get_max_threads_limit();
    let mut active: u32 = 0;
    let (tx, rx) = mpsc::channel();

    for _ in 0..limit {
        if active > max {
            match rx.recv() {
                Ok(()) => active -= 1,
                Err(e) => println!("RECV: {}", e.to_string()),
            }
        }

        let dirpath = util::generate_file_path(root_path.clone(), constant::FOLDER);
        let dirpath_copy = dirpath.clone();
        let txi = tx.clone();

        active += 1;
        thread::spawn(move || match populate_dir(dirpath, limit) {
            Ok(_) => txi.send(()).unwrap(),
            Err(e) => {
                println!("SEND: {}", e);
                process::exit(1);
            }
        });

        generate_files(dirpath_copy, limit, depth - 1)?;
    }

    while active > 0 {
        match rx.recv() {
            Ok(()) => active -= 1,
            Err(e) => println!("RECV: {}", e.to_string()),
        }
    }

    return Ok(());
}

fn populate_dir(fdir: PathBuf, limit: u32) -> Result<(), Box<dyn Error>> {
    let max = constant::get_max_threads_limit();
    let mut active: u32 = 0;
    let (tx, rx) = mpsc::channel();

    fs::create_dir_all(fdir.clone())?;
    for _ in 0..limit {
        if active > max {
            match rx.recv() {
                Ok(()) => active -= 1,
                Err(e) => println!("RECV: {}", e.to_string()),
            }
        }

        let fdir_copy = fdir.clone();
        let txi = tx.clone();

        active += 1;
        thread::spawn(move || match generate_txt_file(fdir_copy) {
            Ok(_) => txi.send(()).unwrap(),
            Err(e) => {
                println!("SEND: {}", e);
                process::exit(1);
            }
        });
    }

    while active > 0 {
        match rx.recv() {
            Ok(()) => active -= 1,
            Err(e) => println!("RECV: {}", e.to_string()),
        }
    }

    return Ok(());
}

fn generate_txt_file(fdir: PathBuf) -> Result<(), Box<dyn Error>> {
    let fpath = util::generate_file_path(fdir, constant::TXT_EXTENSION);
    let mut txt_file_handle = File::create(fpath)?;

    let datastr = util::get_random_string(constant::CONTENT_LEN);
    let data = datastr.as_bytes();

    txt_file_handle.write_all(data)?;

    return Ok(());
}
