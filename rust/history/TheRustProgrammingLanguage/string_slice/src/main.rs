fn main() {
    let s = String::from("Hello World");
    let word = first_word(&s);
    println!("{}", word);
}

fn first_word(string: &str) -> &str {
    for (index, &item) in string.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &string[..index];
        }
    }
    string
}
