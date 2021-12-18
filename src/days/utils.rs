pub fn line_to_chars(line: &String) -> Vec<char> {
    line.split("")
        .filter(|v| !v.is_empty())
        .map(|x| x.parse::<char>().unwrap())
        .collect()
}

pub fn line_to_int(line: &String) -> Vec<u8> {
    line.split("")
        .filter(|v| !v.is_empty())
        .map(|x| x.parse::<u8>().unwrap())
        .collect()
}
