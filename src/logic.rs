
pub fn to_binary(dec: &str) -> String {    
    if dec == "" {
        return "Output: 0".to_string();
    }

    let mut dec_num = match dec.trim().parse::<i128>() {
        Ok(number) => number,
        Err(_) => {
            return "Error: Overflow of i128. Do you really need a number that big?".to_string();
        }
    };

    let mut bin = String::new();
    while dec_num > 0 {
        if dec_num % 2 == 0 {
            bin.push('0');
        } else {
            bin.push('1');
        }
        dec_num /= 2;
    }
    let mut ret = "Output: ".to_owned();
    ret.push_str(bin.chars().rev().collect::<String>().as_str());
    ret
}

pub fn to_decimal(bin: &str) -> String {

    let mut dec: i128 = 0;
    let mut count = 0;
    
    if bin.len() > 127 {
        return "Error: Overflow of i128. Do you really need a number that big?".to_string();
    }

    for c in bin.chars().rev() {
        match c {
            '0' => (),
            '1' => dec += 2_i128.pow(count),
            _ => {
                return format!("Invalid character: {}", c)
            }
        }
        count += 1;
    }
    let mut ret = "Output: ".to_owned();
    ret.push_str(dec.to_string().as_str());
    ret
    
}

