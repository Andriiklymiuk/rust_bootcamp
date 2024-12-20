fn main() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    // Bug 1: Attempting to modify immutable vector
    numbers.push(6);

    let slice = &mut numbers[..];
    modify_slice(slice); // Bug 2: Trying to modify through immutable reference

    let sum: i32 = numbers // Bug 3: Missing something after iterator operation
        .iter()
        .map(|x| x * 2)
        .filter(|x| x % 2 == 0)
        .sum();

    println!("Sum: {} {:?}", sum, numbers);
}

fn modify_slice(slice: &mut [i32]) {
    slice[0] = 10 // Bug 4: Modifying immutable slice
}
