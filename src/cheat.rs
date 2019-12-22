use std::time::Instant;
use mod_exp::mod_exp;

#[derive(Clone, Copy)]
enum Cmd { Deal(i128), Cut(i128), Stack }

static INPUT: [Cmd; 100] = [Cmd::Deal(34), Cmd::Cut(9781), Cmd::Deal(20), Cmd::Cut(8981), Cmd::Deal(11), Cmd::Cut(-3391), Cmd::Deal(15), Cmd::Cut(1485), Cmd::Deal(10), Cmd::Cut(4826), Cmd::Stack, Cmd::Cut(1026), Cmd::Deal(30), Cmd::Cut(1354), Cmd::Deal(46), Cmd::Cut(1955), Cmd::Deal(19), Cmd::Cut(1359), Cmd::Deal(22), Cmd::Cut(9483), Cmd::Deal(52), Cmd::Cut(-2090), Cmd::Deal(50), Cmd::Stack, Cmd::Cut(-2205), Cmd::Deal(69), Cmd::Cut(-7934), Cmd::Deal(11), Cmd::Cut(8311), Cmd::Deal(42), Cmd::Cut(-5430), Cmd::Deal(57), Cmd::Stack, Cmd::Cut(-2616), Cmd::Deal(22), Cmd::Stack, Cmd::Cut(3540), Cmd::Deal(38), Cmd::Cut(-9097), Cmd::Deal(37), Cmd::Cut(-7014), Cmd::Deal(26), Cmd::Cut(6983), Cmd::Deal(11), Cmd::Stack, Cmd::Cut(-4825), Cmd::Stack, Cmd::Cut(-5791), Cmd::Deal(19), Cmd::Cut(-3577), Cmd::Deal(6), Cmd::Stack, Cmd::Deal(29), Cmd::Cut(7299), Cmd::Deal(75), Cmd::Cut(-8498), Cmd::Deal(21), Cmd::Cut(5748), Cmd::Deal(63), Cmd::Cut(-344), Cmd::Deal(5), Cmd::Cut(-4306), Cmd::Deal(65), Cmd::Cut(9431), Cmd::Deal(7), Cmd::Cut(6825), Cmd::Deal(28), Cmd::Stack, Cmd::Deal(66), Cmd::Cut(-1421), Cmd::Deal(19), Cmd::Cut(-8965), Cmd::Deal(48), Cmd::Cut(-5780), Cmd::Deal(75), Cmd::Cut(-3280), Cmd::Deal(50), Cmd::Cut(6866), Cmd::Deal(72), Cmd::Cut(-5471), Cmd::Deal(49), Cmd::Cut(-8247), Cmd::Deal(65), Cmd::Cut(3056), Cmd::Stack, Cmd::Deal(39), Cmd::Cut(7011), Cmd::Deal(48), Cmd::Cut(-9660), Cmd::Deal(56), Cmd::Cut(-6843), Cmd::Stack, Cmd::Cut(5111), Cmd::Deal(29), Cmd::Cut(-7700), Cmd::Stack, Cmd::Deal(23), Cmd::Cut(-5263), Cmd::Deal(61), Cmd::Stack];
const LEN: i128 = 10007;
const M: i128 = 119_315_717_514_047;
const N: i128 = 101_741_582_076_661;

// Convert to a linear equation ax + b
fn to_linear_equation(input: &[Cmd]) -> (i128, i128) {
    let mut a = 1;
    let mut b = 0;
    for &cmd in input.iter().rev() {
        match cmd {
            Cmd::Deal(n) => {
                let n = mod_exp(n, M-2, M);
                a *= n;
                b *= n;
            },
            Cmd::Cut(n) => b += n + M,
            Cmd::Stack => b = M - 1 - b,
        }
        a %= M;
        b %= M;
    }
    (a,b)
}

fn part_two() -> i128 {
    let (a,b) = to_linear_equation(&INPUT);

    // Applying the function n times simplifies to:
    // x * a^n + b * (a^n - 1) / (a-1)
    let term1 = 2020 * mod_exp(a, N, M) % M;
    let tmp = (mod_exp(a, N, M) - 1) * mod_exp(a - 1, M-2, M) % M;
    let term2 = b * tmp % M;
    (term1 + term2) % M
}

fn part_one() -> i128 {
    INPUT.iter().fold(2019, |pos, &cmd| match cmd {
        Cmd::Deal(n) => (pos * n) % LEN,
        Cmd::Cut(n)  => (pos - n) % LEN,
        Cmd::Stack   => LEN - 1 - pos,
    })
}

fn main() {
    let now = Instant::now();
    let part_one = part_one();
    let part_two = part_two();
    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);
    println!("Time: {}ms", now.elapsed().as_millis());
}