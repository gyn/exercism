pub fn raindrops(number: usize) -> String {
    let mut result = String::new();

    if number % 3 == 0 {
        result += "Pling";
    }

    if number % 5 == 0 {
        result += "Plang";
    }

    if number % 7 == 0 {
        result += "Plong";
    }

    if result.is_empty() {
        return number.to_string();
    }

    result
}
