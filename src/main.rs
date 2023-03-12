use std::io::Write;

fn main() {
    let mut res: Option<i64> = Option::None;
    while res.is_none() {
        println!("Welcome to a this program that convers things");
        println!("Choose an options:");
        println!("1. Convert from Binary to Decimal");
        println!("2. Convert from Decimal to Binary");
        print!("Enter your choice: "); std::io::stdout().flush().unwrap();

        let mut buffer = String::new();
        let _ = std::io::stdin().read_line(&mut buffer);

        //checks if the input is a number
        let num = buffer.trim().parse::<i64>();

        match num {
            Ok(number) => res = Some(number),
            Err(_) => {
                println!("Invalid choice")
            }
        }
    }

    let num = res.unwrap();

    match num {
        1 => to_binary(),
        2 => to_decimal(),
        _ => {
            println!("Invalid choice");
            main();
        },
    }
}

fn to_decimal() {
    println!("You chose to convert from Decimal to Binary");
    print!("Enter a decimal number: "); std::io::stdout().flush().unwrap();

    let mut dec_buffer = String::new();
    let _ = std::io::stdin().read_line(&mut dec_buffer);
    let mut dec = match dec_buffer.trim().parse::<i128>() {
        Ok(number) => number,
        Err(_) => {
            println!("Do better");
            to_decimal();
            return;
        }
    };

    let mut bin = String::new();
    while dec > 0 {
        if dec % 2 == 0 {
            bin.push('0');
        } else {
            bin.push('1');
        }
        dec /= 2;
    }
    println!("Out: {}", bin.chars().rev().collect::<String>());
}

fn to_binary() {
    println!("You chose to convert from Binary to Decimal");
    print!("Enter a binary number: "); std::io::stdout().flush().unwrap();

    let mut bin_buffer = String::new();
    let _ = std::io::stdin().read_line(&mut bin_buffer);
    let bin: &str = bin_buffer.trim();

    let mut dec: i128 = 0;
    let mut count = 0;
    
    for c in bin.chars().rev() {
        match c {
            '0' => (),
            '1' => dec += 2_i128.pow(count),
            _ => {
                println!("Invalid binary number, try again");
                to_binary();
                return;
            }
        }
        count += 1;
    }
    println!("Out: {}", dec);
}

