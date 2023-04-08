fn main() {
    let mut s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
    change(&mut s1);
    println!("The new string is '{}'.", s1);

    let mut s = String::from("hello");
    let r1 = &s; // no problem
    // let r_test = &mut s; // problem - cannot borrow as mutable at this time
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    // s.push("a"); // Error: cannot borrow as mutable
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}