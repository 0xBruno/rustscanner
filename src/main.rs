use std::env; 
use std::net::TcpStream; 

const PORTS: i32 = 65535;

fn main() {
    let args: Vec<String> = env::args().collect();

    let ip = &args[1].to_string();

    for port in 1..=PORTS{
        let mut socket = String::new(); 
        
        socket += ip;
        socket += ":";
        socket += &port.to_string();

        if let Ok(_stream) = TcpStream::connect(socket) {
            println!(" Port {} open!", port); 
        } 
    }  
}