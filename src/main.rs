use std::io::{self, BufRead};
use std::env;

fn parse_input_string(input_string: &str, quote_char: char, strip_quote_char: bool) -> Vec<String> {
    let mut result = Vec::new();
    let mut start = 0;
    let mut end = 0;
    let mut in_quotes = false;
    let mut chars = input_string.chars();
    while let Some(c) = chars.next() {
        if c == quote_char {
            in_quotes = !in_quotes;
            if !in_quotes {
                if strip_quote_char {
                    result.push(input_string[start+1..end].to_string());
                    start = end + 2;
                } else {
                    result.push(input_string[start..end+1].to_string());
                    start = end + 2;
                }
            }
        } else if c == ' ' && !in_quotes {
            if end > start {
                result.push(input_string[start..end].to_string());
            }
            start = end + 1;
        }
        end += 1;
    }
    if end > start {
        result.push(input_string[start..end].to_string());
    }
    result
}

fn print_usage() {
    println!("{}", "Usage : pickup {-q} {-s} start:end");
    println!("  -q   -> use single-quote character. note that double-quote is default.");
    println!("  -s   -> strip the quote character in the output");
    println!("start:end example");
    println!("   0   -> first element only");
    println!("   1   -> second element only");
    println!("   -1  -> the last element only");
    println!("   -2  -> the previous of last element only");
    println!("   0:2 -> first, second and thrid elements");
    println!("   1:  -> From the second to the last elements");
    println!("   :3  -> first, second, third and 4th elements");
}

fn main() {
    // parse argument
    let mut quote_char = '"';
    let mut strip_quote_char = false;
    if env::args().len() < 2 {
        print_usage();
        return;
    }

    let mut start_idx = -1;
    let mut end_idx = -1;

    let args: Vec<String> = env::args().collect();
    for arg in args {
        if arg == "-q" {
            quote_char = '\'';
        } else if arg == "-s" {
            strip_quote_char = true;
        } else {
            let parts: Vec<&str> = arg.split(':').collect();
            if parts.len() >= 2 {
                match parts[0].parse::<i32>() {
                    Ok(integer) => {
                        start_idx = integer;
                    },
                    Err(_) => {
                        start_idx = std::i32::MAX;
                    }
                }
                match parts[1].parse::<i32>() {
                    Ok(integer) => {
                        end_idx = integer;
                    },
                    Err(_) => {
                        end_idx = std::i32::MAX;
                    }
                }
            } else {
                match arg.parse::<i32>() {
                    Ok(integer) => {
                        start_idx = integer;
                        end_idx = integer;
                    },
                    Err(_) => {
                        start_idx = std::i32::MAX;
                        end_idx = std::i32::MAX;
                    }
                }
            }
        }
    }

    let stdin = io::stdin();
    for ln in stdin.lock().lines() {
        let line;
        match ln {
            Ok(data) => line = data,
            Err(_) => continue
        }

        let v = parse_input_string(&line, quote_char, strip_quote_char);

        // for i in 0..v.len() {
        //     println!("|{}|", v[i]);
        // }
        // println!("--------------------");
        // println!("start={} end={}", start_idx, end_idx);

        if start_idx == end_idx {
            if start_idx < 0 {
                start_idx = (v.len() as i32) + start_idx;
                if start_idx < 0 {
                    return;
                }
            }
            if start_idx < v.len() as i32 {
                println!("{}", v[start_idx as usize]);
            }
            return;
        }

        if start_idx == std::i32::MAX {
            start_idx = 0; // adjusted
            if end_idx < 0 {
                end_idx = (v.len() as i32) + end_idx;
                if end_idx < 0 {
                    return;
                }
            }
        } else if end_idx == std::i32::MAX {
            end_idx = v.len() as i32 - 1; // adjusted
            if start_idx < 0 {
                start_idx = (v.len() as i32) + start_idx;
                if start_idx < 0 {
                    return;
                }
            }
        } else {
            if start_idx < 0 {
                start_idx = (v.len() as i32) + start_idx;
                if start_idx < 0 {
                    return;
                }
            }
            if end_idx < 0 {
                end_idx = (v.len() as i32) + end_idx;
                if end_idx < 0 {
                    return;
                }
            }
            if start_idx > end_idx {
                return;
            }
        }
        if start_idx < v.len() as i32 && end_idx < v.len() as i32 {
            for i in start_idx..=end_idx {
                if i != end_idx {
                    print!("{} ", v[i as usize]);
                } else {
                    println!("{}", v[i as usize]);
                }
            }
        }
    }
}
