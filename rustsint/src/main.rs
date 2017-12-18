use std::io::{self, Write};
mod sorted_container;

#[derive(Debug)]
enum Command {
    Insert{age: i32, name: String},
    Erase{age: i32, name: String},
    Contains{age: i32, name: String},
    Print,
    Exit,
    Error(String)
}

fn parse_command(input: String) -> Command {
    let command_items: Vec<&str> = input.split_whitespace().collect();
    match (command_items[0], command_items.len()) {
        ("p", 1) => Command::Print,
        ("x", 1) => Command::Exit,
        ("i", 3) => {
            if let Ok(age) = command_items[1].parse::<i32>() {
                Command::Insert{age: age, name: command_items[2].to_string()}
            } else {
                Command::Error("unable to parse int (age).".to_string())
            }
        },
        ("e", 3) => {
            if let Ok(age) = command_items[1].parse::<i32>() {
                Command::Erase{age: age, name: command_items[2].to_string()}
            } else {
                Command::Error("unable to parse int (age).".to_string())
            }
        },
        ("c", 3) => {
            if let Ok(age) = command_items[1].parse::<i32>() {
                Command::Contains{age: age, name: command_items[2].to_string()}
            } else {
                Command::Error("unable to parse int (age).".to_string())
            }
        },

        (_, _) => Command::Error("invalid command.".to_string())
    }
}

#[cfg(test)]
#[test]
fn test1() {
    let t = sorted_container::Tree::new();
    assert_eq!(t.root.is_some(), false, "Root of tree is not empty!");
}
#[test]
fn test2() {
    let mut t = sorted_container::Tree::new();
    assert_eq!(t.root.is_some(), false, "Root of tree is not empty!");
    t.insert(sorted_container::Data::new(10, "aap".to_string()));
    if let Some(ref root) = t.root {
        assert_eq!(root.data.age, 10, "Data is not equivalent");
        assert_eq!(root.data.name, "aap".to_string(), "Data is not equivalent");
        assert_eq!(root.left.is_some(), false, "Left node of root is not null");
        assert_eq!(root.right.is_some(), false, "Right node of root is not null");
    }

    let dif_data = sorted_container::Data::new(20, "noot".to_string());
    assert_eq!(t.contains(dif_data), false, "Data should not be in the container");

    // ERASE IS NOT IMPLEMENTED

}
#[test]
fn test3() {
    // ERASE IS NOT IMPLEMENTED
}
#[test]
fn test4() {
    // ERASE IS NOT IMPLEMENTED
}
#[test]
fn test5() {
    // ERASE IS NOT IMPLEMENTED
}

fn main() {
	let mut t = sorted_container::Tree::new();
    loop {
        let mut input = String::new();

        print!("> ");
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match parse_command(input) {
                    Command::Insert{age, name} => {
                        t.insert(sorted_container::Data::new(age, name));
                    },
                    Command::Erase{age, name} => {
                        unimplemented!();
                    },
                    Command::Contains{age, name} => {
                        match t.contains(sorted_container::Data::new(age, name)) {
                        	true => {
                        		println!("y\n");
                        	},
                        	false => {
                        		println!("n\n");
                        	}
                        }
                    }
                    Command::Print => {
                        t.print_tree();
                    },
                    Command::Exit => {
                        println!("Exiting...");
                        break;
                    },
                    Command::Error(error) => {
                        println!("Error: {}", error);
                    }
                }
            }
            Err(error) => println!("Error: {}", error),
        }
    }
}
