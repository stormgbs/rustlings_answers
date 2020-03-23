// test4.rs
// This test covers the sections:
// - Modules
// - Macros

// Write a macro that passes the test! No hints this time, you can do it!

macro_rules! my_macro {
    ( $( $x:expr ), * ) => {
        {
            let mut msg = "Hello".to_string();
            $(
                msg.push_str(" ");
                msg.push_str($x);
            )*
            msg
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

    #[test]
    fn test_my_macro_jiang_bo() {
        assert_eq!(my_macro!("jiang", "bo"), "Hello jiang bo");
    }
}
