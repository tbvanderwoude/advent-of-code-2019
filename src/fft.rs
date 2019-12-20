use std::fs;

pub fn read_signal(filename: &String) -> Vec<i32> {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let split: Vec<i32> = contents
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i32)
        .collect();
    return split;
}

pub fn sample_square(phase_width: i32, mut x: i32) -> i32 {
    while x < 0 {
        x += phase_width;
    }
    let phased_x = x % phase_width;
    if phased_x < phase_width / 4 {
        return 0;
    } else if phased_x < phase_width / 2 {
        return 1;
    } else if phased_x < phase_width * 3 / 4 {
        return 0;
    } else {
        return -1;
    }
}

pub fn plot_square(phase_width: i32) {
    for i in 0..16 {
        print!("{} ", sample_square(phase_width, i + 1));
    }
    println!();
}

pub fn sol_1(mut signal: Vec<i32>) {
    for i in 0..100 {
        let old_signal: Vec<i32> = signal.clone();
        for j in 0..old_signal.len() {
            signal[j] = old_signal
                .iter()
                .enumerate()
                .map(|(index, x)| x * sample_square(((j + 1) * 4) as i32, (index + 1) as i32))
                .sum();
            signal[j] = signal[j].abs() % 10;
        }
    }
    println!(
        "{}",
        (&signal)
            .into_iter()
            .map(|x| x.to_string())
            .take(8)
            .collect::<String>()
    );
}

pub fn sol_2(mut signal: Vec<i32>) {
    let n = signal.len();

    let skip: usize = signal.iter().take(7).fold(0, |a, b| a * 10 + *b) as usize;
    let mut long_signal: Vec<i32> = signal.iter().cycle().take(n * 10000).map(|x| *x).collect();

    println!(
        "We can skip {} out of {}: {}%",
        skip,
        n * 10000,
        100 * skip / (n * 10000)
    );
    for i in 0..100 {
        for j in (skip..long_signal.len()).rev() {
            if j != long_signal.len() - 1 {
                long_signal[j] = (long_signal[j] + long_signal[j + 1]) % 10;
            }
        }
    }
    println!(
        "{}",
        (&long_signal)
            .into_iter()
            .skip(skip)
            .take(8)
            .map(|x| x.to_string())
            .collect::<String>()
    );
}
