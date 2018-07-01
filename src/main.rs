use std::env;

fn main() {
    for argument in env::args().skip(1) {
        let len = argument.len();
        // If the string has odd length.
        if len & 1 == 1 {
            let mut beginning = argument;
            let mut middle = beginning.split_off(len / 2);
            let end = middle.split_off(1);
            println!("{}|{}|{}", beginning, middle, end);
        }
        // If the string has even length.
        else {
            let mut beginning = argument;
            let end = beginning.split_off(len / 2);
            println!("{}|{}", beginning, end);
        }
    }
}
