extern "C" {
    fn print(ptr: *const u8, len: usize);
}

#[cfg(feature = "part-1")]
#[no_mangle]
pub unsafe extern "C" fn main(ptr: *const u8, len: usize) {
    let input = std::slice::from_raw_parts(ptr, len);
    let s = std::str::from_utf8(input).unwrap();
    let (mut left, mut right) = from_str(s);

    left.sort();
    right.sort();

    let output = left
        .iter()
        .zip(right.iter())
        .fold(0, |acc, (l, r)| acc + (l - r).abs());

    let output = output.to_string();
    print(output.as_ptr(), output.len());
}

#[cfg(feature = "part-2")]
#[no_mangle]
pub unsafe extern "C" fn main(ptr: *const u8, len: usize) {
    use std::collections::HashMap;

    let input = std::slice::from_raw_parts(ptr, len);
    let s = std::str::from_utf8(input).unwrap();
    let (left, right) = from_str(s);

    let right = right.iter().fold(HashMap::new(), |mut acc, i| {
        *acc.entry(*i).or_insert(0) += 1;
        acc
    });

    let output = left
        .iter()
        .filter_map(|n| right.get(n).map(|inner| inner * n))
        .sum::<i32>();

    let output = output.to_string();
    print(output.as_ptr(), output.len());
}

fn from_str(input: &str) -> (Vec<i32>, Vec<i32>) {
    let (left, right) = input
        .lines()
        .fold((Vec::new(), Vec::new()), |mut acc, line| {
            let mut parts = line.split_whitespace();
            let left = parts.next().expect("left not found");
            let right = parts.next().expect("right not found");
            acc.0.push(left.parse::<i32>().expect("parse error"));
            acc.1.push(right.parse::<i32>().expect("parse error"));
            acc
        });

    (left, right)
}
