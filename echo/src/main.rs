use std::{env, process::exit};

fn main() {
    let mut args: Vec<String> = env::args().collect(); // have to fihure out how to handle diffferent shells handling of \\

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
    if use_escape_char {
        to_print = format_print(&mut to_print);
    } else {
        // to_print = to_print.replace("\\\\", "\\"); // backslash
    }
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
fn format_print(output: &mut String) -> String {
    // some of this was adapted from https://codereview.stackexchange.com/questions/230429/rust-echo-implementation-that-supports-command-line-options
    let mut output = String::from(output.as_str());
    output = output.replace("\\\\", "\\"); // backslash
    output = output.replace("\\a", "\x07"); // alert (BEL)
    output = output.replace("\\b", "\x08"); // backspace
    output = output.replace("\\e", "\x1B"); // escape
    output = output.replace("\\f", "\x0c"); // form feed
    output = output.replace("\\n", "\n"); // new line
    output = output.replace("\\r", "\r"); // carriage return
    output = output.replace("\\t", "\t"); // horizontal tab
    output = output.replace("\\v", "\x0b"); // vertical tab

    if output.starts_with("\\x") && 3 <= output.len() && output.len() <= 4 {
        // Hex values with at most 2 digits
        let value = output.trim_start_matches("\\x");
        let chr = u8::from_str_radix(value, 16).unwrap() as char;
        output.push_str(chr.to_string().as_str());
    }

    if output.starts_with("\\0") && 3 <= output.len() && output.len() <= 5 {
        // Octal values with at most 3 digits
        let value = output.trim_start_matches("\\0");

        let chr = u8::from_str_radix(value, 8);
        // The maximum octal value for a byte is 377.
        // Check that this conversion was successful.
        if chr.is_ok() {
            let x = chr.unwrap() as char;
            output.push(x);
        }
    }
    let index = output.find("\\c");
    if index == None {
    } else {
        let index = index.unwrap();
        return String::from(&output[0..index]);
    }
    output
}
