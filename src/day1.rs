use std::collections::BTreeMap;

#[derive(Debug)]
struct DigitFinder {
    start_idx: Option<usize>,
    current: Vec<char>,
    current_valid_sequences: Vec<usize>,
    digit_sequences: Vec<Vec<char>>,
}

impl DigitFinder {
    pub fn new() -> Self {
        let digit_sequences = vec![
            "zero".chars().collect(),
            "one".chars().collect(),
            "two".chars().collect(),
            "three".chars().collect(),
            "four".chars().collect(),
            "five".chars().collect(),
            "six".chars().collect(),
            "seven".chars().collect(),
            "eight".chars().collect(),
            "nine".chars().collect(),
        ];

        Self {
            start_idx: None,
            current: Vec::default(),
            current_valid_sequences: Vec::default(),
            digit_sequences,
        }
    }

    fn reset(&mut self) {
        self.start_idx = None;
        self.current_valid_sequences = Vec::default();
        self.current = Vec::default();
    }

    pub fn add_char(&mut self, index: usize, c: char) -> Option<(usize, usize, u32)> {
        if self.start_idx.is_none() {
            for (sequence_value, sequence) in self.digit_sequences.iter().enumerate() {
                if *sequence.first().unwrap() == c {
                    self.start_idx = Some(index);
                    self.current_valid_sequences.push(sequence_value);
                }
            }
            if self.current_valid_sequences.len() > 0 {
                self.current.push(c);
            }
            None
        } else {
            let start_idx = self.start_idx.unwrap();
            self.current.push(c);

            let mut new_valid_sequences = vec![];
            for (sequence_value, sequence) in self.digit_sequences.iter().enumerate() {
                if *sequence == self.current {
                    let end_index = start_idx + self.current.len();
                    let value = sequence_value as u32;
                    self.reset();
                    self.add_char(index, c);
                    return Some((start_idx, end_index, value));
                }

                if sequence.starts_with(self.current.as_slice()) {
                    new_valid_sequences.push(sequence_value);
                }
            }
            if new_valid_sequences.is_empty() {
                self.current.pop();
                if self.current.len() > 1 {
                    let new_current = &self.current[1..];
                    self.current = new_current.into();
                    self.start_idx = Some(start_idx + 1);
                    return self.add_char(index, c);
                } else {
                    self.reset();
                }
            }
            None
        }
    }
}

fn digit_filter(c: char) -> Option<u32> {
    if c.is_digit(10) {
        let res = u32::try_from(c).expect(&format!("{c} is apparently not a digit"));
        Some(res - 48)
    } else {
        None
    }
}

fn day_one() {
    let input = include_str!("../data/adventofcodeday1input.txt");
    let mut sum = 0;

    input.split("\n").for_each(|line| {
        let digits: Vec<u32> = line.chars().filter_map(digit_filter).collect();
        if digits.len() > 0 {
            sum += digits.first().unwrap_or(&0u32) * 10 + digits.last().unwrap_or(&0u32);
        }
    });
    println!("{sum}")
}

fn day_one_part_2_not_working() -> Vec<(String, Vec<u32>)>{
    let input = include_str!("../data/adventofcodeday1input.txt");
    let mut sum = 0;
    let mut finder = DigitFinder::new();
    let mut digit_results = vec![];

    input.split("\n").for_each(|line| {
        println!("{line}");
        finder.reset();
        let mut digits = vec![];

        for (index, character) in line.chars().enumerate() {
            if let Some((_, _, value)) = finder.add_char(index, character) {
                digits.push(value);
            }
            if let Some(value) = digit_filter(character) {
                digits.push(value)
            }
        }

        if digits.len() > 0 {
            sum += digits.first().unwrap_or(&0u32) * 10 + digits.last().unwrap_or(&0u32);
        }

        digit_results.push((line.to_string(), digits));
    });
    println!("{sum}");
    digit_results
}

fn day_one_part_2() -> Vec<(String, Vec<u32>)>{
    let mut values = BTreeMap::new();
    values.insert("zero", 0u32);
    values.insert("one", 1u32);
    values.insert("two", 2u32);
    values.insert("three", 3u32);
    values.insert("four", 4u32);
    values.insert("five", 5u32);
    values.insert("six", 6u32);
    values.insert("seven", 7u32);
    values.insert("eight", 8u32);
    values.insert("nine", 9u32);
    let mut sum = 0;
    let input = include_str!("../data/adventofcodeday1input.txt");
    let mut digit_results = vec![];
    for line in input.lines() {
        let mut digits = vec![];
        for (i, c) in line.chars().enumerate() {
            if let Some(value) = digit_filter(c) {
                digits.push(value);
            }
            for (k, v) in &values {
                if line[i..].starts_with(k) {
                    digits.push(*v);
                }
            }
        }
        if digits.len() > 0 {
            sum += digits.first().unwrap_or(&0u32) * 10 + digits.last().unwrap_or(&0u32);
        }
        digit_results.push((line.to_string(), digits));
    }
    println!("{sum}");
    digit_results
}

pub fn run() {
    day_one();
    day_one_part_2();
}

#[cfg(test)]
mod test {
    use super::{day_one_part_2, day_one_part_2_not_working, DigitFinder};

    #[test]
    fn test_digit_finder() {
        let mut finder = DigitFinder::new();
        let cast = "6twofive3two";
        for (i, c) in cast.chars().enumerate() {
            if let Some((_, _, value)) = finder.add_char(i, c) {
                println!("{value}")
            }
        }
    }

    #[test]
    fn compare() {
        let mine = day_one_part_2_not_working();
        let simple = day_one_part_2();
        for ((line, my_digits), (_line_copy, their_digits)) in mine.iter().zip(simple.iter()) {
            println!("{line}");
            println!("{my_digits:?}");
            println!("{their_digits:?}");
            assert_eq!(*my_digits, *their_digits);
        }

    }
}
