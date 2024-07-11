enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();

        for cmd in input {
            match cmd.1 {
                Command::Trim => result.push(String::from(cmd.0.trim())),
                Command::Uppercase => result.push(String::from(cmd.0.to_uppercase())),
                Command::Append(size) => {
                    let suffix = vec!["bar"; size].join("");
                    result.push(format!("{}{suffix}", cmd.0))
                }
            }
        }

        println!("{result:?}");

        result
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}
