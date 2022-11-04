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
                    let mut string = string.to_string();
                    // string = string.replace("a","A");
                    // string = string.replace("b","B");
                    // string = string.replace("c","C");
                    // string = string.replace("d","D");
                    // string = string.replace("e","E");
                    // string = string.replace("f","F");
                    // string = string.replace("g","G");
                    // string = string.replace("h","H");
                    // string = string.replace("i","I");
                    // string = string.replace("j","J");
                    // string = string.replace("k","K");
                    // string = string.replace("l","L");
                    // string = string.replace("m","M");
                    // string = string.replace("n","N");
                    // string = string.replace("o","O");
                    // string = string.replace("p","P");
                    // string = string.replace("q","Q");
                    // string = string.replace("r","R");
                    // string = string.replace("s","S");
                    // string = string.replace("t","T");
                    // string = string.replace("u","U");
                    // string = string.replace("v","V");
                    // string = string.replace("w","W");
                    // string = string.replace("x","X");
                    // string = string.replace("y","Y");
                    // string = string.replace("z","Z");
                    string = string.to_uppercase();
                    output.push(string.to_string())

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
                    for _ in 0..(*s){
                        string.push_str("bar");
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
    it_works();
    
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
