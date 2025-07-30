// Returns the Luhn checksum for the given digits (excluding the check digit)
pub fn luhn_checksum(digits: &[u32]) -> u32 {
    let mut sum = 0;
    let len = digits.len();
    for (i, digit) in digits.iter().enumerate() {
        let mut val = *digit;
        // Luhn: double every second digit from the right (i.e., from the end)
        if (len - i) % 2 == 0 {
            val *= 2;
            if val > 9 {
                val -= 9;
            }
        }
        sum += val;
    }
    sum
}

// Given a 15-digit partial string, returns the 16-digit Luhn-complete string
pub fn complete_luhn(partial: &str) -> String {
    let mut number: Vec<u32> = partial
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect();
    // Compute check digit
    let sum = luhn_checksum(&number);
    let check_digit = (10 - (sum % 10)) % 10;
    number.push(check_digit);
    number.iter().map(|d| d.to_string()).collect()
}
