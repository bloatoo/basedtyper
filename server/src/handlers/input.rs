use crate::server::Server;
use serde_json::json;
pub fn input_handler(data: String, server: &mut Server) {
    let args: Vec<&str> = data
        .split(" ")
        .map(|elem| elem.trim())
        .collect();

    match args[0] {
        "start" => {
            println!("Starting");
            let json = json!({
                "call": "start",
                "data": {
                    "words": "here are some random words",
                }
            });

            server.broadcast(serde_json::to_string(&json).unwrap());
        }
        _ => ()
    }
}
