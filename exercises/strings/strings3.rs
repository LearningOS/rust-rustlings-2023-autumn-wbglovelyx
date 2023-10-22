// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.

fn trim_me(input: &str) -> String {
    let trimmed = input.trim(); // This removes leading and trailing whitespace
    trimmed.to_string()
}

fn compose_me(input: &str) -> String {
    let composed = format!("{} world!", input); // Compose the new string
    composed
}

fn replace_me(input: &str) -> String {
    let replaced = input.replace("cars", "balloons"); // Replace "cars" with "balloons"
    replaced
}
