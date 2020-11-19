fn main() {
  a_function();
  another_function(10);
  let x = five();
  println!("{}", x);
  println!("{}", add(1,2));
}

//function that prints hello world
fn a_function() {
    println!("Hello World");
}

//function with parameters
fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

//function that return a value
fn five() -> i32 {
    5
}

//function with multiple parameters and return
fn add(x: i32, y: i32) -> i32 {
    x+y
}
