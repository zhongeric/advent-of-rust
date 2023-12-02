fn main() {
    day1();
}

fn day1() {
    let input = include_str!("../inputs/1.txt");
    let mut sum: i32 = 0;
    for line in input.lines() {
        // get all numbers from line
        let numbers: Vec<i32> = line
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|c| c as i32)
            .collect();
        // get first and last number
        let first = numbers.first().unwrap();
        let last = numbers.last().unwrap();
        // concat first and last number
        let concat = format!("{}{}", first, last);
        // convert to i32
        let concat: i32 = concat.parse().unwrap();
        // add to sum
        sum += concat;
    }
    println!("sum: {}", sum);
}
