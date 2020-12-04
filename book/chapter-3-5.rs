fn main() {

    //if else statement
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // if else if
    let anothernumber = 6;
    if anothernumber % 4 == 0 {
        println!("number is divisible by 4");
    } else if anothernumber % 3 == 0 {
        println!("number is divisible by 3");
    } else if anothernumber % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    //if statement in let
    let condition = true;
    let yetanothernumber = if condition { 5 } else { 6 };
    println!("The value of number is: {}", yetanothernumber);

    //while loop
    let mut countdownnum = 5;
    while countdownnum > 0 {
        println!("{}!", countdownnum);
        countdownnum -= 1;
    }
    println!("LIFTOFF!!!");

    //for loop
    for i in 0..10 {
      println!("{}", i);
    }

    //reversed for loop
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    //for loop for iterating through array
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
