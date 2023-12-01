use std::collections::HashMap;
use std::fs::read_to_string;

fn convert_num_char_to_i32(c: char) -> i32{
    let ascii_0 = '0' as i32; 
    c as i32 - ascii_0
}

fn char_is_num(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn get_calibration_value_part_1(s: String) -> i32 {
    let nums = s.chars().filter(|x| char_is_num(*x)).map(|x| convert_num_char_to_i32(x)).collect::<Vec<i32>>();

    return nums[0] * 10 + nums[nums.len() - 1];
}

fn get_calibration_value_part_2(s: String) -> i32 {
    let chars_map = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9)
    ]);
    let mut nums: Vec<i32> = Vec::new();

    for i in 0..s.len() {
        let curr_char = s.chars().nth(i).unwrap();
        if char_is_num(curr_char) {
            nums.push(convert_num_char_to_i32(curr_char));
        }
        for j in 3..5 + 1 {
            if i + j <= s.len() {
                if let Some(int_val) = chars_map.get(&s[i..i+j]) {
                    nums.push(*int_val);
                }
            }
        }
    }

    return nums[0] * 10 + nums[nums.len() - 1];
}

fn main() {
    let input_file = "input.txt";

    let mut result = 0;

    for line in read_to_string(input_file).unwrap().lines() {
        result += get_calibration_value_part_2(line.to_string());
    }

    println!("{}", result);
}
