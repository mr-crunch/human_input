mod human_input {
    use std::{
        io::{self, Write},
        str::FromStr,
    };

    pub fn read_string(prompt: &str) -> io::Result<Option<String>> {
        print!("{}", prompt);
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(if input.trim() == "" {
            None
        } else {
            Some(input.trim().to_string())
        })
    }

    pub fn read_string_checked(prompt: &str) -> io::Result<String> {
        loop {
            match read_string(prompt)? {
                Some(input) => return Ok(input),
                None => println!("input cannot be empty"),
            }
        }
    }

    pub fn read_typed_checked<T: FromStr>(prompt: &str) -> io::Result<T> {
        loop {
            let untyped_input = read_string_checked(prompt)?;
            match untyped_input.parse::<T>() {
                Ok(input) => return Ok(input),
                Err(_) => println!("{} is not valid", untyped_input),
            }
        }
    }

    pub fn read_menu(
        prompt: &str,
        options: &[impl AsRef<str>],
        default: Option<usize>,
    ) -> io::Result<usize> {
        assert!(
            if let Some(num) = default {
                num < options.len()
            } else {
                true
            },
            "default index must be in options slice"
        );
        loop {
            print!("{} [", prompt);
            let mut options_iter = options.iter().enumerate();
            if let Some((_, option)) = options_iter.next() {
                print!(r#"1: "{}""#, option.as_ref());
            }
            for (idx, option) in options_iter {
                print!(r#", {}: "{}""#, idx + 1, option.as_ref());
            }
            if let Some(def) = default {
                print!(" (default: {})", def + 1);
            }
            print!("]: ");
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let choice = input.trim();
            if let Some(val) = default {
                if choice.is_empty() {
                    return Ok(val);
                }
            }
            match choice.parse::<usize>() {
                Ok(a) => {
                    if a == 0 {
                        println!("0 is not a valid option");
                    } else {
                        let choice = a;
                        if (choice - 1) < options.len() {
                            return Ok(choice);
                        } else {
                            println!("{} is not a valid option (too big)", choice);
                        }
                    }
                }
                Err(_) => {
                    println!("{} is not a valid option", choice);
                }
            }
        }
    }
}

fn main() {
    println!("Hello world!");
    println!("input: {}", input());
    match human_input::read_menu(
        "enter choice: ",
        &["new bill", "list bills", "print month", "list year"],
        Some(1),
    ) {
        Ok(num) => println!("choice: {}", num),
        Err(error) => eprintln!("error: {:?}", error),
    }
}

fn input() -> String {
    match human_input::read_string_checked("enter name: ") {
        Ok(string) => string,
        Err(error) => {
            eprintln!("error: {:?}", error);
            String::from("error")
        }
    }
}
