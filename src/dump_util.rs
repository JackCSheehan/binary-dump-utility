/*
File containing functions and procedures for writing and displaying the 
binary file dump
*/

use std::fs::File;
use std::io::prelude::*;
use std::io;

// Number of columns to display in file and in console
const COLUMNS: usize = 4;

/*
Enum used to indicate which display mode to use when writing to file or dumping
hex to console
*/
pub enum DisplayMode
{
    Hex,
    Binary,
    Octal,
    Decimal,
    ASCII,
    NoneDefault,    // Default value for no display mode
}

/*
Displays the given Vec<u8> of bytes to the console. Formats bytes such that a newline
is inserted after 4 consecutive bytes. Also takes a display mode and changes how data is 
displayed based on that given display mode.
*/
pub fn display_dump(bytes: Vec<u8>, display_mode: DisplayMode)
{
    // Iterate through each byte, print, and format it
    for (index, byte) in bytes.iter().enumerate()
    {
        // Determine how to display based on display mode
        match display_mode
        {
            DisplayMode::Hex => print!("{:0>2x} ", byte),
            DisplayMode::Binary => print!("{:0>8b} ", byte),
            DisplayMode::Octal => print!("{:0>3o} ", byte),
            DisplayMode::Decimal => print!("{:0>3} ", byte),
            DisplayMode::ASCII => print!("{} ", *byte as char),
            _ => (),
        };

        // Add newline after 4 bytes for nice formatting
        if (index + 1) % COLUMNS == 0 { println!(""); }
    }
}

/*
Writes the binary file dump to a local file. Takes a vector of bytes to write, a display mode,
and a string slice with the name of the file to create when writing
*/
pub fn write_dump(bytes: Vec<u8>, display_mode: DisplayMode, file_name: &str) -> Result<(), io::Error>
{
    // Create new file based on given file name
    let mut new_file = File::create(file_name)?;

    // Byte counter for writing loop
    let mut byte_count = 0;

    // Loop through bytes and write them to file
    while byte_count < bytes.len()
    {
        // Pick specific writing function based on display mode parameter
        match display_mode
        {
            DisplayMode::Hex => {write_hex(&mut new_file, bytes[byte_count], file_name)?; 0},
            DisplayMode::Binary => {write_binary(&mut new_file, bytes[byte_count], file_name)?; 0},
            DisplayMode::Octal => {write_octal(&mut new_file, bytes[byte_count], file_name)?; 0},
            DisplayMode::Decimal => {write_decimal(&mut new_file, bytes[byte_count], file_name)?; 0},
            DisplayMode::ASCII => {write_ascii(&mut new_file, bytes[byte_count], file_name)?; 0},
            _ => 0,
        };
        
        // Add newline after 4 bytes for nice formatting
        if (byte_count + 1) % COLUMNS == 0 
        { 
            new_file.write(b"\n")?;
        }
        byte_count += 1;
    }

    Ok(())  
}

/*
Helper method to write given byte to given file formatted a hex number
*/
fn write_hex(file: &mut File, byte: u8, file_name: &str) -> Result<(), io::Error>
{
    file.write_fmt(format_args!("{:0>2x} ", byte))?;

    Ok(())
}

/*
Helper method to write given byte to given file formatted a binary number
*/
fn write_binary(file: &mut File, byte: u8, file_name: &str) -> Result<(), io::Error>
{
    file.write_fmt(format_args!("{:0>8b} ", byte))?;

    Ok(())
}

/*
Helper method to write given byte to given file formatted an octal number
*/
fn write_octal(file: &mut File, byte: u8, file_name: &str) -> Result<(), io::Error>
{
    file.write_fmt(format_args!("{:0>3o} ", byte))?;

    Ok(())
}

/*
Helper method to write given byte to given file formatted a decimal number
*/
fn write_decimal(file: &mut File, byte: u8, file_name: &str) -> Result<(), io::Error>
{
    file.write_fmt(format_args!("{:0>3} ", byte))?;

    Ok(())
}

/*
Helper method to write given byte to given file formatted an ASCII character
*/
fn write_ascii(file: &mut File, byte: u8, file_name: &str) -> Result<(), io::Error>
{
    file.write_fmt(format_args!("{} ", byte as char))?;

    Ok(())
}