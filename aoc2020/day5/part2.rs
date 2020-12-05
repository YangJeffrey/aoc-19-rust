fn main() {
    let input = include_str!("day5.txt");
    let split = input.split("\n");
    let v: Vec<&str> = split.collect();
    let mut highestseatid = 0;
    let mut lowestseatid = 1000;
    let mut seatids: Vec<i32> = Vec::new();

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

        if rowlower * 8 + columnlower < lowestseatid {
            lowestseatid = rowlower * 8 + columnlower;
        }
        seatids.push(rowlower * 8 + columnlower);
    }

    for i in lowestseatid..highestseatid {
        let mut numfound = false;
        for j in 0..seatids.len() {
            if seatids[j] == i {
                numfound = true;
            }
        }
        if numfound == false {
            println!("{}", i);
        }
    }
}