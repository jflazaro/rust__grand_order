#![warn(clippy::all, clippy::pedantic)]

use std::io::stdin;

#[derive(Debug)]
struct Visitor {
    name: [String; 2],
    greeting: String
}

impl Visitor {
    fn new(name: [&str; 2], greeting: &str) -> Self {
        Self {
            name: [name[0].to_string(), name[1].to_string()],
            greeting: greeting.to_string()
        }
    }

    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}

fn get_name() -> String {
    let mut your_name = String::new();

    // stdin() returns an object granting access to the Standard Input.
    // &mut: "Borrow" the variable, allowing changes to be made to it by the called function.
    // .read_line() is a method from the Stdin object.
    // .expect(...): "Unwrap" a Result object, and terminate the program with the message if an
    // error occurs.
    stdin()
        .read_line(&mut your_name)
        .expect("What??");

    your_name
        .trim()
        .to_string()
}

fn main() {
    let mut visitor_list = vec![
        Visitor::new(["altria", "altria pendragon"], "Welcome back, Seiba! :)"),
        Visitor::new(["mash", "mash kyrielight"], "MASHU!! :D"),
        Visitor::new(["josh", "josh"], "Welcome, Master Josh."),
        Visitor::new(["altria ruler", "bunny altria"], "OMG, WELCOME TO CHALDEA, Bunny Altria!!! :O"),
        Visitor::new(["bunny astolfo", "bunny astolfo"], "OMG, WELCOME TO CHALDEA, Astolfo-chan!!! :O"),
        Visitor::new(["space ishtar", "spishtar"], "OMG, WELCOME TO CHALDEA, Space Ishtar!!! :O")
    ];

    loop {
        println!("Hello, what's your name?");
        
        let input_name = get_name();
        let lower_case_name = input_name.to_lowercase();

        let known_visitor = visitor_list
            .iter()
            .find(|visitor| visitor.name[0] == lower_case_name || visitor.name[1] == lower_case_name); 
            // find() takes a closure as input. If the closure returns true, then find() returns the matching visitor.

        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if input_name.is_empty() {
                    break;
                } else {
                    println!("Hmm, hope you're ready to farm embers, {}! >:D", input_name);
                    visitor_list.push(Visitor::new([&lower_case_name, &lower_case_name], "Welcome to the Grind Team! >:D"));
                }
            }
        }
    }

    println!("The final list of visitors:");
    println!("{:#?}", visitor_list); // {:$?} = pretty print, {:?} = raw print
}
