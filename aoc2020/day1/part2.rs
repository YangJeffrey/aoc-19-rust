fn main() {
    let input = include_str!("day1.txt");
    let mut v: Vec<i32> = Vec::new();
    let split = input.split("\n");
    for s in split {
        v.push(s.parse().unwrap());
    }
    let v_clone = v.clone();
    let v_another_clone = v.clone();

    for i in &v {
        for j in &v_clone {
            for k in &v_another_clone {
                if i + j + k == 2020 {
                    println!("{}", i * j * k);
                }
            }
        }
    }
}
