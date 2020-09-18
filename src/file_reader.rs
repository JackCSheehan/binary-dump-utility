/*
Module for reading and writing file info
*/

use std::fs::File;
use std::io::prelude::*;

/*
This function gets the binary data from a file at the given path. Returns the
bytes from the file as a Vec<u8>
*/
pub fn get_binary_data(path_str: &str) -> Vec<u8>
{
    // Open binary file and check that it exists
    let mut binary_file = match File::open(path_str)
    {
        Err(_) => panic!("Error! File {} does not exist!", path_str),
        Ok(binary_file) => binary_file,
    };

    // Contents of binary file
    let mut contents = Vec::new();

    // Read binary file contents
    match binary_file.read_to_end(&mut contents)
    {
        Err(_) => panic!("Could not read from file {}", path_str),
        Ok(_) => (),
    };
    
    // Return vector of bytes from file
    contents
}