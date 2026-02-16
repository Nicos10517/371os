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

    let mut show_lines = false;
    let mut show_words = false;
    let mut show_bytes = false;
    let mut filename: Option<&String> = None;

    //Check arguments (what's typed in terminal)
    for arg in args.iter().skip(1) {
        if arg == "-l" {show_lines = true;continue}
        if arg == "-w" {show_words = true;continue}
        if arg == "-c" {show_bytes = true;continue}

        filename = Some(arg);
    }
    //find filename
    
    if !show_lines && !show_words && !show_bytes {
        show_lines = true;
        show_words = true;
        show_bytes = true;
    }
      
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
    //
    let target = filename.expect("Usage: cargo run -- [flags] <filename>");
    let file = File::open(target)?;
    let mut reader = BufReader::new(file);

    

    //Creating a reusable string buffer for each line
    //Reusing the same string memory faster than creating new one
    
    let mut line = String::new();
    let mut line_count = 0;
    let mut word_count = 0;
    let mut byte_count = 0;




    //Read the file line by line
    //It returns number of bytes read, 0 when it reaches end of file
    
    while reader.read_line(&mut line)? > 0 {
        line_count += 1;
        byte_count += line.len();
        word_count += line.split_whitespace().count();
        line.clear();
    }

    //In Rust, you can't just drop a variable directly into a string, you have to tell rust exactly
    //where data should go:
    //{:?} Prints something in "Debug Mode"
    //{:.2} Limits to 2 decimal Places
    //{:x} Prints a number in Hexadecimal
    //etc.

    let mut output = String::new();
    if show_lines {output.push_str(&format!("{}\t",line_count));}
    if show_words {output.push_str(&format!("{}\t", word_count));}
    if show_bytes {output.push_str(&format!("{}\t", byte_count));}
    println!("{}{}", output, target);

   Ok(())
}
