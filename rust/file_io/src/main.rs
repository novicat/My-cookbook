use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


static TEXT: &'static str =
"hello,
mult-line text,
blah";

fn main() {

    let filepath = Path::new("test.txt");
    
    let mut file = match File::create(&filepath) {
        Err(why) => panic!("Couldn't create file: {}",
                            why.to_string()),
        Ok(file) => file,
    };

    // Or write_all(b"string as bytes")
    match file.write_all(TEXT.as_bytes()) {
        Err(why) => panic!("Couldn't write: {}", why.to_string()),
        Ok(_) => (),
    };


    let contents = fs::read_to_string("test.txt")
        .expect("Something went wrong reading the file");
        
    println!("{}", contents);
}
