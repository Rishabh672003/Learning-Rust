use std::io;

fn math() {
    let z = 5;
    let a = z + 1;
    let a = a * 2;
    println!("The value of a is: {a}");
}

fn shadow() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

fn int_types() {
    let alpha: u32 = 5;
    let beta: u32 = 12;
    let gamma = alpha + beta;
    println!("{gamma}");

    let zeta = 2.9;
    let theta: f64 = 9.88;
    let delta = zeta + theta;
    println!("{delta}");
}

fn basic_math() {
    // addition
    let sum = 5 + 10;
    println!("{sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("{difference}");

    // multiplication
    let product = 4 * 30;
    println!("{product}");

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0
    println!("{quotient}, {floored}");

    // remainder
    let remainder = 43 % 5;
    print!("{remainder}");
}
fn chars() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    // print all variable in the function
    println!("{c}, {z}, {heart_eyed_cat}");
}

fn bool() {
    let s: bool = true;
    println!("{s}");
}

fn tuple() {
    let tup = (500, 6.4, 1);

    let y = tup.1;

    println!("The value of y is: {y}");
}
fn array() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
fn main() {
    math();
    shadow();
    int_types();
    basic_math();
    chars();
    bool();
    tuple();
    array();
}
