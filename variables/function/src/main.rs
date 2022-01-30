fn main() {
    println!("Hello, world!");
    another_function();
    another_function_with_param(5);
    another_function_with_two_param(5,6);
    
    let _x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);

    let five = five();
    println!("The value of five is: {}", five);

    let six = plus_one(5);
    println!("The value of six is: {}", six)
}

fn another_function() {
    println!("Another function.");
}

fn another_function_with_param(x: i32) {
    println!("Another function with param: {}", x);
}

fn another_function_with_two_param(x: i32, y: i32) {
    println!("Another function with two param: {}, {}", x, y);
}

fn five() -> i32{
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}