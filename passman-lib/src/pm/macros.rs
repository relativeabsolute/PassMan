use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io;
use io::prelude::*;

// TODO: macros is a misnomer for this module, fix
pub fn write_all_to_file(file_path: &str, contents: &str) -> io::Result<()> {
    let output_path = Path::new(file_path);
    let display_path = output_path.display();

    match File::create(&output_path) {
        Err(why) => Err(why),
        Ok(mut file) => file.write_all(contents.as_bytes()) 
    }
}