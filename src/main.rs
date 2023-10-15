use std::fs;
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

    println!("Give Me an input Directory:");
    let path_input = getio();
    println!("Input Directory {}", path_input.display());

    println!("Give Me an output Directory:");
    let path_output = getio();
    println!("Output Directory {}", path_output.display());

    fs::copy(path_input, path_output);


    //let paths = fs::read_dir(path_input).unwrap();
//
   // for path in paths{
   //     println!("Name: {}", path.unwrap().path().display())
    //}
}