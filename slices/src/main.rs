fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    // s.clear(); // Causes an error because violates mutates when we have immutable.

    println!("the first word is: {}", word);

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}


fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}