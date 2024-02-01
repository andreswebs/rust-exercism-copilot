pub fn is_armstrong_number(num: u32) -> bool {
    let num_of_digits: u32 = ((num as f32).log10().floor() as u32) + 1;
    let mut sum: u32 = 0;
    let mut temp: u32 = num;
    while temp != 0 {
        let digit: u32 = temp % 10;
        sum = match digit.checked_pow(num_of_digits).and_then(|d: u32| sum.checked_add(d)) {
            Some(v) => v,
            None => return false,
        };
        temp /= 10;
    }
    sum == num
}