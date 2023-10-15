use std::fs;
use std::fs::read_to_string;
use std::io;
use std::path::{Path, PathBuf};

fn getio() -> PathBuf {
    let mut pathio = String::new();
    io::stdin()
        .read_line(&mut pathio)
        .expect("Failed to read Path str");
    return PathBuf::from(pathio);
}

fn main() {
    let mut pathinput = PathBuf::new();

    println!("Give Me an input Directory");
    let mut pathinput = getio();
    println!("Input Directory {}", pathinput.display());


    //let paths = fs::read_dir("./").unwrap();
    //
    //for path in paths{
    //    println!("Name: {}", path.unwrap().path().display())
    //}
}