#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut values = Vec::new();

    for elem in inputs {
        match elem {
            CalculatorInput::Add => {
                let y = values.pop()?;
                let x = values.pop()?;

                values.push(x + y);
            }
            CalculatorInput::Subtract => {
                let y = values.pop()?;
                let x = values.pop()?;

                values.push(x - y);
            }
            CalculatorInput::Multiply => {
                let y = values.pop()?;
                let x = values.pop()?;

                values.push(x * y);
            }
            CalculatorInput::Divide => {
                let y = values.pop()?;
                let x = values.pop()?;

                values.push(x / y);
            }
            CalculatorInput::Value(x) => values.push(*x),
        }
    }

    if values.len() == 1 {
        values.pop()
    } else {
        None
    }
}
