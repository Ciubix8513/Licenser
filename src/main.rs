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
use std::{
    fs::{self, OpenOptions},
    io::{Seek,Write,Error},
    path::PathBuf
};

use std::io::prelude::*;

//Thank you chat gpt, I love you so much
fn insert_text_to_file(filename: PathBuf, text: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new().read(true).write(true).open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    file.seek(std::io::SeekFrom::Start(0))?;
    file.write_all(text.as_bytes())?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

fn get_files(path: &str) ->Result<Vec<PathBuf>,Error> {
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
    return Ok(files);
}

fn correct_file_ext(checking: &str, exts: &str) -> bool{
    if !checking.contains(".") {
        return false;
    }
    let ext = checking.split(".").last().unwrap();
    let exts: Vec<&str> = exts.split(' ').collect();
    return exts.contains(&ext);
}

fn license_file(path: PathBuf, license: &str) -> std::io::Result<()>{
    println!("Licensing {0}", path.to_str().unwrap());
    insert_text_to_file(path, license)?;
    return  Ok(());
}

fn main() {
    //2+ args
    //1st 2 are mandatory
    //1st directory to license
    //2nd license file
    //3 file extensions as a single string
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        print_usage();
        return;
    }
    //Load license notice
    let license = &fs::read_to_string(&args[2]).expect("Failed to read the license file");

    let f = get_files(&args[1]).unwrap();
    let mut count = 0;
    for f in f {
        if args.len() == 3 {
            license_file(f, license).expect("Failed to license a file");
            count += 1;
            continue;
        }
        if correct_file_ext(f.to_str().unwrap(),&args[3]) {
            license_file(f, license).expect("Failed to license a file");
            count += 1;
        }
    }
    println!("Licensed {0} files", count);
}

fn print_usage(){
    println!("Usage:\n licenser \"Directory/To/License\" \"Path/to/license/notice\" (optional)\"list of file extensions to license\" ");
}