struct  Server{
    address : String,
}

impl Server{
    fn new(ip_address:String) -> Server {
        Server{
            address : ip_address,
        }
    }
    fn run(self){
        println!("Listening on {}",self.address);
    }
}

struct Request{
    path: String,
    query_string: String,
    method: String
}

enum Method{
    GET,
    DELETE,
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH
}

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}


