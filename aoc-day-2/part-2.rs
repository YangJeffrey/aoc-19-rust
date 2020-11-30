fn main() {
    let input = include_str!("day2.txt");
    let output = 19690720;

    for noun in 0..100 {
        for verb in 0..100 {

            let counter_upper = 0;

            let mut v: Vec<i32> = Vec::new();
            let split = input.split(",");
            for s in split {
                v.push(s.parse().unwrap());
            }

            v[1] = noun;
            v[2] = verb;

            let v_clone = v.clone();

            for i in (0..v.len()).step_by(4) {

                if v[i] == 1 {
                    v[v_clone[i+3] as usize] = v[v_clone[i+1] as usize] + v[v_clone[i+2] as usize];
                } else if v[i] == 2 {
                    v[v_clone[i+3] as usize] = v[v_clone[i+1] as usize] * v[v_clone[i+2] as usize];
                } else {
                    break;
                }
            }

            if v[0] == output {
                println!("{}", 100*noun+verb);
            }
        }
    }
}
