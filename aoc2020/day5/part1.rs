fn main() {
    let input = include_str!("day5.txt");
    let split = input.split("\n");
    let v: Vec<&str> = split.collect();
    let mut highestseatid = 0;

    for seat in v {
        let mut rowlower = 0;
        let mut rowupper = 127;
        let mut columnlower = 0;
        let mut columnupper = 7;


        for (i, c) in seat.chars().enumerate() {
            if c == 'F' {
                rowupper -= (rowupper - rowlower)/2 + 1;
            } else if c == 'B' {
                rowlower += (rowupper - rowlower)/2 + 1;
            } else if c == 'L' {
                columnupper -= (columnupper - columnlower + 1)/2;
            } else if c == 'R' {
                columnlower += (columnupper - columnlower + 1)/2;
            }
        }

        if rowlower * 8 + columnlower > highestseatid {
            highestseatid = rowlower * 8 + columnlower;
        }
    }
    println!("{}", highestseatid);

}