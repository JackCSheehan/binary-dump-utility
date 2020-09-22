mod file_reader;
mod dump_util;
mod cmd_arg_parser;

use std::io;

fn main() -> Result<(), io::Error>
{
    // Get arguments from the command line
    let args = cmd_arg_parser::CommandLineArgs::from_args()?;
    
    // Get base based on user's given base flag
    let base = match &args.base_flag[..]
    {
        cmd_arg_parser::HEX_FLAG => dump_util::DisplayMode::Hex,
        cmd_arg_parser::BINARY_FLAG => dump_util::DisplayMode::Binary,
        cmd_arg_parser::DECIMAL_FLAG => dump_util::DisplayMode::Decimal,
        cmd_arg_parser::OCTAL_FLAG => dump_util::DisplayMode::Octal,
        cmd_arg_parser::ASCII_FLAG => dump_util::DisplayMode::ASCII,
        _ => dump_util::DisplayMode::NoneDefault,
    };

    // Get contents from file given by user
    let contents = file_reader::get_binary_data(&args.read_file_path);

    // If the user is dumping to file
    if args.destination_flag == cmd_arg_parser::FILE_FLAG
    {
        dump_util::write_dump(contents?, base, &args.write_file_path)?;
    }
    // If the user is dumping to console
    else
    {
        dump_util::display_dump(contents?, base);
    }

    Ok(())
}
