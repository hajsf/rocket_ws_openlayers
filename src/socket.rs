use ws::{Handler, Message, Request, Response, Result, Sender, CloseCode, WebSocket};
use std::fs;

pub struct Server {
    pub out: Sender,
}

#[derive(Serialize, Deserialize, Debug)]
struct Location {
    unique_id: String,
    lat: f64,
    lon: f64,
    speed: f64
}

impl Handler for Server {

    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("Server got message '{}'. ", msg);

      //  let deserialized: Location = serde_json::from_str(msg.as_text()?).unwrap();
      //  let id = deserialized.unique_id;

        self.out.broadcast(msg)
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away   => println!("The client is leaving the site."),
            _ => println!("The client encountered an error: {}", reason),
        }
    }

    fn on_request(&mut self, req: &Request) -> Result<(Response)> {
        match req.resource() {
            "/ws" => Response::from_request(req),
            // Create a custom response
            "/" => {
                let filename = "index.html";
                let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
                Ok(Response::new(200, "OK", contents.as_bytes().to_vec()))
            }
            _ => Ok(Response::new(404, "Not Found", b"404 - Not Found".to_vec())),
        }
    }
}