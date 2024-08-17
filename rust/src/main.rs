use std::net::TcpListener;
use std::io::Read;

fn main() {
  println!("yoyoyo!");

  let listener = TcpListener::bind("0.0.0.0:3001").unwrap();

  for stream in listener.incoming() {
    println!("Hello!");

//    let reader = BufReader::new(stream.unwrap());

    let mut buffer = String::new();

    stream.unwrap().read_to_string(&mut buffer).unwrap();

    println!("Howdy!");

//    let data = std::io::read_to_string(reader).unwrap();

    println!("Hello again!");

    println!("{}", &buffer);
  }
}

