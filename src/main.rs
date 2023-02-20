//The GPLv3 License (GPLv3) llllf
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
    io::{self, Error, Read, Seek, Write},
    path::{Path, PathBuf},
};

#[derive(Parser, Debug)]
#[command(author,version,
    about=
"A small tool to add a license notices
Note:
The program will ignore all files with unknown file extensions",
    long_about = None)]
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
        required_unless_present("dry_run"),
        help = "Specifies the file containing the license notice"
    )]
    license: Option<String>,
    #[arg(short, long, help = "Specifies what file extensions to license")]
    extensions: Option<String>,
    #[arg(short, long, help = "Prints only the number of modified files")]
    silent: bool,
    #[arg(short, long, help = "Automatically add comments")]
    comment: bool,
    #[arg(short, long, help = "Replaces existing license notices with new ones")]
    replace: bool,
}

fn main() {
    let args = Args::parse();
    if args.dry_run {
        dry_run(&args.directory, args.extensions, !args.silent);
        return;
    }
    if args.replace {
        println!("WARNING everything from the beginning of a file to the first empty line is considered to be a license notice and will be replaced");
        println!("Continue? [Y/n]");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim().to_lowercase();
        if !input.is_empty() && input != "y" {
            return;
        }
    }
    let license =
        &fs::read_to_string(args.license.unwrap()).expect("Failed to read the license file");
    let f = get_files(&args.directory).unwrap();
    let exts = args.extensions;
    let mut count = 0;
    for f in f {
        match exts {
            None => license_file(f, license, !args.silent, args.comment, args.replace)
                .expect("Failed to license a file"),
            Some(_) => {
                if correct_file_ext(f.clone(), &exts.clone().unwrap()) {
                    license_file(f, license, !args.silent, args.comment, args.replace)
                        .expect("Failed to license a file");
                }
            }
        }
        count += 1;
    }
    println!("Licensed {0} files", count);
}

//Thank you chat gpt, I love you so much
fn insert_text_to_file(
    filename: PathBuf,
    text: &str,
    replace: bool,
    verbose: bool,
) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(filename.clone())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let comment = get_comment_format(&filename)
        .or(get_multiline_comment_format(&filename)
            .and_then(|x| Some(x.split("\n").collect::<Vec<&str>>()[0])))
        .unwrap_or("");
    if contents
        .split('\n')
        .collect::<Vec<&str>>()
        .first()
        .unwrap()
        .contains(comment)
    {
        if replace {
            let pos = contents.find("\n\n");
            if let Some(pos) = pos {
                contents = contents[pos + 2..].to_string()
            }
        } else {
            if verbose {
                println!("Skipping {0}", filename.to_str().unwrap());
            }
            return Ok(());
        }
    }
    file.rewind()?;
    file.write_all(text.as_bytes())?;
    file.write_all(contents.as_bytes())?;
    if verbose {
        println!("Licensing {0}", filename.to_str().unwrap());
    }
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

fn correct_file_ext(checking: PathBuf, exts: &str) -> bool {
    let ext = checking.extension();
    if ext.is_none() {
        return false;
    }
    let exts: Vec<&str> = exts.split(' ').collect();
    exts.contains(&ext.unwrap().to_str().unwrap())
}

fn license_file(
    path: PathBuf,
    license: &str,
    verbose: bool,
    comment: bool,
    replace: bool,
) -> std::io::Result<()> {
    if comment {
        let license = comment_string(license, path.clone());
        if license.is_none() {
            if verbose {
                println!(
                    "Was unable to license {0}, no comment format found",
                    path.to_str().unwrap()
                );
            }
            return Ok(());
        }
        let license = license.unwrap();

        insert_text_to_file(path, &license, replace, verbose)?;
    } else {
        insert_text_to_file(path, license, replace, verbose)?;
    }
    Ok(())
}

fn dry_run(path: &str, exts: Option<String>, verbose: bool) {
    let f = get_files(path).unwrap();
    let mut count = 0;
    println!("Licensing:");
    match exts {
        Some(e) => {
            for f in f {
                if correct_file_ext(f.clone(), &e) {
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

fn comment_string(input: &str, filename: PathBuf) -> Option<String> {
    let comment = get_comment_format(&filename);
    if comment.is_none() {
        //Try to get multiline
        let comment = get_multiline_comment_format(&filename)?;
        let comment: Vec<&str> = comment.split('\n').collect();
        return Some(String::from(comment[0]) + "\n" + input + comment[1] + "\n\n");
    }
    let comment = comment.unwrap();
    let mut a = comment.to_owned() + &input.replace('\n', &(String::from("\n") + comment));
    if a.split('\n').last().unwrap() == comment {
        a.truncate(a.len() - comment.len())
    }
    Some(a + "\n")
}

//Thank you chatGPT
#[allow(unreachable_patterns)]
fn get_comment_format(filename: &Path) -> Option<&'static str> {
    match filename.extension().and_then(|ext| ext.to_str()) {
        Some("cpp") | Some("hpp") | Some("cc") | Some("cxx") | Some("hxx") | Some("hh")
        | Some("c++") | Some("inl") | Some("java") | Some("js") | Some("ts") | Some("cs")
        | Some("swift") | Some("kt") | Some("kts") | Some("go") | Some("rs") | Some("dart")
        | Some("rb") | Some("scala") => Some("//"),
        Some("py") | Some("rb") | Some("pl") | Some("pm") | Some("sh") | Some("r")
        | Some("coffee") | Some("haml") | Some("sass") | Some("scss") | Some("Makefile")
        | Some("makefile") | Some("Makefile.inc") | Some("makefile.inc") | Some("Dockerfile") => {
            Some("#")
        }
        _ => None,
    }
}
fn get_multiline_comment_format(filename: &Path) -> Option<&'static str> {
    match filename.extension().and_then(|ext| ext.to_str()) {
        Some("html") | Some("xml") | Some("xhtml") => Some("<!--\n-->"),
        Some("css") => Some("/*\n*/"),
        Some("razor") => Some("@*\n*@"),
        _ => None,
    }
}
