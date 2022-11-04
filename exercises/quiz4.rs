// quiz4.rs
// This quiz covers the sections:
// - Modules
// - Macros

// Write a macro that passes the quiz! No hints this time, you can do it!
// #[proc_macro]
// pub fn my_macro(s: &str) -> &str{
//     let mut hello = "Hello ".to_string();
//     hello.push_str(s);
//     &hello
// }

macro_rules! my_macro{
    ( $( $x:expr ),* ) => {
        {
            let mut temp_str = "Hello ".to_string();
            $(
                temp_str.push_str($x);
            )*
            temp_str
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_macro_world() {
        assert_eq!(my_macro!("world!"), "Hello world!");
    }

    #[test]
    fn test_my_macro_goodbye() {
        assert_eq!(my_macro!("goodbye!"), "Hello goodbye!");
    }
}
