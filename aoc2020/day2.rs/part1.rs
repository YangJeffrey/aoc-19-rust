fn main() {
    let input = include_str!("day22020.txt");
    let v: Vec<Vec<&str>> = input.split('\n').map(|x: &str| x.split(' ').collect()).collect();
    let mut valid_passwords = 0;

    for thing in v {
        let mut bounds: Vec<&str>;
        bounds = thing[0].split('-').collect();
        let lower: i32 = bounds[0].parse().unwrap();
        let upper: i32 = bounds[1].parse().unwrap();
        println!("{}", lower);
        println!("{}", upper);
        let mut s = String::from(thing[1]);
        s.pop();
        let letter = s;
        println!("{}", letter);
        let password = thing[2];
        println!("{}", password);
        let mut counter = 0;
        for (i, c) in password.chars().enumerate() {
            if c.to_string() == letter {
                counter += 1;
            }
        }
        if counter >= lower && counter <= upper {
            valid_passwords += 1;
        }
    }

    println!("{}", valid_passwords);
}
