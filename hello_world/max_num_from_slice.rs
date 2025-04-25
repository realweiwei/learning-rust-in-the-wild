pub fn largest<T: PartialOrd>(slice: &[T]) -> Option<&T> {
    let mut max = slice.get(0)?;
    for item in &slice[1..] {
        if item > max {
            max = item;
        }
    }
    Some(max)
}

fn main() {
    let nums = [3, 7, 2, 9, 4];
    match largest(&nums) {
        Some(&n) => println!("The largest number is {}", n),
        None => println!("Slice was empty!"),
    }
    let empty: [i32; 0] = [];
    assert!(largest(&empty).is_none());
}