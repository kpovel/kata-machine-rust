fn main() {
    let mut data = [false; 10000];

    for x in 3043..data.len() {
        data[x] = true;
    }

    let res = two_crystal_balls(&data);
    match res {
        Some(v) => println!("{}", v),
        None => println!("Error"),
    };
}

fn two_crystal_balls(breaks: &[bool]) -> Option<u32> {
    let len = breaks.len() as f64;
    let jmp_amount = len.sqrt().floor() as u32;

    let mut i = jmp_amount;
    for _ in 0..jmp_amount {
        if breaks[i as usize] {
            break;
        }

        i += jmp_amount;
    }

    i -= jmp_amount;

    for _ in 0..jmp_amount {
        if breaks[i as usize] {
            return Some(i);
        }

        i += 1;
    }

    None
}

