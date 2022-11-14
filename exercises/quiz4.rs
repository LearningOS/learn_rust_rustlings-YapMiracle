// quiz4.rs
// This quiz covers the sections:
// - Modules
// - Macros

// Write a macro that passes the quiz! No hints this time, you can do it!

// I AM NOT DONE
macro_rules! my_macro{
    ($a:expr)=>{
        let mut a = "Hello ";
        let mut b = $a.to_string();
        return a+b
    }
}
// macro_rules! add{
//     // macth like arm for macro
//        ($a:expr,$b:expr)=>{
//     // macro expand to this code
//            {
//    // $a and $b will be templated using the value/variable provided to macro
//                $a+$b
//            }
//        }
//    }
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
