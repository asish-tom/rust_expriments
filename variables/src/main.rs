fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);
    shadowing_test();
    shadowing_test_1();
    shadowing_vs_mutation();
    understanding_floating_points();

}


fn shadowing_test() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
}


fn shadowing_test_1() {
    let spaces = "   ";
    let spaces = spaces.len();

    println!("The space between words {}", spaces)
}

fn shadowing_vs_mutation() {
    let mut spaces = "   ";
//    spaces = spaces.len();
    println!("The space between words {}", spaces)
}

fn understanding_floating_points() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("Sum of the floats: {}", x+y);

}