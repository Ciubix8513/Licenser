use std::{path::PathBuf, fs::{self, OpenOptions}, io::{SeekFrom, Write, Seek}};

fn get_files(path: &String) -> Vec<PathBuf> {
    let contents = std::fs::read_dir(path).expect("");
    let mut files = Vec::new();
    for c in contents {
        let c = c.expect("");
        if c.file_type().expect("").is_dir() {
            // dirs.push(c.path());
            let mut recursive = get_files(&c.path().to_str().expect("").to_string());
            files.append(&mut recursive);
            continue;
        }
        files.push(c.path());
    }
    return files;
}

fn correct_file_ext(checking: String, exts: &String) -> bool {
    if !checking.contains(".") {
        return false;
    }
    let ext = checking.split(".").last().expect("");
    return exts.contains(ext);
}

fn license_file(path: PathBuf, license: &String) {
    println!("Licensing {0}",path.to_str().expect("").to_string());
    let mut file = OpenOptions::new().read(true).write(true).open(path).expect("");
    file.seek(SeekFrom::Start(0)).expect("");
    file.write(license.as_bytes()).expect("");
}

fn main() {
    //2+ args
    //1st 2 are mandatory
    //1st directory to license
    //2nd license file
    //3 file extensions as a single string
    let args:Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        return;
    }
    //Load license notice
    let license = &fs::read_to_string(&args[2]).expect("");

    let f = get_files(&args[1]);
    let mut count = 0;
    for f in f {
        if args.len() == 3 {
            license_file(f,license);
            count+=1;
            continue
        }
        if correct_file_ext(f.to_str().expect("").to_string(), &args[3]) {
            license_file(f,license);
            count+=1;
        }
    }
    println!("Licensed {0} files",count);
}
