pub type CharSet = u64;
pub fn bitwise_it(s: &str) -> CharSet {
    let mut result = 0;
    for i in s.chars() {
        let value = get_value(i) - 1;
        result |= (1 << value)
    }
    result
}

pub fn get_value(c: char) -> u32 {
    if c.is_uppercase() {
        c as u32 - 'A' as u32 + 27
    } else {
        c as u32 - 'a' as u32 + 1
    }
}
pub fn get_value_shifted(c: char) -> CharSet{
    1 << get_value(c)
}


