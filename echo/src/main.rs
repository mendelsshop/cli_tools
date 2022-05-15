use std::{env, env::consts::OS, io, process::exit};

fn main() {
    let mut args: Vec<String> = env::args().collect();

    args.remove(0);
    let args = args; // makiing immutable now
    if args.is_empty() {
        println!("");
        exit(0);
    }
    let (index, no_newline, use_escape_char) = find_args(&args);
    let mut to_print = match index {
        -1 => String::from(""),
        _ => vec_to_string(&args, index.try_into().unwrap()),
    };
    to_print = format_print(&mut to_print, use_escape_char);
    print!("{}{}", to_print, if no_newline { "" } else { "\n" });
}

fn find_args(args: &Vec<String>) -> (i32, bool, bool) {
    let mut use_escape_char: bool = false;
    let mut no_newline: bool = false;
    for (index, arg) in args.iter().enumerate() {
        if !arg.starts_with('-') {
            return (index.try_into().unwrap(), no_newline, use_escape_char);
        }

        for (indexs, chars) in arg.chars().enumerate() {
            match chars {
                'e' => use_escape_char = true,
                'E' => use_escape_char = false,
                'n' => no_newline = true,
                _ => {
                    if indexs == 0 && chars == '-' {
                    } else {
                        return (index.try_into().unwrap(), no_newline, use_escape_char);
                    }
                }
            }
        }
    }
    (-1, no_newline, use_escape_char)
}

fn vec_to_string(args: &Vec<String>, index: usize) -> String {
    let mut output = String::new();
    for (indexs, string) in args.iter().enumerate() {
        if indexs < index {
            continue;
        } else {
            output.push_str(string);
            if indexs != args.len() - 1 {
                output.push_str(" ");
            }
        };
    }
    output
}

fn format_print(output: &mut String, use_escape_char: bool) -> String {
    let mut slash_count = 0;
    let mut returns = String::new();
    let mut slashes = String::new();

    for (indexs, chars) in output.chars().enumerate() {
        if use_escape_char {
            if chars == '\\' {
                if indexs == output.len() - 1 {
                    let mut it: String = String::new();
                    if slash_count % 2 == 0 {
                        print!(">");
                        io::stdin().read_line(&mut it).expect("Failed to read line");
                    }
                    returns.push_str(do_slash_count(slash_count + 1, &mut slashes));
                    break;
                }
                slash_count += 1;
                continue;
            }
            match chars {
                // TODO #0: find the proper slash count for -e
                // TODO #1: create a funtion to do what each match brach does
                'a' => {
                    if slash_count > 1 {
                        returns.push_str(do_slash_count(slash_count, &mut slashes));
                        returns.push('\x07') // /x07 is the escape character for "bell" in unicode/assci https://en.wikipedia.org/wiki/Bell_character
                    } else {
                        returns.push(chars)
                    }
                    slash_count = 0;
                }
                'b' => {
                    if slash_count > 1 {
                        returns.push_str(do_slash_count(slash_count, &mut slashes));
                        returns.remove(returns.len() - 1);
                    } else {
                        returns.push(chars);
                    }
                    slash_count = 0;
                }
                'c' => {
                    if slash_count > 1 {
                        returns.push_str(do_slash_count(slash_count, &mut slashes));
                        return returns;
                    } else {
                        returns.push(chars);
                    }
                    slash_count = 0;
                }
                'n' => {
                    if slash_count > 1 {
                        returns.push_str(do_slash_count(slash_count, &mut slashes));
                        returns.push('\n');
                    } else {
                        returns.push(chars);
                    }
                    slash_count = 0;
                }
                'r' => {
                    if slash_count > 1 {
                        returns.push_str(do_slash_count(slash_count, &mut slashes));
                        returns.push('\r');
                    } else {
                        returns.push(chars);
                    }
                    slash_count = 0;
                }
                't' => {
                    if slash_count > 1 {
                        returns.push_str(do_slash_count(slash_count, &mut slashes));
                        returns.push('\t');
                    } else {
                        returns.push(chars);
                    }
                    slash_count = 0;
                }
                _ => {
                    if slash_count > 1 {
                        returns.push_str(do_slash_count(slash_count, &mut slashes));
                    }
                    returns.push(chars);
                    slash_count = 0;
                }
            }
        } else {
            returns.push(chars);
        }
    }
    returns
}

fn do_slash_count(slash_count: i32, returnss: &mut String) -> &str {
    returnss.clear();

    let num_slashs_to_display = if slash_count % 2 == 0 {
        slash_count - slash_count / 2
    } else {
        slash_count - 1 - slash_count / 2
    };
    for _num in 0..num_slashs_to_display {
        returnss.push('\\')
    }

    returnss
}
