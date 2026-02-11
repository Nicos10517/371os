use std::env; //CLI arguments :: is path separator, std Standard Library
use std::fs::File; //Helps us open files
use std::io::{self, BufRead, BufReader};

//BufRead- adds buffering by managing calls to the std::io::Read trait methods
//Reads a large chuck of data into memory at once and serves smaller requests for that memory

//io::Result<()> tells rust that main might fail, returns empty if everything is fine and Input or
//Output Error if not


// :: Path Separator-- Navigates through libraries and folders
// ! Macro-- Function that that does special compile-time work (like println!)
// ? Try Operator-- Unwraps result or stops program on error
// &mut Mutable Borrow-- Lends data to function so the function can modify
// () Unit Type -- "nothing" or empty String

fn main() -> io::Result<()> {
    //Turns all arguments into vector, not iterator
    //Makes it easier to check length
    let args: Vec<String> = env::args().collect();

    //Check arguments (what's typed in terminal)
    if args.len() <2 {
        println!("Usage: {} [-l] <filename>", args[0]);
        return Ok(());
    }

    let lines_only = args.contains(&"-l".to_string());

    //find filename
    let filename = &args[args.len()-1];

    /*
    //Reads the file name
    //Nth(1) to skip the program name
    //None, if no file name given
    //In rust, args is an iterator

    let filename = match env::args().nth(1) {
        Some(name) => name,
        None => return Ok(()),
    };

    //Question Mark means check and return
    //If it returns an error, the question mark immediately stops main and passes error to the
    //terminal-- shorthand for if-else block, very useful rust 0.0
    */
    
    //Open File and read it w/BufReader, prevents too many system calls

    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);

    let mut line_count = 0;
    let mut word_count = 0;
    let mut byte_count = 0;

    

    //Creating a reusable string buffer for each line
    //Reusing the same string memory faster than creating new one
    
    let mut line = String::new();



    //Read the file line by line
    //It returns number of bytes read, 0 when it reaches end of file
    
    while reader.read_line(&mut line)? > 0 {
        line_count += 1;
        
        if !lines_only {
            byte_count += line.len()+1;
            word_count += line.split_whitespace().count();
        }
        line.clear();
    }

    //In Rust, you can't just drop a variable directly into a string, you have to tell rust exactly
    //where data should go:
    //{:?} Prints something in "Debug Mode"
    //{:.2} Limits to 2 decimal Places
    //{:x} Prints a number in Hexadecimal
    //etc.

    if lines_only {
        println!("{}\t{}", line_count, filename);
    } else {
        println!("{}\t{}\t{}\t{}", line_count, word_count, byte_count, filename);
    }

   Ok(())
}
