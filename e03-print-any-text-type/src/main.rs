use std::ffi::CString;

fn info<T: std::fmt::Display>(a: &T) {
    println!("{}", a);
}

fn main() {
    let a: &str = "?";
    let b: String = "?".to_string();
    info(&a);
    info(&b);
}

#[test]
fn str() {
    let input: &str = "Rust";
    info(&input);
}

#[test]
fn string() {
    let input = String::from("Rust");
    info(&input);
}

#[test]
fn chars() {
    let input = 'r';
    info(&input);
}

// #[test]
// fn cstring() {
//     use std::ffi::CString;
//     let input = CString::new("Rust").unwrap();
//     info(&input);
// }

// #[test]
// fn path() {
//     use std::path::Path;
//     let input = Path::new("/tmp/rust");
//     info(input);
// }


