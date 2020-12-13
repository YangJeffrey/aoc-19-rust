fn main() {
    let input = include_str!("day12.txt");
    let split = input.split("\n");
    let v: Vec<&str> = split.collect();

    let mut x = 0;
    let mut y = 0;
    let mut direction = "e";

    for thing in v {
        if thing.chars().nth(0).unwrap() == 'N' {
            let removedfirst = thing.chars().next().map(|c| &thing[c.len_utf8()..]);
            y += removedfirst.unwrap().parse::<i32>().unwrap();
        } else if thing.chars().nth(0).unwrap() == 'S' {
            let removedfirst = thing.chars().next().map(|c| &thing[c.len_utf8()..]);
            y -= removedfirst.unwrap().parse::<i32>().unwrap();
        } else if thing.chars().nth(0).unwrap() == 'E' {
            let removedfirst = thing.chars().next().map(|c| &thing[c.len_utf8()..]);
            x += removedfirst.unwrap().parse::<i32>().unwrap();
        } else if thing.chars().nth(0).unwrap() == 'W' {
            let removedfirst = thing.chars().next().map(|c| &thing[c.len_utf8()..]);
            x -= removedfirst.unwrap().parse::<i32>().unwrap();
        } else if thing.chars().nth(0).unwrap() == 'F' {
            let removedfirst = thing.chars().next().map(|c| &thing[c.len_utf8()..]);
            if direction == "n" {
                y += removedfirst.unwrap().parse::<i32>().unwrap();
            } else if direction == "s" {
                y -= removedfirst.unwrap().parse::<i32>().unwrap();
            } else if direction == "e" {
                x += removedfirst.unwrap().parse::<i32>().unwrap();
            } else if direction == "w" {
                x -= removedfirst.unwrap().parse::<i32>().unwrap();
            }
        } else if thing.chars().nth(0).unwrap() == 'L' {
            let removedfirst = thing.chars().next().map(|c| &thing[c.len_utf8()..]);
            if removedfirst.unwrap().parse::<i32>().unwrap() == 90 {
                if direction == "n" {
                    direction = "w";
                } else if direction == "s" {
                    direction = "e";
                } else if direction == "e" {
                    direction = "n";
                } else if direction == "w" {
                    direction = "s";
                }
            } else if removedfirst.unwrap().parse::<i32>().unwrap() == 180 {
                if direction == "n" {
                    direction = "s";
                } else if direction == "s" {
                    direction = "n";
                } else if direction == "e" {
                    direction = "w";
                } else if direction == "w" {
                    direction = "e";
                }
            } else if removedfirst.unwrap().parse::<i32>().unwrap() == 270 {
                if direction == "s" {
                    direction = "w";
                } else if direction == "n" {
                    direction = "e";
                } else if direction == "e" {
                    direction = "s";
                } else if direction == "w" {
                    direction = "n";
                }
            }
        } else if thing.chars().nth(0).unwrap() == 'R' {
            let removedfirst = thing.chars().next().map(|c| &thing[c.len_utf8()..]);
            if removedfirst.unwrap().parse::<i32>().unwrap() == 90 {
                if direction == "s" {
                    direction = "w";
                } else if direction == "n" {
                    direction = "e";
                } else if direction == "e" {
                    direction = "s";
                } else if direction == "w" {
                    direction = "n";
                }
            } else if removedfirst.unwrap().parse::<i32>().unwrap() == 180 {
                if direction == "n" {
                    direction = "s";
                } else if direction == "s" {
                    direction = "n";
                } else if direction == "e" {
                    direction = "w";
                } else if direction == "w" {
                    direction = "e";
                }
            } else if removedfirst.unwrap().parse::<i32>().unwrap() == 270 {
                if direction == "n" {
                    direction = "w";
                } else if direction == "s" {
                    direction = "e";
                } else if direction == "e" {
                    direction = "n";
                } else if direction == "w" {
                    direction = "s";
                }
            }
        }
    }
    println!("{}", x.abs() + y.abs());
}