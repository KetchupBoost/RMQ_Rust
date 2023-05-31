fn rep_loops() {
    let mut counter = 0;

    let result = loop{
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Result = {result}!");
}

fn multiple_loops() {
    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println("remaining = {remaining}")
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

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1; 
    }
    println!("LIFT OFF!");
}

fn loop_in_collection() {
    let array = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("While");
        println!("The value is: {}", array[index]);
        index += 1;
    }

    for element in array {
        println!("For in Elements of an Array");
        println!("The value is: {element}");
    }

    for number in (1..4).rev() {
        println!("For in Range Loop");
        println!("{number}");
    }

}