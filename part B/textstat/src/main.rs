use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;
use std::collections::HashMap;

fn main() {

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
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.description()),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }

    s.replace(".", "");
    s.replace(",","");
    s.remove(".");



    let mut words: HashMap<&str, usize> = HashMap::new();
    for word in s.split(is_whitespace).filter(is_not_empty) {
        if let Some(count) = words.get_mut(word) {
            *count += 1;
            continue;
        }
        words.insert(word, 1);
    }



    let iterator = words.iter();
    let counter = iterator.count();

    let iterator2 = words.iter();
    let mut count_vec: Vec<_> = iterator2.collect();
    count_vec.sort_by(|a, b| b.1.cmp(&a.1));
    count_vec.truncate(10);


    println!("{}", counter);
    println!("{:?}",count_vec);

    #[inline]
    fn is_not_empty(s: &&str) -> bool {
        !s.is_empty()
    }

    #[inline]
    fn is_whitespace(c: char) -> bool {
        c == ' ' || c == '\t' || c == '\n'
    }

    // `file` goes out of scope, and the "hello.txt" file gets closed
}