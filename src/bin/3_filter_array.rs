#[derive(Debug)]
struct Console {
    brand: String,
    model: String,
    release_year: u32,
}

fn main() {
    let mut consoles = vec![
        Console {
            brand: String::from("Xbox"),
            model: String::from("360"),
            release_year: 2005,
        },
        Console {
            brand: String::from("Xbox"),
            model: String::from("One"),
            release_year: 2013,
        },
        Console {
            brand: String::from("Xbox"),
            model: String::from("Series X"),
            release_year: 2020,
        },
        Console {
            brand: String::from("PlayStation"),
            model: String::from("4"),
            release_year: 2013,
        },
        Console {
            brand: String::from("PlayStation"),
            model: String::from("5"),
            release_year: 2020,
        },
    ];

    // Bug 1: Using wrong slice method (should use filter)
    let modern_consoles = consoles
        .iter()
        .filter(|c| c.release_year >= 2010)
        .collect::<Vec<&Console>>();
    println!("Modern consoles: {:?}", &modern_consoles);

    // Bug 2: Incorrect string slice usage - trying to modify a string slice
    let brand_slice = &mut consoles[0].brand;
    brand_slice.push_str(" Series S");
    println!("Brand slice: {:?}", brand_slice);

    // Bug 3: Incorrect string slice usage - trying to modify a string slice
    let first_brand: String = consoles[0].brand.clone();
    println!("First brand: {}", first_brand);
}
