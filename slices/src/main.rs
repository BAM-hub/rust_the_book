fn main() {
    let s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    // err 
    //  clear usess a mut and truncates the string 
    // which means we cant use a refrence to it later
    // s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
    println!("the first word is: {}", word);
    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&s[0..6]);
    let word = first_word(&s[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&s);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    print!("{}",word);
}


fn first_word (s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

