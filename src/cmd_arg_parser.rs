/*
File containing associate methods for dealing with and parsing
command line args
*/

use std::env;

// Constants for command line flags and arguments
pub const CONSOLE_FLAG: &str = "-c";    // Indicates that dump should be displayed to console
pub const FILE_FLAG: &str = "-f";       // Indicates that dump should be written to file
pub const HEX_FLAG: &str = "-h";        // Dump as hex
pub const BINARY_FLAG: &str = "-b";     // Dump as binary
pub const DECIMAL_FLAG: &str = "-d";    // Dump as decimal
pub const OCTAL_FLAG: &str = "-o";      // Dump as octal
pub const ASCII_FLAG: &str = "-a";      // Dump as ASCII

// Constants for flag positions in command line vectors
const DESTINATION_FLAG_INDEX: usize = 1;
const BASE_FLAG_INDEX: usize = 2;
const READ_FILE_INDEX: usize = 3;
const WRITE_FILE_INDEX: usize = 4;

// Constants for expected arg counts
const CONSOLE_EXPECTED_ARGS: usize = 4; // Total number of expected args for dumping to console
const FILE_EXPECTED_ARGS: usize = 5;    // Total number of expected args for dumping to file

//dump-util -c -h hello.txt
//dump-util -f -o hello.txt output.txt
/*
Structure used for storing and organizing command line arguments
*/
pub struct CommandLineArgs
{
    pub destination_flag: String,   // Where to dump file contents
    pub base_flag: String,          // What base to dump file contents in
    pub read_file_path: String,     // File to read path, if needed
    pub write_file_path: String,    // File to write path, if needed
}

impl CommandLineArgs
{
    /*
    Constructs and returns a new CommandLineArgs struct based on args passed to
    the command line
    */
    pub fn from_args() -> Self
    {
        // Get args from command line
        let args: Vec<String> = env::args().collect();

        // Error checking for minimum required size
        if args.len() < CONSOLE_EXPECTED_ARGS
        {
            panic!("ERROR: Expected at least {} arguments", CONSOLE_EXPECTED_ARGS - 1);
        }
        
        // Get flags based on expected indexes
        let destination_flag = args[DESTINATION_FLAG_INDEX].clone();
        let base_flag = args[BASE_FLAG_INDEX].clone();

        // Error checking to ensure file arg is provided to file dump command
        if destination_flag == FILE_FLAG && args.len() < FILE_EXPECTED_ARGS 
        {
            panic!("ERROR: Too few arguments for file dump. Need a file name to write to");
        }

        let read_file_path;
        let write_file_path;

        // Get the file to read from
        read_file_path = args[READ_FILE_INDEX].clone();

        // Get write file path if dumping to file
        if destination_flag == FILE_FLAG
        {
            write_file_path = args[WRITE_FILE_INDEX].clone();
        }
        // Get source file if dumping
        else
        {
            // If dumping to CMD, provide blank string to write path
            write_file_path = String::new();
        };

        Self
        {
            destination_flag,
            base_flag,
            read_file_path,
            write_file_path
        }
    }
}