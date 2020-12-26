pub fn parse_numbers(data: String) -> Vec<i32> {
    let split = data.split("\n");
    let mut weights: Vec<i32> = vec![];
    for s in split {
        weights.push(s.to_string().parse::<i32>().unwrap());
    }
    return weights;
}
