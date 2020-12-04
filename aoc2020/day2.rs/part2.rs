fn main() {
    let input = include_str!("day22020.txt");
    let v: Vec<Vec<&str>> = input.split('\n').map(|x: &str| x.split(' ').collect()).collect();
    let mut valid_passwords = 0;

    for thing in v {
        let mut bounds: Vec<&str>;
        bounds = thing[0].split('-').collect();
        let mut pos1: i32 = bounds[0].parse().unwrap();
        let mut pos2: i32 = bounds[1].parse().unwrap();
        pos1 -= 1;
        pos2 -= 1;
        let mut s = String::from(thing[1]);
        s.pop();
        let letter = s;
        let password = thing[2];

        let mut pos1_cond = false;
        let mut pos2_cond = false;

        for (i, c) in password.chars().enumerate() {
            if (i as i32 == pos1) && (c.to_string() == letter) {
                pos1_cond = true;
            }
            if (i as i32 == pos2) && (c.to_string() == letter) {
                pos2_cond = true;
            }
        }
        if (pos1_cond && !pos2_cond) || (!pos1_cond && pos2_cond) {
            valid_passwords += 1;
        }
    }
    println!("{}", valid_passwords);
}
