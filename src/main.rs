use pass_vault::password::Password;
use std::fs;
use std::io::{stdin, stdout, Write};

fn main() -> std::io::Result<()> {
    let path = String::from("misc_data/output.txt");
    let mut passwords: Vec<Password> = Vec::new();
    loop {
        println!("Enter your name of password and password with a space (or exit to exit):");
        stdout().flush().unwrap();
        let mut buffer = String::new();
        let bytes_read = stdin().read_line(&mut buffer)?;
        if bytes_read == 0 {
            break;
        } // если встреоии EOF
        let input = buffer.trim();
        if input.eq("exit") {
            break;
        }
        let mut splits = input.split_whitespace(); // возвращет только не нулевую длину - доп проверок не надо
        let Some(name) = splits.next() else {
            println!("Missing name");
            continue;
        };

        let Some(password) = splits.next() else {
            println!("Missing password");
            continue;
        };
        if splits.next().is_some() {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "too many elements",
            ))?
        }
        let psw = Password::new(name.to_string(), password.to_string()).unwrap();
        if passwords.contains(&psw) {
            println!("{} already exists, changing", &psw.get_name());
            if let Some(index) = passwords
                .iter()
                .position(|el| el.get_name() == psw.get_name())
            {
                passwords.remove(index);
                passwords.push(psw);
                continue
            }
        }

        passwords.push(psw)
    }

    let mut file = fs::File::create(&path)?; // создает или перезаписывает файл
    for password in &passwords {
        // write_all гарантиурет что все байты буудт записаны
        file.write_all(format!("{}\n", password.to_string()).as_bytes())?;
    }

    Ok(())
}
