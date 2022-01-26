fn main() {
    let string = String::from("localhost:8080".to_string());
    let server = Server::new(string);
    server.run();
}
struct  Server{
    addr:String,
}
impl Server{
    fn new(addr:String)->Self{
        Self { 
            addr
        }
    }
    fn run(self){
        println!("Listening on {}",self.addr);
    }
}

struct Request{
    path:String,
    query_string:String,
    method:Method
}
enum Method{
    GET,
    POST,
    DELETE,
    PUT
}

/*
GET /user?id=10 HTTP/1.1\r\n
HEADERS\r\n
BODY
*/

