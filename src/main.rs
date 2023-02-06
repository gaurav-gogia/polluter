use std::error::Error;
use std::path::PathBuf;
use std::str::FromStr;
use std::{env, fs, process};

mod constant;
mod fileio;
mod util;

fn main() {
    let (abs_path, limit, depth) = match get_inputs() {
        Ok((abs_path, limit, depth)) => (abs_path, limit, depth),
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    };
    println!(
        "Generating {} files with {} depth in {}",
        limit,
        depth,
        abs_path.display(),
    );

    match fileio::generate_files(abs_path, limit, depth) {
        Ok(_) => println!("Done!"),
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    }
}

fn get_inputs() -> Result<(PathBuf, u32, u32), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        let cwd = PathBuf::from(".");
        let lidep: u32 = 1;
        return Ok((cwd, lidep, lidep));
    }

    let fpathstr = args[1].clone();
    let fpath = PathBuf::from(fpathstr);
    let mut abs_path = fs::canonicalize(fpath)?;

    if abs_path.is_file() {
        abs_path.pop();
    }

    let limit = u32::from_str(&args[2])?;
    let depth = u32::from_str(&args[3])?;

    return Ok((abs_path, limit, depth));
}
