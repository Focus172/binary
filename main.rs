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
        _ => println!("Invalid choice"),
    }
}

fn to_decimal() {
    println!("You chose to convert from Decimal to Binary");
    print!("Enter a decimal number: "); std::io::stdout().flush().unwrap();

    let mut dec_buffer = String::new();
    let _ = std::io::stdin().read_line(&mut dec_buffer);
    let mut dec = match dec_buffer.trim().parse::<i64>() {
        Ok(number) => number,
        Err(_) => {
            println!("Do better");
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
    let mut bin = match bin_buffer.trim().parse::<i64>() {
        Ok(number) => number,
        Err(_) => {
            println!("Do better");
            return;
        }
    };

    let mut dec = 0;
    let mut count = 0;
    while bin > 0 {
        let dig = bin % 10;
        match dig {
            0 => (),
            1 => dec += 2_i64.pow(count),
            _ => {
                println!("Invalid binary number");
                return;
            }
        }
        count += 1;
        bin /= 10;
    }
    println!("Out: {}", dec);
}

