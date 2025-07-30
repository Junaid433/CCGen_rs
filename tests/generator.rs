use ccgen_rs::{GeneratorOptions, Generate_Cards};

// Helper: Luhn check
fn luhn_check(number: &str) -> bool {
    let digits: Vec<u32> = number.chars().filter_map(|c| c.to_digit(10)).collect();
    let sum: u32 = digits.iter().rev().enumerate().map(|(i, d)| {
        if i % 2 == 1 {
            let dbl = d * 2;
            if dbl > 9 { dbl - 9 } else { dbl }
        } else {
            *d
        }
    }).sum();
    sum % 10 == 0
}

#[test]
fn generates_correct_amount() {
    let opts = GeneratorOptions {
        bin_pattern: "411111xxxxxxxxxx",
        month: Some(1),
        year: Some(2030),
        cvv: Some(123),
        amount: Some(5),
    };
    let cards = Generate_Cards(&opts);
    assert_eq!(cards.len(), 5);
}

#[test]
fn card_number_length_is_16() {
    let opts = GeneratorOptions {
        bin_pattern: "411111xxxxxxxxxx",
        month: None,
        year: None,
        cvv: None,
        amount: Some(1),
    };
    let cards = Generate_Cards(&opts);
    assert_eq!(cards[0].number.len(), 16);
}


#[test]
fn respects_custom_month_year_cvv() {
    let opts = GeneratorOptions {
        bin_pattern: "411111xxxxxxxxxx",
        month: Some(7),
        year: Some(2042),
        cvv: Some(999),
        amount: Some(1),
    };
    let cards = Generate_Cards(&opts);
    let card = &cards[0];
    assert_eq!(card.month, "07");
    assert_eq!(card.year, "2042");
    assert_eq!(card.cvv, "999");
}

#[test]
fn random_month_year_cvv_are_valid() {
    let opts = GeneratorOptions {
        bin_pattern: "411111xxxxxxxxxx",
        month: None,
        year: None,
        cvv: None,
        amount: Some(10),
    };
    let cards = Generate_Cards(&opts);
    for card in cards {
        let month: u32 = card.month.parse().unwrap();
        let year: u32 = card.year.parse().unwrap();
        let cvv: u32 = card.cvv.parse().unwrap();
        assert!((1..=12).contains(&month));
        assert!((2025..=2050).contains(&year));
        assert!(cvv <= 999);
    }
}

#[test]
fn short_bin_pattern_is_padded() {
    let opts = GeneratorOptions {
        bin_pattern: "4111",
        month: None,
        year: None,
        cvv: None,
        amount: Some(1),
    };
    let cards = Generate_Cards(&opts);
    assert_eq!(cards[0].number.len(), 16);
    assert!(cards[0].number.starts_with("4111"));
}

#[test]
fn amount_none_defaults_to_one() {
    let opts = GeneratorOptions {
        bin_pattern: "411111xxxxxxxxxx",
        month: None,
        year: None,
        cvv: None,
        amount: None,
    };
    let cards = Generate_Cards(&opts);
    assert_eq!(cards.len(), 1);
}

#[test]
fn edge_cvv_values() {
    let opts_zero = GeneratorOptions {
        bin_pattern: "411111xxxxxxxxxx",
        month: None,
        year: None,
        cvv: Some(0),
        amount: Some(1),
    };
    let card_zero = &Generate_Cards(&opts_zero)[0];
    assert_eq!(card_zero.cvv, "000");

    let opts_max = GeneratorOptions {
        bin_pattern: "411111xxxxxxxxxx",
        month: None,
        year: None,
        cvv: Some(999),
        amount: Some(1),
    };
    let card_max = &Generate_Cards(&opts_max)[0];
    assert_eq!(card_max.cvv, "999");
}

#[test]
fn different_cards_generated_with_x() {
    let opts = GeneratorOptions {
        bin_pattern: "411111xxxxxxxxxx",
        month: None,
        year: None,
        cvv: None,
        amount: Some(5),
    };
    let cards = Generate_Cards(&opts);
    let mut numbers = cards.iter().map(|c| &c.number).collect::<Vec<_>>();
    numbers.sort();
    numbers.dedup();
    assert!(numbers.len() > 1, "Expected different card numbers, got only one unique");
}

// Additional tests to reach at least 24
#[test]
fn bin_pattern_with_letters_is_rejected() {
    let opts = GeneratorOptions {
        bin_pattern: "4111abcdxxxxxxxx",
        month: None,
        year: None,
        cvv: None,
        amount: Some(1),
    };
    let cards = Generate_Cards(&opts);
    assert_eq!(cards.len(), 0);
}


#[test]
fn bin_pattern_too_long() {
    let opts = GeneratorOptions {
        bin_pattern: "41111111111111111111",
        month: None,
        year: None,
        cvv: None,
        amount: Some(1),
    };
    let cards = Generate_Cards(&opts);
    assert_eq!(cards.len(), 0);
}

#[test]
fn bin_pattern_too_short() {
    let opts = GeneratorOptions {
        bin_pattern: "4",
        month: None,
        year: None,
        cvv: None,
        amount: Some(1),
    };
    let cards = Generate_Cards(&opts);
    assert_eq!(cards[0].number.len(), 16);
}

#[test]
fn invalid_month_is_rejected() {
    let opts = GeneratorOptions {
        bin_pattern: "411111xxxxxxxxxx",
        month: Some(13),
        year: Some(2030),
        cvv: None,
        amount: Some(1),
    };
    let cards = Generate_Cards(&opts);
    assert_eq!(cards.len(), 0);
}

#[test]
fn invalid_year_is_rejected() {
    let opts = GeneratorOptions {
        bin_pattern: "411111xxxxxxxxxx",
        month: Some(1),
        year: Some(2100),
        cvv: None,
        amount: Some(1),
    };
    let cards = Generate_Cards(&opts);
    assert_eq!(cards.len(), 0);
}

#[test]
fn invalid_cvv_is_rejected() {
    let opts = GeneratorOptions {
        bin_pattern: "411111xxxxxxxxxx",
        month: Some(1),
        year: Some(2030),
        cvv: Some(1000),
        amount: Some(1),
    };
    let cards = Generate_Cards(&opts);
    assert_eq!(cards.len(), 0);
}

#[test]
fn generate_large_amount() {
    let opts = GeneratorOptions {
        bin_pattern: "411111xxxxxxxxxx",
        month: None,
        year: None,
        cvv: None,
        amount: Some(100),
    };
    let cards = Generate_Cards(&opts);
    assert_eq!(cards.len(), 100);
}

#[test]
fn all_generated_cards_unique() {
    let opts = GeneratorOptions {
        bin_pattern: "411111xxxxxxxxxx",
        month: None,
        year: None,
        cvv: None,
        amount: Some(20),
    };
    let cards = Generate_Cards(&opts);
    let mut numbers = cards.iter().map(|c| &c.number).collect::<Vec<_>>();
    numbers.sort();
    numbers.dedup();
    assert_eq!(numbers.len(), 20);
}

#[test]
fn generated_card_month_padding() {
    let opts = GeneratorOptions {
        bin_pattern: "411111xxxxxxxxxx",
        month: Some(5),
        year: Some(2030),
        cvv: None,
        amount: Some(1),
    };
    let card = &Generate_Cards(&opts)[0];
    assert_eq!(card.month, "05");
}

#[test]
fn generated_card_year_format() {
    let opts = GeneratorOptions {
        bin_pattern: "411111xxxxxxxxxx",
        month: Some(1),
        year: Some(2035),
        cvv: None,
        amount: Some(1),
    };
    let card = &Generate_Cards(&opts)[0];
    assert_eq!(card.year, "2035");
}

#[test]
fn generated_card_cvv_padding() {
    let opts = GeneratorOptions {
        bin_pattern: "411111xxxxxxxxxx",
        month: None,
        year: None,
        cvv: Some(7),
        amount: Some(1),
    };
    let card = &Generate_Cards(&opts)[0];
    assert_eq!(card.cvv, "007");
}

#[test]
fn bin_pattern_with_spaces() {
    let opts = GeneratorOptions {
        bin_pattern: "4111 11xx xxxx xxxx",
        month: None,
        year: None,
        cvv: None,
        amount: Some(1),
    };
    let cards = Generate_Cards(&opts);
    assert_eq!(cards.len(), 0);
}

#[test]
fn bin_pattern_with_symbols() {
    let opts = GeneratorOptions {
        bin_pattern: "4111-11xx-xxxx-xxxx",
        month: None,
        year: None,
        cvv: None,
        amount: Some(1),
    };
    let cards = Generate_Cards(&opts);
    assert_eq!(cards.len(), 0);
}

#[test]
fn bin_pattern_with_partial_x() {
    let opts = GeneratorOptions {
        bin_pattern: "411111xx",
        month: None,
        year: None,
        cvv: None,
        amount: Some(1),
    };
    let cards = Generate_Cards(&opts);
    assert_eq!(cards[0].number.len(), 16);
}
