fn main() {
    let _number = 3;
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number was something other than zero");
    }

    if number % 2 == 0 {
        println!("number is divisible by two");
    } else if number % 3 == 0 {
        println!("number is divisible by three");
    } else {
        println!("number is not divisible by two or three");
    }
    let mut num = 3;
    while num != 0 {
        println!("{}!", num);
        num = num - 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index = index + 1;
    }

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
