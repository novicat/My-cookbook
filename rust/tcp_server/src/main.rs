use std::net::{TcpListener, TcpStream};
use std::io::Read;
use std::str;

fn handle_client(mut stream: TcpStream) {
    println!("{:?}", stream);

    // for a set sized buffer
  //  let mut buffer = [0; 256];
   // match stream.read(&mut buffer) {

  // for a dynamic sized buffer
  let mut buffer = vec![];

  match stream.read_to_end(&mut buffer) {
        Ok(_) => println!("ok!"),
        Err(e) => println!("{:?}", e)

    };
    let buff_as_string = str::from_utf8(&buffer).unwrap();
    println!("{}", buff_as_string);
}



fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000")?;

    

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
Ok(())
}