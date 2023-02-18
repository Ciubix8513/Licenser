//The GPLv3 License (GPLv3)
//
//Copyright (c) 2023 Ciubix8513
//
//This program is free software: you can redistribute it and/or modify
//it under the terms of the GNU General Public License as published by
//the Free Software Foundation, either version 3 of the License, or
//any later version.
//
//This program is distributed in the hope that it will be useful,
//but WITHOUT ANY WARRANTY; without even the implied warranty of
//MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//GNU General Public License for more details.
//
//You should have received a copy of the GNU General Public License
//along with this program.  If not, see <http://www.gnu.org/licenses/>.
use clap::Parser;
use std::{
    fs::{self, OpenOptions},
    io::{Error, Read, Seek, Write},
    path::PathBuf,
};

//2+ args
//1st 2 are mandatory
//1st directory to license
//2nd license file
//3 file extensions as a single string
#[derive(Parser, Debug)]
#[command(author,version,about,long_about = None)]
struct Args {
    #[arg(
        long = "dry-run",
        help = "Performs a dry run, showing how many files would be affected"
    )]
    dry_run: bool,
    #[arg(
        short,
        long,
        help = "Specifies the directory to add license notices to"
    )]
    directory: String,
    #[arg(
        short,
        long,
        default_value = "",
        required_unless_present("dry_run"),
        help = "Specifies the file containing the license notice"
    )]
    license: String,
    #[arg(short, long, help = "Specifies what file extensions to license")]
    extensions: Option<String>,
    #[arg(short, long, help = "Prints all licensed files")]
    verbose: bool,
}

//Thank you chat gpt, I love you so much
fn insert_text_to_file(filename: PathBuf, text: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new().read(true).write(true).open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    file.rewind()?;
    file.write_all(text.as_bytes())?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

fn get_files(path: &str) -> Result<Vec<PathBuf>, Error> {
    let contents = std::fs::read_dir(path)?;
    let mut files = Vec::new();
    for c in contents {
        let c = c?;
        if c.file_type()?.is_dir() {
            // dirs.push(c.path());
            let mut recursive = get_files(c.path().to_str().unwrap())?;
            files.append(&mut recursive);
            continue;
        }
        files.push(c.path());
    }
    Ok(files)
}

fn correct_file_ext(checking: &str, exts: &str) -> bool {
    if !checking.contains('.') {
        return false;
    }
    let ext = checking.split('.').last().unwrap();
    let exts: Vec<&str> = exts.split(' ').collect();
    exts.contains(&ext)
}

fn license_file(path: PathBuf, license: &str, verbose: bool) -> std::io::Result<()> {
    insert_text_to_file(path.clone(), license)?;
    if verbose {
        println!("Licensing {0}", path.to_str().unwrap());
    }
    Ok(())
}

fn main() {
    let args = Args::parse();
    if args.dry_run {
        dry_run(&args.directory, args.extensions, args.verbose);
        return;
    }
    let license = &fs::read_to_string(args.license).expect("Failed to read the license file");
    let f = get_files(&args.directory).unwrap();
    let exts = args.extensions;
    let mut count = 0;
    for f in f {
        match exts {
            Some(_) => license_file(f, license, args.verbose).expect("Failed to license a file"),
            None => {
                if correct_file_ext(f.to_str().unwrap(), &exts.clone().unwrap()) {
                    license_file(f, license, args.verbose).expect("Failed to license a file");
                }
            }
        }
        count += 1;
    }
    println!("Licensed {0} files", count);
}

fn dry_run(path: &str, exts: Option<String>, verbose: bool) {
    let f = get_files(path).unwrap();
    let mut count = 0;
    println!("Licensing:");
    match exts {
        Some(e) => {
            for f in f {
                if correct_file_ext(f.to_str().unwrap(), &e) {
                    count += 1;
                    if verbose {
                        println!("{}", f.to_str().unwrap())
                    }
                }
            }
        }
        None => {
            count = f.len();
            if verbose {
                for f in f {
                    println!("{}", f.to_str().unwrap());
                }
            }
        }
    }
    println!("Would've licensed {0} files", count);
}
