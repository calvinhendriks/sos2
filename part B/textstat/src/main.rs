use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;
use std::collections::HashMap;

fn main() {

	let mut avg_word_size = 0;
	let mut word_count = 0;
    // Get parameters from command line
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    // Create a path to the desired file
    let path = Path::new(filename);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut buffer = String::new();
    match file.read_to_string(&mut buffer) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.description()),
        Ok(_) => {
        	let mut words: HashMap<&str, usize> = HashMap::new();
    		for word in buffer.split_whitespace() {
        		if let Some(count) = words.get_mut(word){
        			*count += 1;
        			continue;
        		}
        		words.insert(word, 1);
        	}
        },
    }
}