use std::io; //use the standard library

fn main() {

    let mut operation = String::new(); //reporting the operation variable
    
    println!("choose an operation");
    println!(
        "1. multiplication
2. addition
3. subtraction
4. division"
    );

    io::stdin().read_line(&mut operation).unwrap(); //read the user input

    let number: i32 = operation.trim().parse().unwrap(); //convert the input value to integer

    let mut x = String::new(); //reporting the x variable

    println!("enter the first number:");
    io::stdin().read_line(&mut x).unwrap(); //read the user input

    let x: f32 = x.trim().parse().unwrap(); //convert the input value to float

    let mut y = String::new(); //reporting the y variable

    println!("enter the second number:");
    io::stdin().read_line(&mut y).unwrap(); //read the user input

    let y: f32 = y.trim().parse().unwrap(); //convert the input value to float

    //run the choosed operation
    if number == 1 {
        println!("the result is :{}", multiplication(x, y))
    }
    if number == 2 {
        println!("the result is :{}", addition(x, y))
    }
    if number == 3 {
        println!("the result is :{}", subtraction(x, y))
    }
    if number == 4 {
        println!("the result is :{}", division(x, y))
    }
}

//todo: add errors handling when i learn it :)

fn multiplication(x: f32, y: f32) -> f32 {
    x * y
}

fn addition(x: f32, y: f32) -> f32 {
    x + y
}

fn subtraction(x: f32, y: f32) -> f32 {
    x - y
}

fn division(x: f32, y: f32) -> f32 {
    x / y
}
