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
    let mut total_characters = 0;

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


    let mut words: HashMap<&str, usize> = HashMap::new();
    for word in s.split(is_whitespace).filter(is_not_empty) {
    	total_characters += word.chars().count();
        if let Some(count) = words.get_mut(word) {
            *count += 1;
            continue;
        }
        words.insert(word, 1);
    }

    fn count(map: &mut HashMap<&str, usize>) -> usize{
        let iterator = map.iter();
        let counter = iterator.count();
        return counter;
    }



    fn average_wordsize(map: &mut HashMap<&str, usize>) -> usize {
        let mut tot_chars = 0;
        let count = count(map);
        for (key, value) in map {
            // println!("{} / {}", key, value);
            tot_chars += key.chars().count() * *value;
        }
        return tot_chars/count;
    }

    fn words_per_size(map: &mut HashMap<&str, usize>) -> HashMap<usize, usize> {
        let mut length_words: HashMap<usize, usize> = HashMap::new();
        for n in 1..10 {
            length_words.insert(n, 0);
        }
        for (key, value) in map {
            //println!("{} / {}", key, value);
            let _chars = key.chars().count();
            if let Some(char_count) = length_words.get_mut(&_chars) {
                *char_count += *value;
                continue;
                }
            length_words.insert(_chars,1);
        }
        return length_words;
    }

    fn top10(map: &mut HashMap<&str, usize>) {
        for (key, value) in &*map {
            println!("{} / {}", key, value);
        }
        //map.clear();
    }

    println!("amount of words: {}", count(&mut words));
    println!("average wordsize: {:?}", average_wordsize(&mut words));
    println!("words per size: {:?}", words_per_size(&mut words));

 

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