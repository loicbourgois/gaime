extern crate ws;

use ws::listen;



fn main() {
    println!("Starting snake_server");
    listen("0.0.0.0:8080", |out| {
        move |msg: ws::Message| {
            println!("Received: {}", msg);
            let text = msg.as_text().unwrap();
            let login = "login";
            if text.starts_with(login) {
                let username = text.split_at(login.len()).1;
                let response = format!("{}{}", "Hello ", username);
                out.send(ws::Message::Text(response))
            } else {
                out.send(msg)
            }
        }
    }).unwrap()
}
