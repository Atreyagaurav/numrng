use std::collections::HashSet;

use number_range::{NumberRange, NumberRangeOptions};

fn main() {
    let mut sorted = false;
    let mut negative = false;
    let mut unique = false;
    let mut numrng = Vec::new();
    let mut stdin = false;
    for arg in std::env::args().skip(1) {
        if arg.starts_with("-") {
            // is an option
            if arg.len() == 1 {
                stdin = true;
            }
            // since clap is heavy for this simple task, doing it manually
            for a in arg[1..].chars() {
                match a {
                    'h' => {
                        print_help(false);
                        return;
                    }
                    's' => {
                        sorted = true;
                    }
                    'n' => {
                        negative = true;
                    }
                    'u' => {
                        unique = true;
                    }
                    _ => {
                        eprintln!("Invalid Option");
                        print_help(true);
                        return;
                    }
                }
            }
        } else {
            numrng.push(arg);
        }
    }

    for num in numrng {
        print_numbers(&num, negative, unique, sorted);
    }

    if stdin {
        for line in std::io::stdin().lines() {
            print_numbers(&line.unwrap(), negative, unique, sorted);
        }
    }
}

fn print_numbers(num: &str, negative: bool, unique: bool, sorted: bool) {
    let sep = if negative || num.contains(":") {
        ':'
    } else {
        '-'
    };
    let op = NumberRangeOptions::default()
        .with_range_sep(sep)
        .with_whitespace(true);
    let rng: NumberRange<i64> = match op.parse(num) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{:?}", e);
            return;
        }
    };
    if unique {
        let mut rng: Vec<i64> = rng.collect::<HashSet<i64>>().into_iter().collect();
        rng.sort();
        rng.iter().for_each(|n| println!("{n}"));
    } else if sorted {
        let mut rng: Vec<i64> = rng.collect();
        rng.sort();
        rng.iter().for_each(|n| println!("{n}"));
    } else {
        rng.for_each(|n| println!("{n}"));
    }
}

fn print_help(err: bool) {
    let help_msg = concat!(
        "Usage: numrng [options] [NUM-RNG]\n\n",
        "Options:\n",
        "  -h: print this help message\n",
        "  -s: sort the numbers within a line before printing\n",
        "      Note: sort only occurs within a line (from stdin)\n",
        "      or within a single argument (from CLI), not globally.\n",
        "  -n: input has negative numbers\n",
        "      (input must use : instead of - for range separator)\n",
        "  -u: print only unique numbers (implies sort)\n",
        "\n",
        "NUM-RNG: Number Range in human readable format. (pass - for stdin)\n",
        "         The number range can be comma separated list of\n",
        "         single range numbers, which can be a single number\n",
        "         start:end, or start:step:end. For non-negative numbers\n",
        "         `-` can be used instead of `:` for range separator.\n",
        "         The range separator will be auto detected unless -n\n",
        "         (negative) flag is on.\n",
        "\n",
        "Examples:\n",
        "  `numrng 1-4`             : 1, 2, 3, 4\n",
        "  `numrng 1:4`             : 1, 2, 3, 4\n",
        "  `numrng -1:2`            : -1, 0, 1, 2\n",
        "  `numrng 5:-1:2`          : 5, 4, 3, 2\n",
        "  `numrng 30:32,1:4,31`    : 30, 31, 32, 1, 2, 3, 4, 31\n",
        "  `numrng 30:32,1:4 -s`    : 1, 2, 3, 4, 30, 31, 31, 32\n",
        "  `numrng 30:32,1:4 -u`    : 1, 2, 3, 4, 30, 31, 32\n"
    );
    if err {
        eprintln!("{}", help_msg);
    } else {
        println!("{}", help_msg);
    }
}
