pub struct GeneratorOptions<'a> {
    pub bin_pattern: &'a str,
    pub month: Option<usize>,
    pub year: Option<usize>,
    pub cvv: Option<usize>,
    pub amount: Option<usize>,
}