fn main() {
    // let s1 = String::from("hello");
    // let len = calculate_length(&s1);
    // println!("The length of '{s1}' is {len}.");

    // // Mutable Reference (at most one)
    // let mut s = String::from("hello");
    // change(&mut s);
    // println!("s is: '{s}'.");

    // // Dangling Pointer
    // let reference_to_nothing = dangle();

    // // Slice 
    // let mut s = String::from("hello world");
    // let word = first_word(&s);
    // s.clear();
    // // s is then set to "", and the index value stored in `word` would not make any sense
    // println!("{}", word);

    let my_string = String::from("hello world");
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    let word = first_word(&my_string);
    let my_string_literal = "hello world";
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);
    let word = first_word(my_string_literal);
}

// Returns a byte index vlue into the String parameter
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    // enumerate() method returns a tuple
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}


// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// fn dangle() -> &String {
//     let s = String::from("hello");
//     // Won't work, the owner will be dropped
//     &s
// }
