pub fn reverse(input: &str) -> String {
    let mut reversed_string = String::new();
    for letter in input.chars().rev() {
        reversed_string.push(letter);
    }

    reversed_string
}
