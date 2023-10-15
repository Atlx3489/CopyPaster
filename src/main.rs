use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn main() {

    let mut pathinput = PathBuf::new();

    let mut pathoutput = PathBuf::new();

    println!("Input Target Copy Directory:");




    let paths = fs::read_dir(pathinput.clone()).unwrap();

    println!("Files Copied to {}:", pathoutput.clone().display());

    for path in paths {
        println!("Name: {}", path.unwrap().path().display());
        fs::copy(&mut pathinput, &mut pathoutput);
    }


}