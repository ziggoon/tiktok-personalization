use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{body, Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    //let twilio_sid = String::from("AC15b53c90d74c0a168d6e17171035e45a");
    //let twilio_token = String::from("e54b12521052583e97b0b677fd763293");
    //let client = twilio::Client::new(&twilio_sid, &twilio_token);

    let cloned_uri = req.uri().clone();
    println!("\nreceived a POST :: {}", cloned_uri);
    
    let bytes = body::to_bytes(req.into_body()).await?;
    let bod = String::from_utf8(bytes.to_vec()).expect("response was not valid utf-8");
    
    let split: Vec<&str> = bod.split(|c| c == '&' || c == '=').collect();
    let num_to = split[25].to_string().replace("%2B", "+");
    let num_from = split[37].to_string().replace("%2B", "+");
    let msg_body = split[21].to_string().replace("+", "");
    println!("\n!!new message received!!");
    println!("to: {}", num_to);
    println!("from: {}", num_from);
    println!("body: {}\n", msg_body);

    // post message to db... need help fixing this 
    /*let conn = Connection::open("db.db").expect("connection failed");
    conn.execute(
        "insert into messages (number_to, number_from, msg_body) values (?1, ?2, ?3)",
        [num_to, num_from, msg_body],
    ).expect("insert failed");*/


    Ok(Response::new(Body::from(bod)))
}

#[tokio::main]
pub async fn main() {
    // Construct our SocketAddr to listen on...
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // And a MakeService to handle each connection...
    let make_service = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(handle_request))
    });

    // Then bind and serve...
    let server = Server::bind(&addr).serve(make_service);
    
    println!("\t\t\tlistening on http://{}\n", &addr);

    // And run forever...
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}