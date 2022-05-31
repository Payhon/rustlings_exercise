// strings2.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings2` for hints :)

// I AM NOT


fn main() {
    let word = String::from("green"); // Try not changing this line :)
    if is_a_color_word(&word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

/**
#[cfg(test)]
mod test{
    #[test]
    pub fn test_case() {
        let word = String::from("green"); // Try not changing this line :)
        if is_a_color_word(&word) {
            println!("That is a color word I know!");
        } else {
            println!("That is not a color word I know.");
        }
    }
}
**/

fn is_a_color_word(attempt: &String) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}
