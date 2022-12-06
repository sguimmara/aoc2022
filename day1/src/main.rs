fn sums(groups: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut result : Vec<i32> = Vec::new();
    for group in groups  {
       let sum = group.into_iter().sum();
        result.push(sum);
    }

    result
}

fn parse(path: &str) -> Vec<Vec<i32>> {
    let bytes = std::fs::read(path).unwrap();
    let text = std::str::from_utf8(&bytes).unwrap();

    let mut result: Vec<Vec<i32>> = Vec::new();

    let mut current_group : Vec<i32> = Vec::new();

    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.len() == 0 {
            result.push(current_group);
            current_group = Vec::new();
        } else {
            let number : i32 = trimmed.parse().unwrap();
            current_group.push(number);
        }
    }

    if !current_group.is_empty() {
        result.push(current_group);
    }

    result
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = &args[1];

    let parsed = parse(input);
    let mut s = sums(&parsed);

    // println!("max is {}", s.into_iter().max().unwrap());

    s.sort();

    let len = s.len();

    let sum_of_three = s.get(len - 1).unwrap() + s.get(len - 2).unwrap() + s.get(len - 3).unwrap();

    println!("the sum of the biggest three is {}", sum_of_three);
}
