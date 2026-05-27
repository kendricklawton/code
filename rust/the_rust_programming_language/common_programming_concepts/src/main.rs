fn main() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("Guess: {guess}");

    let x_f64 = 2.0;
    println!("x_f64: {x_f64}");

    let y_f32: f32 = 3.0;
    println!("y_f32: {y_f32}");

    let sum = 5 + 10;
    println!("sum: {sum}");

    let difference = 95.5 - 4.3;
    println!("difference: {difference}");

    let product = 4 * 30;
    println!("product: {product}");

    let quotient = 56.7 / 32.3;
    println!("quotient: {quotient}");

    let truncated = -5 / 3;
    println!("truncated: {truncated}");

    let reminder = 43 % 5;
    println!("Reminder: {reminder}");

    let t_bool = true;
    let f_bool: bool = false;

    println!("Booleans: T:{t_bool}, F:{f_bool}");

    let c_char = 'z';
    let z_char: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("C Char: {c_char}, Z Char: {z_char}, Heart Eyed Cat: {heart_eyed_cat}");

    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x_tup, y_tup, z_tup) = tup;
    println!("The value of X_TUP is {x_tup}");
    println!("The value of Y_TUP is: {y_tup}");
    println!("The value of Z_TUP is: {z_tup}");

    let tup_x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = tup_x.0;
    let six_point_four = tup_x.1;
    let one = tup_x.2;

    println!("Five Hunderd: {five_hundred}, Six Point Four: {six_point_four}, One: {one}");

    let a_array = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let first_month = months[0];
    println!("Months[0]: {first_month}");

    let a_array_two: [i32; 5] = [1, 2, 3, 4, 5];

    // let a_array_three = [3,3,3,3,3];
    let a_array_three = [3; 5];

    let first_a_array = a_array[0];
    let second_a_array = a_array[1];
    let first_a_array_two = a_array_two[0];
    let second_a_array_two = a_array_two[1];
    let first_a_array_three = a_array_three[0];
    let secound_a_array_three = a_array_three[1];

    println!(
        "
        First A Array: {first_a_array},
        Second A Array: {second_a_array},
        First A Array Two {first_a_array_two},
        Second A Array Two {second_a_array_two},
        First A Array Three {first_a_array_three},
        Second A Array Three {secound_a_array_three},
        "
    );

    another_function_01();
    another_function_02(5);
    print_labeled_measurement(5, 'h');

    // Expressions

    let y_expression = {
        let x = 3;
        x + 1
    };

    println!("The value of y_expression is: {y_expression}");

    let five_x = five();
    println!("five_x: {five_x}");

    let plus_one = plus_one(5);
    println!("plus_one: {plus_one}");

    // Control Flow
    let number = 5;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number_two = 3;

    if number_two != 0 {
        println!("number_two was something other than zero");
    }

    let number_three = 6;

    if number_three % 4 == 0 {
        println!("number_three is divisible by 4");
    } else if number_three % 3 == 0 {
        println!("number_three is divisible by 3");
    } else if number_three % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number_four = if condition { 5 } else { 6 };

    println!("The value of number_four is: {number_four}");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

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

    // Streamlining Conditional Loops with while
    let mut count_down_num = 3;

    while count_down_num != 0 {
        println!("{count_down_num}!");
        count_down_num -= 1;
    }

    println!("LIFTOFF!!!");

    // Looping Through a Collection with for
    let a_loop = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a_loop[index]);
        index += 1;
    }

    let a_loop_two = [10, 20, 30, 40, 50];

    for element in a_loop_two {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }

    println!("LIFTOFF NUNBER TWO !!!");
}

fn another_function_01() {
    println!("Another function.");
}

fn another_function_02(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
