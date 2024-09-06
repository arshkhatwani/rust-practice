use clap::Parser;

#[derive(Parser)]
struct Cli {
    operation: String,
    num1: f32,
    num2: f32,
}

fn calculate(operation: &str, num1: &f32, num2: &f32) -> f32 {
    let result = match operation {
        "add" => num1 + num2,
        "sub" => num1 - num2,
        "mul" => num1 * num2,
        "div" => num1 / num2,
        _ => {
            println!("Invalid operation");
            return 0.0;
        }
    };
    return result;
}

fn main() {
    let args = Cli::parse();
    let operation = &args.operation;
    let num1 = &args.num1;
    let num2 = &args.num2;

    println!("operation: {}, num1: {}, num2: {}", operation, num1, num2);

    let result = calculate(&operation, &num1, &num2);
    println!("Result: {}", result);
}
