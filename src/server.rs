use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    addr: String,
}

fn arr(a: &[u8]) {}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }
    pub fn run(self) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                  let mut buffer = [0; 5];
                    stream.read(&mut buffer);
                }
                Err((e)) => println!("Err"),
            }

            let res = listener.accept();

            if res.is_err() {
                continue;
            }

            let (stream, addr) = res.unwrap();
        }
    }
}
