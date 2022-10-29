// strings3.rs
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a hint.




fn trim_me(input: &str) -> String {
     
    let mut s = input.to_string();
    while s.chars().nth(0)==Some(' ') {
        s.remove(0);
    }

    let mut len = s.len();
    while len>0 && s.chars().nth(len-1)==Some(' '){
        s.remove(len-1);
        len = s.len();
    }
    s
}

fn compose_me(input: &str) -> String {
    let mut s = input.to_string();
    s.push_str(" world!");
    s
}

fn replace_me(input: &str) -> String {
    let mut s = input.to_string();
    s = s.replace("cars","balloons");
    s
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
