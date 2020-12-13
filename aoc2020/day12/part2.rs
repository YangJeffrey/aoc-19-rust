fn main() {
    let input = include_str!("day12.txt");
    let split = input.split("\n");
    let v: Vec<&str> = split.collect();

    let mut x = 10;
    let mut y = 1;
    let mut shipx: i32 = 0;
    let mut shipy: i32 = 0;

    for thing in v {
        if thing.chars().nth(0).unwrap() == 'N' {
            let removedfirst = thing.chars().next().map(|c| &thing[c.len_utf8()..]);
            y += removedfirst.unwrap().parse::<i32>().unwrap();
            println!("x = {}, y = {}, shipx = {}, shipy = {}", x, y, shipx, shipy);
        } else if thing.chars().nth(0).unwrap() == 'S' {
            let removedfirst = thing.chars().next().map(|c| &thing[c.len_utf8()..]);
            y -= removedfirst.unwrap().parse::<i32>().unwrap();
            println!("x = {}, y = {}, shipx = {}, shipy = {}", x, y, shipx, shipy);
        } else if thing.chars().nth(0).unwrap() == 'E' {
            let removedfirst = thing.chars().next().map(|c| &thing[c.len_utf8()..]);
            x += removedfirst.unwrap().parse::<i32>().unwrap();
            println!("x = {}, y = {}, shipx = {}, shipy = {}", x, y, shipx, shipy);
        } else if thing.chars().nth(0).unwrap() == 'W' {
            let removedfirst = thing.chars().next().map(|c| &thing[c.len_utf8()..]);
            x -= removedfirst.unwrap().parse::<i32>().unwrap();
            println!("x = {}, y = {}, shipx = {}, shipy = {}", x, y, shipx, shipy);
        } else if thing.chars().nth(0).unwrap() == 'F' {
            let removedfirst = thing.chars().next().map(|c| &thing[c.len_utf8()..]);
            for i in 0..removedfirst.unwrap().parse::<i32>().unwrap() {
                shipx += x;
                shipy += y;
            }
            println!("x = {}, y = {}, shipx = {}, shipy = {}", x, y, shipx, shipy);
        } else if thing.chars().nth(0).unwrap() == 'L' {
            let removedfirst = thing.chars().next().map(|c| &thing[c.len_utf8()..]);
            if removedfirst.unwrap().parse::<i32>().unwrap() == 90 {
                let tempy = y;
                y = x;
                x = -tempy;
            } else if removedfirst.unwrap().parse::<i32>().unwrap() == 180 {
                y = -y;
                x = -x;
            } else if removedfirst.unwrap().parse::<i32>().unwrap() == 270 {
                let tempy = y;
                y = -x;
                x = tempy;
            }
            println!("x = {}, y = {}, shipx = {}, shipy = {}", x, y, shipx, shipy);
        } else if thing.chars().nth(0).unwrap() == 'R' {
            let removedfirst = thing.chars().next().map(|c| &thing[c.len_utf8()..]);
            if removedfirst.unwrap().parse::<i32>().unwrap() == 90 {
                let tempy = y;
                y = -x;
                x = tempy;
            } else if removedfirst.unwrap().parse::<i32>().unwrap() == 180 {
                y = -y;
                x = -x;
            } else if removedfirst.unwrap().parse::<i32>().unwrap() == 270 {
                let tempy = y;
                y = x;
                x = -tempy;
            }
            println!("x = {}, y = {}, shipx = {}, shipy = {}", x, y, shipx, shipy);
        }
    }
    println!("{}", shipx.abs() + shipy.abs());
    //39145 too high
}