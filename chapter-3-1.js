fn main() {
    //mutable variables and printing variables
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    //constants
    const MAX_POINTS: u32 = 100_000;
    println!("{}", MAX_POINTS);

    //shadowing 
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("{}", y);

    //example where shadowing works and mut doesnt
    let spaces = "   ";
    let spaces = spaces.len();
    println!("{}", spaces);
}
