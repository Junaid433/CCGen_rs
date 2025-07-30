use rand::Rng;
use crate::card::{CardInfo, GeneratorOptions};
use crate::luhn::complete_luhn;

pub fn normalize_bin_pattern(mut pattern: String) -> String {
    while pattern.len() < 15 {
        pattern.push('x');
    }
    pattern
}

pub fn Generate_Cards(opts: &GeneratorOptions) -> Vec<CardInfo> {
    // Input validation
    let bin_pattern = opts.bin_pattern;
    // Reject if bin_pattern contains anything other than digits or 'x', or has spaces/symbols/letters
    if bin_pattern.len() > 16 || !bin_pattern.chars().all(|c| c.is_ascii_digit() || c == 'x') {
        return Vec::new();
    }
    // Reject if bin_pattern contains spaces or symbols
    if bin_pattern.chars().any(|c| c.is_whitespace() || (!c.is_ascii_digit() && c != 'x')) {
        return Vec::new();
    }
    // Validate month
    if let Some(m) = opts.month {
        if m < 1 || m > 12 {
            return Vec::new();
        }
    }
    // Validate year
    if let Some(y) = opts.year {
        if y < 2025 || y > 2050 {
            return Vec::new();
        }
    }
    // Validate cvv
    if let Some(c) = opts.cvv {
        if c > 999 {
            return Vec::new();
        }
    }

    let mut cards = Vec::new();
    let mut rng = rand::rng();

    let amount = opts.amount.unwrap_or(1);
    let mut pattern = normalize_bin_pattern(bin_pattern.to_string());
    // Always use only the first 15 digits for the base
    pattern.truncate(15);

    for _ in 0..amount {
        let mut generated = String::new();

        for ch in pattern.chars() {
            if ch == 'x' {
                generated.push_str(&rng.random_range(0..10).to_string());
            } else {
                generated.push(ch);
            }
        }
        // generated is now 15 digits, pass to complete_luhn
        let card_number = complete_luhn(&generated);

        let month = opts.month
        .map(|m| format!("{:02}", m))
        .unwrap_or_else(|| format!("{:02}", rng.random_range(01..=12)));

        let year = opts.year
            .map(|y| format!("{:04}", y))
            .unwrap_or_else(|| format!("{:04}", rng.random_range(2025..=2050)));

        let cvv = opts.cvv
            .map(|c| format!("{:03}", c))
            .unwrap_or_else(|| format!("{:03}", rng.random_range(0..=999)));

        cards.push(CardInfo {
            number: card_number,
            month,
            year,
            cvv,
        });
    }

    cards
}
