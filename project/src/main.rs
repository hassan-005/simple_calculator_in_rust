enum Operation {
    Addition,
    Subtraction,
    Multiply,
    Division,
}
fn calculate (operator: Operation, num1: f64, num2: f64) -> f64 {

    match operator{
        Operation::Addition =>   num1 + num2,
        Operation::Subtraction =>   num1 - num2,
        Operation::Multiply =>   num1 * num2,
        Operation::Division =>   num1 / num2,
    }  
    
}



use std::io::stdin ;

fn main() {
    let mut number1= String::new();
    let mut number2= String::new();
    let mut operator = String::new();

    println!("Enter the first number: ");
    stdin().read_line(&mut number1).expect("error");

    println!("Enter the Operation symbol: ");
    stdin().read_line(&mut operator).expect("error");

    println!("Enter the second number: ");
    stdin().read_line(&mut number2).expect("error");

    


    let num1:f64 = match number1.trim().parse(){
        Ok(num) => num,
        Err(_) =>  return
    };


    let num2 : f64 = match number2.trim().parse(){
        Ok(num) =>num,
        Err(_) => return
    };



    let operation: Operation  = match operator.trim() {
        "+" => Operation::Addition,
        "-" => Operation::Subtraction,
        "*" => Operation::Multiply,
        "/" =>  Operation::Division,
        _ => {println!("Invalid error. Defaulting to addition.");
                Operation::Addition
            } 
    };

    let result = calculate( operation, num1, num2);
    println!("result: {}", result);
 
}
