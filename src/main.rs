use std::{fs::File, io::Write};

use rand::{distributions::{Alphanumeric, DistString, Uniform}, Rng, prelude::Distribution};
use text_io::try_read;


fn password_gen (password_amount: usize, password_lenght: usize, output_to_file: bool, output_to_console: bool) -> Vec<String> {
    let mut password_vec: Vec<String> = Vec::new();
    for _ in 0..password_amount {
        let string = Alphanumeric.sample_string(&mut rand::thread_rng(), password_lenght);
        if output_to_file {
            password_vec.push(string.clone());
        }
        if output_to_console {
            println!("{}", string);
        }
    }
    password_vec
}
fn password_gen_utf (password_amount: usize, password_lenght: usize, output_to_file: bool, output_to_console: bool) -> Vec<String> {
    let mut password_vec: Vec<String> = Vec::new();
    for _ in 0..password_amount {
        let string = rand_utf8::rand_utf8(&mut rand::thread_rng(), password_lenght);
        if output_to_file {
            password_vec.push(string.clone().into_string());
        }
        if output_to_console {
            println!("{}", string.clone().into_string());
        }
    }
    password_vec
}
fn password_gen_strong(password_amount: usize, password_lenght: usize, output_to_file: bool, output_to_console: bool) -> Vec<String> {
    let mut password_vec = vec![];
    for _ in 0..password_amount {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";

    let string: String = (0..password_lenght)
        .map(|_| {
            let index = rand::thread_rng().gen_range(0..CHARSET.len());
            CHARSET[index] as char
        })
        .collect();
        if output_to_file {
            password_vec.push(string.clone());
        }
        if output_to_console {
            println!("{}", string.clone());
        }
    }
    password_vec
}
fn password_gen_numbers(password_amount: usize, password_lenght: usize, output_to_file: bool, output_to_console: bool) -> Vec<String> {
    let mut password_vec = vec![];
    for _ in 0..password_amount {
        let range_start = 10_u32.pow(password_lenght as u32-1_u32);
        let range_end = (10_u32.pow(password_lenght as u32))-1;
        let range = Uniform::from(range_start..=range_end);
    let string: String = range.sample(&mut rand::thread_rng()).to_string();
        if output_to_file {
            password_vec.push(string.clone());
        }
        if output_to_console {
            println!("{}", string.clone());
        }
    }
    password_vec
}
fn main() {
    println!("Type of Password to generate");
    println!("1 -- ASCII Password");
    println!("2 -- Password with Symbols");
    println!("3 -- UTF8 Password");
    println!("4 -- Numbers Password");
    let pwd_type: usize = match try_read!() {
        Ok(n) =>n,
        Err(err) => match err {
            text_io::Error::Parse(_, _) => panic!("It is not a number"),
            _ => panic!("Something gone wrong")
        }
    };
    let filtered_pwd_type = match pwd_type {
        1 => 1,
        2 => 2,
        3 => 3,
        4 => 4,
        _ => panic!("Not a type included")
        
    };
    println!("How many passwords to generate?");
    let pwd_amount: usize = match try_read!() {
        Ok(n) =>n,
        Err(err) => match err {
            text_io::Error::Parse(_, _) => panic!("Number too high or is not a number"),
            _ => panic!("Something gone wrong")
        }
    };
    println!("Password lenght");
    let pwd_lenght: usize = match try_read!() {
        Ok(n) =>n,
        Err(err) => match err {
            text_io::Error::Parse(_, _) => panic!("Number too high or is not a number"),
            _ => panic!("Something gone wrong")
        }
    };
    println!("Output to text file?");
    println!("Type Yes or No");
    let pwd_to_file: String = match try_read!() {
        Ok(n) =>n,
        Err(err) => match err {
            _ => panic!("Something gone wrong")
        }
    };
    let output_to_file = match pwd_to_file.to_ascii_lowercase().as_str() {
        "yes" => true,
        "no" => false,
        _ => panic!("Only yes and no is accepted")
        
    };
    println!("Output to console?");
    println!("Type Yes or No");
    let pwd_to_console: String = match try_read!() {
        Ok(n) =>n,
        Err(err) => match err {
            _ => panic!("Something gone wrong")
        }
    };
    let output_to_console = match pwd_to_console.to_ascii_lowercase().as_str() {
        "yes" => true,
        "no" => false,
        _ => panic!("Only yes and no is accepted")
        
    };
    let password_vec: Vec<String>;
    if filtered_pwd_type == 1 {
        password_vec = password_gen(pwd_amount, pwd_lenght, output_to_file, output_to_console);
    } else if filtered_pwd_type == 2 {
        password_vec = password_gen_strong(pwd_amount, pwd_lenght, output_to_file, output_to_console);
    } else if filtered_pwd_type == 3 {
        password_vec = password_gen_utf(pwd_amount, pwd_lenght, output_to_file, output_to_console);
    } else if filtered_pwd_type == 4 {
        password_vec = password_gen_numbers(pwd_amount, pwd_lenght, output_to_file, output_to_console);
    } else {
        panic!("That's not a type of password")
    }
    if output_to_file {
        let mut file = File::create("./passwords.txt").unwrap();
        cfg_if::cfg_if! {
            if #[cfg(target_os = "windows")] {
                writeln!(file, "{}", password_vec.join("\r\n")).unwrap();
            } else {
                writeln!(file, "{}", password_vec.join("\n")).unwrap();
            }
        }
        
    }
}
