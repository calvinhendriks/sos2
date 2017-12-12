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



    fn average_wordsize(map: &mut HashMap<&str, usize>) {
        for (key, value) in &*map {
            println!("{} / {}", key, value);
        }
        //map.clear();
    }

    fn words_per_size(map: &mut HashMap<&str, usize>) {
        for (key, value) in &*map {
            println!("{} / {}", key, value);
        }
        //map.clear();
    }

    fn top10<'a>(map: &'a mut HashMap<&str, usize>) -> Vec<(&'a&'a str, &'a usize)>  {
        let iterator2 = map.iter();
        let mut count_vec: Vec<_> = iterator2.collect();
        count_vec.sort_by(|a, b| b.1.cmp(&a.1));
        count_vec.truncate(10);
        return count_vec;
        //map.clear();
    }

    println!("amount of words: {}", count(&mut words));
    println!("amount of words: {:?}", average_wordsize(&mut words));

    println!("top 10: {:?}", top10(&mut words));



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