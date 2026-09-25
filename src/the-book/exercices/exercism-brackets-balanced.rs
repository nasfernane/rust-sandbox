fn main() {
    pub fn brackets_are_balanced(string: &str) -> bool {
        let mut openings = String::new();

        for char in string.chars() {
            match char {
              
                '{' | '[' | '(' => {
                    openings.push(char);
                }
                ')' => match openings.pop() {
                    Some('(') => {}
                    _ => return false,
                },
                '}' => match openings.pop() {
                    Some('{') => {}
                    _ => return false,
                },
                ']' => match openings.pop() {
                    Some('[') => {}
                    _ => return false,
                },
                _ => continue,
            };
        }

        openings.is_empty()
    }

    let balanced_string = "{what is (42)}?";
    // let unbalanced_string = "[text}";

    println!(
        "Is string balanced ? {}",
        brackets_are_balanced(balanced_string)
    );
    // brackets_are_balanced(unbalanced_string);
}
