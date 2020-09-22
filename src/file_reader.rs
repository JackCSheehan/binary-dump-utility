/*
Module for reading binary file info
*/

use std::fs::File;
use std::io::prelude::*;
use std::io;

/*
This function gets the binary data from a file at the given path. Returns the
bytes from the file as a Vec<u8>
*/
pub fn get_binary_data(path_str: &str) -> Result<Vec<u8>, io::Error>
{
    // Open binary file and check that it exists
    let mut binary_file = File::open(path_str)?;

    // Contents of binary file
    let mut contents = Vec::new();

    // Read binary file contents
    binary_file.read_to_end(&mut contents)?;
    
    // Return vector of bytes from file
    Ok(contents)
}