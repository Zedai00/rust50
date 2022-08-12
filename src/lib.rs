use std::io;
use std::io::Write;
use std::str::FromStr;

pub fn get_input<T: FromStr>(q: &str) -> T {
     loop {
        print!("{}", q);
        io::stdout().flush().unwrap();
    
        let mut str = String::new();
    
        io::stdin().read_line(&mut str).ok();
        let string = str.trim().parse();
        let string = match string {
            Ok(n) => n,
            Err(_) => continue,
        };
        return string
    }
}