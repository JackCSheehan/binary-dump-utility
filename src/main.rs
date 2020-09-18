mod file_reader;
mod dump_util;

fn main()
{
    // Get the file's contents
    let contents = file_reader::get_binary_data("Resume-LL 9-16-20.pdf");
    
    dump_util::write_dump(contents, dump_util::DisplayMode::Octal, "test.txt");
    
}
