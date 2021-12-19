use std::io;
use std::io::Write;

pub fn get_string(q: &str) -> String {
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

// make the type same as varible
pub fn get_int(q: &str) -> i64 {
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

pub fn get_float(q: &str) -> f64 {
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

