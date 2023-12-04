use std::fs::read_to_string;

fn convert_num_char_to_i32(c: char) -> i32{
    let ascii_0 = '0' as i32; 
    c as i32 - ascii_0
}

fn char_is_num(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn check_if_engine_part(engine_scheme: &Vec<Vec<char>>, start_i: usize, start_j: usize, end_j: usize) -> bool {
    let j_start_bound = if start_j > 0 {
        start_j - 1
    } else {
        start_j
    };

    let j_end_bound = if end_j + 1 == engine_scheme.len() {
        end_j 
    } else {
        end_j + 1
    };

    let j_range = j_start_bound..j_end_bound;

    let symbol_count_row_1 = if start_i > 0 {
        engine_scheme[start_i - 1][j_range.clone()].iter().filter(|c| !char_is_num(**c) && **c != '.').count()
    } else {
        0
    };

    let symbol_count_row_2 = engine_scheme[start_i][j_range.clone()].iter().filter(|c| !char_is_num(**c) && **c != '.').count();

    let symbol_count_row_3 = if start_i + 1 < engine_scheme.len() {
         engine_scheme[start_i + 1][j_range.clone()].iter().filter(|c| !char_is_num(**c) && **c != '.').count()
    } else {
        0
    };

    symbol_count_row_1 > 0 || symbol_count_row_2 > 0 || symbol_count_row_3 > 0
}

fn get_result_1(engine_scheme: Vec<Vec<char>>) -> i32 {
    let mut result = 0;

    for i in 0..engine_scheme.len() {
        let mut curr_num = 0;
        let mut num_start_index = None;

        for j in 0..engine_scheme[i].len() {
            if char_is_num(engine_scheme[i][j]) {
                curr_num *= 10;
                curr_num += convert_num_char_to_i32(engine_scheme[i][j]);
                if num_start_index == None {
                    num_start_index = Some((i, j));
                }
            } else if curr_num > 0 {
                if let Some((start_i, start_j)) = num_start_index {
                    if check_if_engine_part(&engine_scheme, start_i, start_j, j) {
                        result += curr_num;
                    }
                }
                num_start_index = None;
                curr_num = 0;
            }
        }

        if curr_num > 0 {
            if let Some((start_i, start_j)) = num_start_index {
                if check_if_engine_part(&engine_scheme, start_i, start_j, engine_scheme[i].len() - 1) {
                    result += curr_num;
                }
            }
            num_start_index = None;
            curr_num = 0;
        }
    }
    result
}

fn get_num_from_index(engine_scheme: &Vec<Vec<char>>, row: usize, column: usize) -> i32 {
    let mut num = convert_num_char_to_i32(engine_scheme[row][column]);
    let mut factor = 1;
    for i in (0..column).rev() {
        if char_is_num(engine_scheme[row][i]) {
            factor *= 10;
            num += convert_num_char_to_i32(engine_scheme[row][i]) * factor;
        } else {
            break;
        }
    }

    for i in column+1..engine_scheme[row].len() {
        if char_is_num(engine_scheme[row][i]) {
            num *= 10;
            num += convert_num_char_to_i32(engine_scheme[row][i]);
        } else {
            break;
        }
    }
    num
}

fn find_gear_ratio(engine_scheme: &Vec<Vec<char>>, row: usize, column: usize) -> i32 {
    let mut num1 = 0;
    let mut num2 = 0;
    let mut continuous_num = false;

    let row_range = if row > 0 {
        row - 1..=row + 1
    } else {
        row..=row + 1
    };

    let col_range = if column > 0 {
        column - 1..=column + 1
    } else {
        column..=column + 1
    };

    for i in row_range {
        continuous_num = false;
        for j in col_range.clone() {
            if char_is_num(engine_scheme[i][j]) {
                if !continuous_num {
                    if num1 == 0 {
                        num1 = get_num_from_index(engine_scheme, i, j);
                        continuous_num = true;
                    } else {
                        num2 = get_num_from_index(engine_scheme, i, j);
                        break;
                    } 
                }
            } else {
                continuous_num = false;
            }
        }
        if num2 > 0 {
            break;
        }
    }

    num1 * num2
}

fn get_result_2(engine_scheme: Vec<Vec<char>>) -> i32 {
    let mut result = 0;

    for i in 0..engine_scheme.len() {
        for j in 0..engine_scheme[i].len() {
            if engine_scheme[i][j] == '*' {
                result += find_gear_ratio(&engine_scheme, i, j);
            }
        }
    }
    result
}

fn main() {
    let input_file = "input.txt";
    let mut engine_scheme: Vec<Vec<char>> = Vec::new();

    for line in read_to_string(input_file).unwrap().lines() {
        engine_scheme.push(line.to_string().chars().collect());
    }

    let result = get_result_2(engine_scheme);

    println!("{}", result);
}
