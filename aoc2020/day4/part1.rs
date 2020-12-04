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
                } else if !str.contains("iyr") {
                    isvalid = false;
                } else if !str.contains("eyr") {
                    isvalid = false;
                } else if !str.contains("hgt") {
                    isvalid = false;
                } else if !str.contains("hcl") {
                    isvalid = false;
                } else if !str.contains("ecl") {
                    isvalid = false;
                } else if !str.contains("pid") {
                    isvalid = false;
                }
                if isvalid == true {
                    validpasswords += 1;
                } else {
                    isvalid = true;
                }
                str = String::from("");
            }
        }
    }
    println!("Total {}", validpasswords);
}
