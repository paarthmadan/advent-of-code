fn main() {
    println!("{}", pt_2());
}

fn pt_2() -> u32 {
    const LOWER_RANGE: u32 = 307237;
    const UPPER_RANGE: u32 = 769058;

    let mut ok_pattern_count = 0;

    'guess: for guess in LOWER_RANGE..=UPPER_RANGE {
        let mut prev = 'x';
        let mut count: Vec<u32> = Vec::new();
        let mut currently_parsing_group = false;
        let mut number_of_groups: usize = 0;

        for digit in guess.to_string().chars() {
            if prev == 'x' { 
                prev = digit; 
                continue;
            }
            if prev > digit { continue 'guess; }
            let same_digit = prev == digit;
            match (currently_parsing_group, same_digit) {
                (false, true) => new_group(&mut currently_parsing_group, &mut count),
                (true, true) => same_group(&mut count, &number_of_groups),
                (true, false) => end_group(&mut currently_parsing_group, &mut number_of_groups),
                (false, false) => {}, 
            }
            prev = digit;
        }
        
        if count.into_iter().any(|x| x == 2) {
            ok_pattern_count += 1;
        }
    }
    ok_pattern_count
}

fn same_group(count: &mut Vec<u32>, number_of_groups: &usize) {
    count[*number_of_groups] += 1;
}

fn end_group(currently_parsing_group: &mut bool, number_of_groups: &mut usize) {
    *currently_parsing_group = false;
    *number_of_groups += 1;
}

fn new_group(currently_parsing_group: &mut bool, count: &mut Vec<u32>) {
    *currently_parsing_group = true;
    count.push(2);
}
