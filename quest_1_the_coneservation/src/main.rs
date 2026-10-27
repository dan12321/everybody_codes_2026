use std::{fs::OpenOptions, io::Read, path::PathBuf};

use clap::Parser;

#[derive(Parser)]
#[command(about, long_about = None)]
struct Args {
    /// File to process
    filename: PathBuf,
    /// Whether to calc to next square or next shell
    #[arg(short, long, default_value_t = false)]
    shell: bool,
    /// Whether to calc to next odd square
    #[arg(short, long, default_value_t = false)]
    odd: bool,
}

fn main() {
    let args = Args::parse();
    if args.shell && args.odd {
        eprintln!("Can't be both shell and odd");
        return;
    }
    let mut file = OpenOptions::new()
        .read(true)
        .write(false)
        .open(args.filename)
        .unwrap();

    let mut text = String::new();
    let _ = file.read_to_string(&mut text).unwrap();
    let result = if args.shell {
        calculate_shell_from_text(&text)
    } else if args.odd {
        calculate_odd_from_text(&text)
    } else {
        calculate_from_text(&text)
    };
    println!("{}", result);
}

fn calculate_from_text(s: &str) -> usize {
    s.trim()
        .split(",")
        .map(|n| n.parse().unwrap())
        .map(|n| diff_to_next_square(n) as usize)
        .sum()
}

fn diff_to_next_square(n: f32) -> f32 {
    n.sqrt().ceil().powi(2).max(1.0) - n
}

fn calculate_odd_from_text(s: &str) -> usize {
    s.trim()
        .split(",")
        .map(|n| n.parse().unwrap())
        .map(|n| diff_to_next_odd_square(n) as usize)
        .sum()
}

fn diff_to_next_odd_square(n: f32) -> f32 {
    let sq = (n + 1.0).sqrt().ceil();
    let odd_sq = sq + ((sq + 1.0) % 2.0);
    (odd_sq.powi(2).max(9.0) - 1.0) - n
}

fn calculate_shell_from_text(s: &str) -> usize {
    s.trim()
        .split(",")
        .map(|n| n.parse().unwrap())
        .map(diff_to_next_shell)
        .sum()
}

fn diff_to_next_shell(n: usize) -> usize {
    // It's just the next multiple of 4 unless we're under 8
    if n < 8 {
        return 8 - n;
    }
    (4 - n % 4) % 4
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3() {
        assert_eq!(diff_to_next_square(3.0), 1.0);
    }

    #[test]
    fn test_11() {
        assert_eq!(diff_to_next_square(11.0), 5.0);
    }

    #[test]
    fn test_16() {
        assert_eq!(diff_to_next_square(16.0), 0.0);
    }

    #[test]
    fn test_1() {
        assert_eq!(diff_to_next_square(1.0), 0.0);
    }

    #[test]
    fn test_0() {
        assert_eq!(diff_to_next_square(0.0), 1.0);
    }

    #[test]
    fn test_calc() {
        assert_eq!(calculate_from_text("3,11,12,16,24,25"), 11);
    }

    #[test]
    fn test_calc_newline() {
        assert_eq!(calculate_from_text("3,11,12,16,24,25\n"), 11);
    }

    #[test]
    fn test_shell_3() {
        assert_eq!(diff_to_next_shell(3), 5);
    }

    #[test]
    fn test_shell_12() {
        assert_eq!(diff_to_next_shell(12), 0);
    }

    #[test]
    fn test_shell_25() {
        assert_eq!(diff_to_next_shell(25), 3);
    }

    #[test]
    fn test_calc_shell() {
        assert_eq!(calculate_shell_from_text("3,11,12,16,24,25\n"), 9);
    }

    #[test]
    fn test_odd_3() {
        assert_eq!(diff_to_next_odd_square(3.0), 5.0);
    }

    #[test]
    fn test_odd_11() {
        assert_eq!(diff_to_next_odd_square(11.0), 13.0);
    }

    #[test]
    fn test_odd_16() {
        assert_eq!(diff_to_next_odd_square(16.0), 8.0);
    }

    #[test]
    fn test_odd_1() {
        assert_eq!(diff_to_next_odd_square(1.0), 7.0);
    }

    #[test]
    fn test_odd_0() {
        assert_eq!(diff_to_next_odd_square(0.0), 8.0);
    }

    #[test]
    fn test_odd_24() {
        assert_eq!(diff_to_next_odd_square(24.0), 0.0);
    }

    #[test]
    fn test_odd_25() {
        assert_eq!(diff_to_next_odd_square(25.0), 23.0);
    }

    #[test]
    fn test_calc_odd() {
        assert_eq!(calculate_odd_from_text("3,11,12,16,24,25\n"), 61);
    }
}
