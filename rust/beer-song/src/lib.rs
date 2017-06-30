pub fn verse(index: usize) -> String {
    match index {
        2 => {
            "2 bottles of beer on the wall, 2 bottles of beer.\n\
             Take one down and pass it around, 1 bottle of beer \
             on the wall.\n"
                .to_string()
        }
        1 => {
            "1 bottle of beer on the wall, 1 bottle of beer.\n\
             Take it down and pass it around, no more bottles of beer \
             on the wall.\n"
                .to_string()
        }
        0 => {
            "No more bottles of beer on the wall, no more bottles of beer.\n\
             Go to the store and buy some more, 99 bottles of beer \
             on the wall.\n"
                .to_string()
        }
        _ => {
            format!(
                "{} bottles of beer on the wall, {} bottles of beer.\n\
                 Take one down and pass it around, {} bottles of beer \
                 on the wall.\n",
                index,
                index,
                index - 1
            )
        }
    }
}

pub fn sing(upper: usize, lower: usize) -> String {
    (lower..upper + 1)
        .rev()
        .map(verse)
        .collect::<Vec<_>>()
        .join("\n")
}
