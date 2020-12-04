fn main() {
    let input = include_str!("day4.txt");
    let v: Vec<Vec<&str>> = input.split('\n').map(|x: &str| x.split('\n').collect()).collect();
    let mut validpasswords = 0;
    let mut str = String::from("");

    for thing in v {
        let mut vsub: Vec<&str>;
        vsub = thing[0].split('\n').collect();

        for anotherthing in vsub {
            if anotherthing != "" {
                str = format!("{} {}", str, thing[0]);
            } else {
                let mut isvalid = true;
                if !str.contains("byr") {
                    isvalid = false;
                    println!("{} is missing byr", str);
                } else if !str.contains("iyr") {
                    isvalid = false;
                    println!("{} is missing iyr", str);
                } else if !str.contains("eyr") {
                    isvalid = false;
                    println!("{} is missing eyr", str);
                } else if !str.contains("hgt") {
                    isvalid = false;
                    println!("{} is missing hgt", str);
                } else if !str.contains("hcl") {
                    isvalid = false;
                    println!("{} is missing hcl", str);
                } else if !str.contains("ecl") {
                    isvalid = false;
                    println!("{} is missing ecl", str);
                } else if !str.contains("pid") {
                    isvalid = false;
                    println!("{} is missing pid", str);
                }
                if isvalid == true {
                    validpasswords += 1;
                    println!("{} is valid", str);
                } else {
                    isvalid = true;
                }
                str = String::from("");
            }
        }
    }
    println!("Total {}", validpasswords);
}
