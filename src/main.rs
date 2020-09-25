/*
Command line tool for dumping file contents in various bases to either the console
or a file.
*/
mod file_reader;
mod dump_utils;
mod cmd_arg_parser;

use std::process;

fn main()
{
    // Get arguments from the command line and validate input
    let args = match cmd_arg_parser::CommandLineArgs::from_args()
    {
        Ok(valid_args) => valid_args,
        Err(_) => { println!("ERROR: Too few arguments in call"); process::exit(1); }
    };
    
    // Get base based on user's given base flag
    let base = match &args.base_flag[..]
    {
        cmd_arg_parser::HEX_FLAG => dump_utils::DisplayMode::Hex,
        cmd_arg_parser::BINARY_FLAG => dump_utils::DisplayMode::Binary,
        cmd_arg_parser::DECIMAL_FLAG => dump_utils::DisplayMode::Decimal,
        cmd_arg_parser::OCTAL_FLAG => dump_utils::DisplayMode::Octal,
        cmd_arg_parser::ASCII_FLAG => dump_utils::DisplayMode::ASCII,
        _ => dump_utils::DisplayMode::NoneDefault,
    };

    // Get contents from file given by user
    let contents = match file_reader::get_binary_data(&args.read_file_path)
    {
        Ok(c) => c,
        Err(_) => { println!("ERROR: Could not read data from file"); process::exit(1); }
    };

    // If the user is dumping to file
    if args.destination_flag == cmd_arg_parser::FILE_FLAG
    {
        // Write file contents; check to make sure write was successful
        match dump_utils::write_dump(contents, base, &args.write_file_path)
        {
            Ok(_) => (),
            Err(_) => { println!("ERROR: Could not write data to file"); process::exit(1); }
        }
    }
    // If the user is dumping to console
    else
    {
        dump_utils::display_dump(contents, base);
    }
}
