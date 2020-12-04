fn main() {
    let input = include_str!("day1.txt");
    let mut v: Vec<i32> = Vec::new();
    let split = input.split("\n");
    let mut counter_upper = 0;
    for s in split {
        v.push(s.parse().unwrap());
    }

 for module in v {
     counter_upper += fuel_required(module);
 }

 println!("{}", counter_upper);
}

fn fuel_required(x: i32) -> i32 {
    x/3-2
}
