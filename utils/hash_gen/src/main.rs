use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: {} <password>", args[0]);
        eprintln!("Example: {} TestPassword1", args[0]);
        std::process::exit(1);
    }
    
    let password = &args[1];
    let cost = 4; // Lower cost for development (faster)
    
    match bcrypt::hash(password, cost) {
        Ok(hash) => println!("{}", hash),
        Err(e) => {
            eprintln!("Error generating hash: {}", e);
            std::process::exit(1);
        }
    }
}
