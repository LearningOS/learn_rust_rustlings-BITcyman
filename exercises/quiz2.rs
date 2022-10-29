// quiz2.rs
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums

// Let's build a little machine in form of a function.
// As input, we're going to give a list of strings and commands. These commands
// determine what action is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
// Execute `rustlings hint quiz2` or use the `hint` watch subcommand for a hint.


pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complete the function signature!
    pub fn transformer(input: Vec<(String,Command)>) -> Vec<String> {
        // TODO: Complete the output declaration!
        let mut output: Vec<String> = vec![];
        for (string, command) in input.iter() {
            match command{
                Command::Uppercase => {
                    string.replace("a","A");
                    string.replace("b","B");
                    string.replace("c","C");
                    string.replace("d","D");
                    string.replace("e","E");
                    string.replace("f","F");
                    string.replace("g","G");
                    string.replace("h","H");
                    string.replace("i","I");
                    string.replace("j","J");
                    string.replace("k","K");
                    string.replace("l","L");
                    string.replace("m","M");
                    string.replace("n","N");
                    string.replace("o","O");
                    string.replace("p","P");
                    string.replace("q","Q");
                    string.replace("r","R");
                    string.replace("s","S");
                    string.replace("t","T");
                    string.replace("u","U");
                    string.replace("v","V");
                    string.replace("w","W");
                    string.replace("x","X");
                    string.replace("y","Y");
                    string.replace("z","Z");


                },
                Command::Trim => {
                    let mut string = string.to_string();
                    string.remove(0);
                    let len = string.len();
                    string.remove(len-1);
                    output.push(string)
                },
                Command::Append(s)=>{
                    let s0 = string;
                    let mut string = string.to_string();
                    for _ in 0..*s{
                        string.push_str(s0);
                    }
                    output.push(string)
                },
            }
        }
        output
    }
}

fn main() {
    
    use my_module::transformer;

    
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
