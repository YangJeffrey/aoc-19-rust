fn main() {
    let input = include_str!("day8.txt");
    let mut v: Vec<Vec<&str>> = input.split("\n").map(|x: &str| x.split(" ").collect()).collect();

    for iter in 0..v.len() {
        if v[iter][0] == "nop" {
            v[iter][0] = "jmp";
            let mut accumulator = 0;
            let mut ranthru: Vec<i32> = vec![];
            let mut i= 0;

            while !ranthru.contains(&i) && i < v.len() as i32 {
                if v[i as usize][0] == "acc" {
                    ranthru.push(i);
                    accumulator += v[i as usize][1].replace("+", "").parse::<i32>().unwrap();
                    i += 1;
                } else if v[i as usize][0] == "nop" {
                    ranthru.push(i);
                    i += 1;
                } else if v[i as usize][0] == "jmp" {
                    ranthru.push(i);
                    i += v[i as usize][1].replace("+", "").parse::<i32>().unwrap();
                }
            }

            if i as usize == v.len() {
                println!("{}", accumulator);
            }
            v[iter][0] = "nop";

        } else if v[iter][0] == "jmp" {
            v[iter][0] = "nop";
            let mut accumulator = 0;
            let mut ranthru: Vec<i32> = vec![];
            let mut i= 0;

            while !ranthru.contains(&i) && i < v.len() as i32 {
                if v[i as usize][0] == "acc" {
                    ranthru.push(i);
                    accumulator += v[i as usize][1].replace("+", "").parse::<i32>().unwrap();
                    i += 1;
                } else if v[i as usize][0] == "nop" {
                    ranthru.push(i);
                    i += 1;
                } else if v[i as usize][0] == "jmp" {
                    ranthru.push(i);
                    i += v[i as usize][1].replace("+", "").parse::<i32>().unwrap();
                }
            }

            if i as usize == v.len() {
                println!("{}", accumulator);
            }
            v[iter][0] = "jmp";
        }
    }
}