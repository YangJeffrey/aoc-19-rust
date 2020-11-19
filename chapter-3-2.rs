fn main() {
    //i32 = 32 bit integer, i64 = 64 bit integer
    //u32 = 32 bit integer without a sign (0 or positive)
    //f64 = 64 bit float

    let x = 2.0; // f64 
    let b: f32 = 3.0; // f32
    println!("{}, {}", x, b);

    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    // remainder
    let remainder = 43 % 5;
    println!("{}, {}, {}, {}, {}", sum, difference, product, quotient, remainder);

    //booleans
    let t = true;
    let f: bool = false; // with explicit type annotation

    //characters
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{}, {}, {}", c, z, heart_eyed_cat);

    //destructuring tuples
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x, y, z is: {}, {}, {}", x, y, z);
    //direct access using .
    let xtup: (i32, f64, u8) = (500, 6.4, 1);
    let first = xtup.0;
    let second = xtup.1;
    let third = xtup.2;
    println!("{}, {}, {}", first, second, third);

    //arrays
    let a = [1, 2, 3, 4, 5];
    //accessing elements
    let afirst = a[0];
    let asecond = a[1];
    println!("{}, {}", afirst, asecond);
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    //[type of each element, number of elements]
    let c: [i32; 5] = [1, 2, 3, 4, 5];
    //[value, number of those values]
    let d = [3; 5];

    //vector
    let mut v = vec![6,7,8,9,10];
    //push value into last index of vec
    v.push(11);
    //remove last index of vec
    v.pop();
    //remove element at index 1 of vec
    v.remove(1);
    //print specific index of vec
    println!("{}", v[v.len()-1]);
    //print entire vec
    println!("{:?}", v);

}
