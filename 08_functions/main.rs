use std::{fs::File, io::ErrorKind};

fn main() {
    println!("{}", return_greater(1, 2));

    let mut original = String::from("initial value");
    println!("{}", original);
    print_var(&original);
    println!("Can use original again: {}", original); // original holds the value again as "print_original" returned and the vars inside its scope got destroyed
    change_var(&mut original);
    println!("Has original changed? Let's see: {}", original);

    // closures
    let thanos = String::from("I am");
    let closure = |s: &String| -> String { format!("{}... inevitable!", s) };
    println!("{}", closure(&thanos));

    // errors
    // panic!("panic"); // this is how you panic
    // panic_vec() // this is how you panic pt.2: "index out of bounds"

    // error handling with Result Enum
    let path = "/impossiblethatthispathexists/i/do/not/exist";
    match File::open(path) {
        Ok(f) => {
            println!("{:#?}", f);
        }
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create(path) { // trying to recovering the error by creating the file
                Ok(fc) => {
                    println!("{:#?}", fc);
                }
                Err(e) => {
                    println!("cannot create file because: {:#?}", e);
                }
            },
            _ => {
                println!("cannot open file because: {:#?}", err);
            }
        },
    }
}

// pass by value
fn return_greater(n1: u8, n2: u8) -> u8 {
    if n1 > n2 {
        return n1;
    }
    n2
}

// pass by reference - borrowing the value from the parameter passed in
fn print_var(s: &String) {
    println!("fn prints {}", s);
}

// pass by value - if this is called then "original" in main() can't be used anymore, as the ownership got transferred to "s" (fn param)
/* fn print_original(s: String) {
    println!("fn prints {}", s);
} */

// pass by reference - borrowing for read and write (mutability)
fn change_var(s: &mut String) {
    *s = String::from("I have become death, destroyer of worlds");
}

fn _panic_vec() {
    let v = vec![1, 2];
    println!("{}", v[5]);
}
