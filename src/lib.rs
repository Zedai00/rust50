use std::io;
use std::io::Write;

pub fn get_int(q: &str) -> i32 {
    loop {
        print!("{}", q);
        io::stdout().flush().unwrap();
    
        let mut str = String::new();
    
        io::stdin().read_line(&mut str).ok();
        let num = str.trim().parse();
        let num = match num {
            Ok(n) => n,
            Err(_) => continue,
        };
        return num
    }
}
