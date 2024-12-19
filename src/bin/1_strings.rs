fn main() {
    // FIXME: Each of these has a type-related bug!
    let percentage: u8 = -50;  // Bug 1: negative number in unsigned
    
    let price: f32 = 10 / 3;  // Bug 2: integer division happening before cast
    
    let message = String::from("Hello");
    let borrowed: String = &message;  // Bug 3: wrong string type coercion
    
    let chars: &str = 'A';  // Bug 4: char vs string confusion

    let crab = '🦀';
    let crab_len = crab.len(); // Bug 5: char.len() instead of ?

    println!("Results: {} {} {} {} {}", 
        percentage, price, borrowed, chars, crab_len);
}