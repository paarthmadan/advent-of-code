fn main() {
    println!("{}", pt_1());
}

fn pt_1() -> u32 {
    const LOWER_RANGE: u32 = 307237;
    const UPPER_RANGE: u32 = 769058;

    let mut ok_pattern_count = 0;

    'guess: for guess in LOWER_RANGE..=UPPER_RANGE {
        let mut contains_adjacent_pair = false;
        let mut prev: Option<char> = None;
        for digit in guess.to_string().chars() {
            match prev {
                Some(p) => {
                    if p > digit { continue 'guess; }
                    if p == digit { contains_adjacent_pair = true };
                    prev = Some(digit);
                },
                None => prev = Some(digit),
            }
        }
        if contains_adjacent_pair { 
            ok_pattern_count += 1; 
        }
    }
    ok_pattern_count
}
