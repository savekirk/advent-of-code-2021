pub fn line_to_chars(line: &String) -> Vec<char> {
    line.split("")
        .filter(|v| !v.is_empty())
        .map(|x| x.parse::<char>().unwrap())
        .collect()
}
