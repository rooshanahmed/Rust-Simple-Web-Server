use simple_server::{Server,StatusCode};
fn main() {
    let server = Server::new(|request,mut response|{
        println!("request received {} {}",request.method(),request.uri());
        Ok(response.body(String::from("Hello From Rust Server").into_bytes())?)
    });
    server.listen("127.0.0.1" , "5000")
}
