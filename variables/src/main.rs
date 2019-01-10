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
    testing_basic_math();
    cool_it_got_rid_of_shitty_ass_ascii();
    tuple_test();

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

fn testing_basic_math() {
    println!("Sum of the ints: {}", 5 + 10);
    println!("Dif of the floats: {}", 95.5 - 4.3);
    println!("Product of ints {}", 4 * 30);
    println!("Quotient of floats {}", 56.7 / 32.2);
    println!("remainder of ints {}", 43 % 5);
}

fn cool_it_got_rid_of_shitty_ass_ascii() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("Cat with heart {}",heart_eyed_cat);
}


fn tuple_test() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("Ok print my tuple {} {} {}", x, y, z );
    println!("Ok print my tuple with dot optr {} {} {}", tup.0, tup.1, tup.2);
}