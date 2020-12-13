fn main() {
    let input = include_str!("day8.txt");
    let mut v: Vec<Vec<&str>> = input.split("\n").map(|x: &str| x.split(" ").collect()).collect();
    let mut accumulator = 0;
    let mut ranthru: Vec<i32> = vec![1000];

    let mut i= 0;

    while !ranthru.contains(&i) {
            if v[i as usize][0] == "acc" {
                ranthru.push(i);
                accumulator += v[i as usize][1].replace("+", "").parse::<i32>().unwrap();
            } else if v[i as usize][0] == "nop" {
                ranthru.push(i);
            }

        if v[i as usize][0] == "jmp" {
            ranthru.push(i);
            i += v[i as usize][1].replace("+", "").parse::<i32>().unwrap();
        } else {
            i += 1;
        }
    }
    println!("{}", accumulator);
}