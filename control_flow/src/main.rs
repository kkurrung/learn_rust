fn main() {
    test_if();
    test_if_2(26);
    test_return_value_from_loop();
    test_loop_lable();
    test_while();
    test_for();
    test_for_2();
}

fn test_if() {
    let value_ = 3;

    if value_ < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn test_if_2( value_ : i32 ) {

    if value_ % 4 == 0 {
        println!("value_ is divisible by 4");
    } else if value_ % 3 == 0 {
        println!("value_ is divisible by 3");
    } else if value_ % 2 == 0 {
        println!("value_ is divisible by 2");
    } else {
        println!("value_ is not divisible by 4, 3, or 2");
    }
}

fn test_return_value_from_loop()
{
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn test_loop_lable() {
    println!("\ntest_loop_lable()");
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn test_while() {
    println!("\ntest_while()");
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}


fn test_for() {
    println!("\ntest_for()");
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn test_for_2() {
    println!("\ntest_for_2()");
    // for number in (1..4).rev() {
    for number in 1..4 {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
