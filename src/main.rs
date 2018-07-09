use std::io;

fn main() {
    println!("Enter a positive integer.");

    let mut number = String::new();
    io::stdin().read_line(&mut number)
        .expect("Could not read your input");

    let mut number = number.trim().parse()
        .expect("Could not convert the string to a number.");

    let output = roman_numerals(&mut number);

    println!("The roman numeral version of the integer is {}", output);

}

fn roman_numerals(x: &mut u32) -> String {
    let mut output = String::new();
    let mut multiplier: u32; 

    //Thousands - M
    if *x >= 1000 {
        multiplier = *x / 1000;
        let a = "M".to_string().repeat(multiplier as usize);
        output.push_str(&a);
        *x = *x - (1000 * multiplier);
    }

    //Hundreds - D(500) C(100)
    multiplier = *x / 100;
    match multiplier {
        1 | 2 | 3 => {
            let a = "C".to_string().repeat(multiplier as usize);
            output.push_str(&a);
        },
        4 => output.push_str("CD"),
        5 => output.push_str("D"),
        6 => output.push_str("DC"),
        7 => output.push_str("DCC"),
        8 => output.push_str("DCCC"),
        9 => output.push_str("CM"),
        _ => {},
    }
    *x = *x - (100 * multiplier);
    
    //Tens - L(50) X(10)
    multiplier = *x / 10;
    match multiplier {
        1 | 2 | 3 => {
            let a = "X".to_string().repeat(multiplier as usize);
            output.push_str(&a);
        },
        4 => output.push_str("XL"),
        5 => output.push_str("L"),
        6 => output.push_str("LX"),
        7 => output.push_str("LXX"),
        8 => output.push_str("LXXX"),
        9 => output.push_str("XC"),
        _ => {},
    }
    *x = *x - (10 * multiplier);

    //Ones - V(5) I(1)
    match *x {
        1 | 2 | 3 => {
            let a = "I".to_string().repeat(*x as usize);
            output.push_str(&a);
        },
        4 => output.push_str("IV"),
        5 => output.push_str("V"),
        6 => output.push_str("VI"),
        7 => output.push_str("VII"),
        8 => output.push_str("VIII"),
        9 => output.push_str("IX"),
        _ => {},
    }

    output
}
