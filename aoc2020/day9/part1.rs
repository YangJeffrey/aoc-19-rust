fn main() {
    let input = include_str!("day9.txt");
    let mut v: Vec<i64> = Vec::new();
    let split = input.split("\n");
    for s in split {
        v.push(s.parse().unwrap());
    }

    for i in 25..v.len() {
        let mut works = false;
        for j in (i-25)..i {
            for k in (i-25)..i {
                if v[j] + v[k] == v[i] {
                    works = true;
                    break;
                }
            }
        }

        if !works {
            println!("{}", v[i]);
        }
    }
}