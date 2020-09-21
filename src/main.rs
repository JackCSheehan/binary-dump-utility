mod file_reader;
mod dump_util;
mod cmd_arg_parser;

fn main()
{
    // Get the file's contents
    //let contents = file_reader::get_binary_data("Resume-LL 9-16-20.pdf");
    
    //dump_util::write_dump(contents, dump_util::DisplayMode::Octal, "test.txt");
    
    // Get arguments from the command line
    let args = cmd_arg_parser::CommandLineArgs::from_args();

    // Get the user's base preference from the command line args
    let base = args.base_flag;
    
    // If the user is dumping to file
    if args.destination_flag == cmd_arg_parser::FILE_FLAG
    {
        let contents = file_reader::get_binary_data(&args.read_file_path);
        dump_util::write_dump(contents, dump_util::DisplayMode::Decimal, &args.write_file_path)
    }
}
