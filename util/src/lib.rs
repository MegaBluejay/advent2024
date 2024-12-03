use atoi::FromRadix10;

pub fn atoi_with_rest<I: FromRadix10>(text: &[u8]) -> Option<(I, &[u8])> {
    match I::from_radix_10(text) {
        (_, 0) => None,
        (n, used) => Some((n, &text[used..])),
    }
}
