pub struct WordProblem {
    cmd: String,
}

#[derive(PartialEq, Debug)]
enum Token {
    CmdWhat,
    CmdIs,
    Number,
    Operator,
    OperatorBy,
}

enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
}

impl WordProblem {
    pub fn new(command: &str) -> WordProblem {
        WordProblem {
            cmd: command.to_string(),
        }
    }

    pub fn answer(&self) -> Result<i32, String> {
        let command = self.cmd
            .split(|x| x == '?' || char::is_whitespace(x))
            .filter(|w| !w.is_empty())
            .collect::<Vec<_>>();

        let mut result: i32 = 0;
        let mut lastop = Operator::Plus;
        let mut status = Token::CmdWhat;

        for word in command {
            match word {
                "What" if status == Token::CmdWhat => status = Token::CmdIs,
                "is" if status == Token::CmdIs => status = Token::Number,
                "plus" if status == Token::Operator => {
                    lastop = Operator::Plus;
                    status = Token::Number
                }
                "minus" if status == Token::Operator => {
                    lastop = Operator::Minus;
                    status = Token::Number
                }
                "multiplied" if status == Token::Operator => {
                    lastop = Operator::Multiply;
                    status = Token::OperatorBy
                }
                "divided" if status == Token::Operator => {
                    lastop = Operator::Divide;
                    status = Token::OperatorBy
                }
                "by" if status == Token::OperatorBy => status = Token::Number,
                _ if status == Token::Number => {
                    let value: i32;
                    if let Ok(v) = word.parse::<i32>() {
                        value = v;
                    } else {
                        return Err("Invalid number".to_string());
                    }

                    match lastop {
                        Operator::Plus => result += value,
                        Operator::Minus => result -= value,
                        Operator::Multiply => result *= value,
                        Operator::Divide => result /= value,
                    }

                    status = Token::Operator
                }
                _ => return Err("Invalid command".to_string()),
            }
        }

        Ok(result)
    }
}
