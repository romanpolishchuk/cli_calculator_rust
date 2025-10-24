use std::io::stdin;

struct Lexer;

impl Lexer {
    fn create_tokens(string: &String) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();

        let mut num = String::new();
        for char in string.chars().filter(|x| *x != ' ') {
            if char.is_ascii_digit() {
                num.push(char);
            } else {
                if !num.is_empty() {
                    result.push(num.clone());
                    num.clear();
                }

                result.push(char.to_string());
            }
        }

        if !num.is_empty() {
            result.push(num);
        }

        result
    }
}

struct Parcer;

impl Parcer {
    fn get_operator_weight(char: &char) -> i32 {
        match char {
            '+' => 1,
            '-' => 1,
            '*' => 2,
            '/' => 2,
            _ => 1,
        }
    }

    fn evaluate(vector: &Vec<String>) -> i32 {
        let vector = vector.clone();
        let mut vector_with_weidth: Vec<(String, i32)> = Vec::new();

        let mut additional_weight = 0;

        for string in vector {
            let char = string.chars().last().unwrap();
            if char == '(' {
                additional_weight += 100;
            } else if char == ')' {
                additional_weight -= 100;
            } else if char.is_ascii_digit() {
                vector_with_weidth.push((string.to_string(), -1));
            } else {
                let char_weight = Self::get_operator_weight(&char) + additional_weight;
                vector_with_weidth.push((char.to_string(), char_weight));
            }
        }

        while vector_with_weidth.len() > 1 {
            let mut max_op_weight = 0;
            let mut max_op_index = 0;

            for (i, char) in vector_with_weidth.iter().enumerate() {
                let char = char.0.chars().last().unwrap();

                if !char.is_ascii_digit() {
                    let char_weight = vector_with_weidth[i].1;
                    if char_weight > max_op_weight {
                        max_op_weight = char_weight;
                        max_op_index = i;
                    }
                }
            }

            let calculation = match vector_with_weidth[max_op_index].0.as_str() {
                "+" => {
                    vector_with_weidth[max_op_index - 1]
                        .0
                        .parse::<i32>()
                        .unwrap()
                        + vector_with_weidth[max_op_index + 1]
                            .0
                            .parse::<i32>()
                            .unwrap()
                }
                "-" => {
                    vector_with_weidth[max_op_index - 1]
                        .0
                        .parse::<i32>()
                        .unwrap()
                        - vector_with_weidth[max_op_index + 1]
                            .0
                            .parse::<i32>()
                            .unwrap()
                }
                "*" => {
                    vector_with_weidth[max_op_index - 1]
                        .0
                        .parse::<i32>()
                        .unwrap()
                        * vector_with_weidth[max_op_index + 1]
                            .0
                            .parse::<i32>()
                            .unwrap()
                }
                "/" => {
                    vector_with_weidth[max_op_index - 1]
                        .0
                        .parse::<i32>()
                        .unwrap()
                        / vector_with_weidth[max_op_index + 1]
                            .0
                            .parse::<i32>()
                            .unwrap()
                }
                _ => 0,
            };

            vector_with_weidth[max_op_index] = (calculation.to_string(), -1);
            vector_with_weidth.remove(max_op_index - 1);
            vector_with_weidth.remove(max_op_index);
        }

        vector_with_weidth[0].0.parse::<i32>().unwrap()
    }
}

fn main() {
    loop {
        let mut input = String::new();

        stdin().read_line(&mut input).unwrap();

        let tokens = Lexer::create_tokens(&(input.trim().to_string()));

        print!("{}\n", Parcer::evaluate(&tokens));
    }
}
