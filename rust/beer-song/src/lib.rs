pub fn verse(index: usize) -> String {
    let mut first = String::new();
    let mut second = String::new();
    let third: &str;
    let mut last = String::new();

    match index {
        2 => {
            first.push_str("2 bottles");
            second = first.clone();
            third = "Take one down and pass it around, ";
            last.push_str("1 bottle of");
        }
        1 => {
            first.push_str("1 bottle");
            second = first.clone();
            third = "Take it down and pass it around, ";
            last.push_str("no more bottles of");
        }
        0 => {
            first.push_str("No more bottles");
            second.push_str("no more bottles");
            third = "Go to the store and buy some more, ";
            last.push_str("99 bottles of");
        }
        _ => {
            first = format!("{} bottles", index);
            second = first.clone();
            third = "Take one down and pass it around, ";
            last = format!("{} bottles of", index - 1);
        }
    };

    format!(
        "{} of beer on the wall, {} of beer.\n{}{} beer on the wall.\n",
        first,
        second,
        third,
        last
    )
}

pub fn sing(upper: usize, lower: usize) -> String {
    let mut result = String::new();

    for i in (lower..upper + 1).rev() {
        result = result + &verse(i);

        result.push('\n');
    }

    result.pop();

    result
}
