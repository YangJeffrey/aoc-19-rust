fn main() {
    let input = include_str!("day6.txt");
    let split = input.split("\n");
    let v: Vec<&str> = split.collect();
    let alphabet: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
    let mut vsub: Vec<&str> = vec![""];
    let mut answers: Vec<char> = vec![' '];
    let mut total = 0;

    for thing in v {
        if thing != "" {
            vsub.push(thing);
        } else {
            for subthing in vsub {
                for (i, c) in subthing.chars().enumerate() {
                    answers.push(c);
                }
            }
            for letter in &alphabet {
                if answers.contains(letter) {
                    total += 1;
                }
            }
            vsub = vec![""];
            answers = vec![' '];
        }
    }

    println!("{}", total);
}