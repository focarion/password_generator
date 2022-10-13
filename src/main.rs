use std::{fs::File, io::Write};

use rand::distributions::{Alphanumeric, DistString};
use text_io::read;

fn main() {
    println!("How many passwords to generate?");
    let pwd_amount: usize = read!();
    println!("Password lenght");
    let pwd_lenght: usize = read!();
    println!("Output to text file?");
    println!("Type Yes or No");
    let pwd_to_file: String = read!();
    let output_to_file = match pwd_to_file.as_str() {
        "yes" => true,
        "Yes" => true,
        "YES" => true,
        "no" => false,
        "No" => false,
        "NO" => false,
        _ => panic!("Only yes and no is accepted")
        
    };
    let mut password_vec: Vec<String> = Vec::new();
    for _ in 0..pwd_amount {
        let string = Alphanumeric.sample_string(&mut rand::thread_rng(), pwd_lenght);
        if output_to_file {
            password_vec.push(string.clone());
        }
        println!("{}", string);
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
