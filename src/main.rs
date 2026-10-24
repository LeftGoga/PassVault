use std::io::{stdin, stdout, Write};
use std::collections::HashMap;
use std::fs;
fn main() -> std::io::Result<()> {
    let path = String::from("misc_data/output.txt");
    let mut passwords:HashMap<String, String> = HashMap::new();
    while true{
        println!("Enter your name of password and password with a space (or exit to exit):");
        stdout().flush().unwrap();
        let mut buffer = String::new();
        stdin().read_line(&mut buffer);
        let input = buffer.trim();
        if input.eq("exit") {break}
        let mut splits = input.split_whitespace();
        let name = splits.next().unwrap();
        if name.len() <= 0{
            Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "name length is invalid"))?
        }
        let password = splits.next().unwrap();
        if password.len() <= 0{
            Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "password length is invalid"))?
        }
        if splits.next().is_some() {
            Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "too many elements"))?
        }

        passwords.insert(String::from(name), String::from(password));
    }

    let mut file = fs::File::create(&path)?; // создает или перезаписывает файл
    for (name, password) in &passwords {
        file.write(format!("{}:{}\n",name, password).as_bytes())?;
    }


    Ok(())
}
