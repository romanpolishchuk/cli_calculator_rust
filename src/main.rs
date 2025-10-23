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

fn main() {
    let test_string = String::from("10 + 20 * 30 + 400 * 5000 + 60000");

    let tokens = Lexer::create_tokens(&test_string);

    print!("{tokens:?}\n");
}
