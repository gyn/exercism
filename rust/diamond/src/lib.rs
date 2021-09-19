pub fn get_diamond(c: char) -> Vec<String> {
    let middle = c as usize - 'A' as usize;
    let width = 2 * middle + 1;

    (0..width)
        .map(|i| {
            let mut line = vec![b' '; width];

            let offset = if i < middle { i } else { 2 * middle - i };

            line[middle + offset] = b'A' + offset as u8;
            line[middle - offset] = b'A' + offset as u8;

            String::from_utf8(line).unwrap()
        })
        .collect()
}
