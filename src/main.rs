fn main() {
    let mut result = 0;
    for line in input_text() {
        let parsed = parse_digits(&line);
        let digits = concat_first_and_last(&parsed);
        println!("{:?}", digits);
        result += digits;
    }
    println!("Result: {}", result);
}

fn parse_digits(string: &str) -> Vec<u32> {
    string.chars().filter_map(|c| c.to_digit(10)).collect()
}

fn input_text() -> Vec<String> {
    let input = "
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
        ";
    input
        .to_string()
        .lines()
        .map(|s| replace_digits(s))
        .collect()
}

fn replace_digits(string: &str) -> String {
    let new_string = string
        .replace("one", "oonee")
        .replace("two", "ttwoo")
        .replace("three", "tthreee")
        .replace("four", "ffourr")
        .replace("five", "ffivee")
        .replace("six", "ssixx")
        .replace("seven", "ssevenn")
        .replace("eight", "eeightt")
        .replace("nine", "nninee")
        .replace("zero", "zzeroo");
    new_string
        .replace("one", "1")
        .replace("two", "2")
        .replace("three", "3")
        .replace("four", "4")
        .replace("five", "5")
        .replace("six", "6")
        .replace("seven", "7")
        .replace("eight", "8")
        .replace("nine", "9")
        .replace("zero", "0")
}

fn concat_first_and_last(digits: &Vec<u32>) -> u32 {
    if digits.len() == 0 {
        return 0;
    }
    [
        digits.get(0).unwrap().to_string(),
        digits.last().unwrap().to_string(),
    ]
    .join("")
    .parse::<u32>()
    .unwrap()
}
