pub fn abbreviate<S>(name: S) -> String where S: Into<String> {
    let mut next_capital = true;
    let mut prev_lowercase = false;

    name.into()
        .chars()
        .flat_map(|x| match x {
            _ if next_capital => {
                next_capital = false;
                Some(x.to_uppercase().to_string())
            }
            _ if x == '-' => {
                next_capital = true;
                None
            }
            _ if x.is_whitespace() => {
                next_capital = true;
                None
            }
            _ if x.is_uppercase() && !next_capital && prev_lowercase => {
                prev_lowercase = false;
                Some(x.to_uppercase().to_string())
            }
            _ if x.is_lowercase() && !next_capital => {
                prev_lowercase = true;
                None
            }
            _ => None,
        })
        .collect()
}
