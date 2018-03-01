extern crate pencil;
extern crate kiteconnect as kite;

use std::env;

use pencil::{Pencil, Request, Response, PencilResult};
use pencil::jsonify;

use kite::connect::KiteConnect;


fn holdings(_: &mut Request) -> PencilResult {
    let api_key: &str = &env::var("API_KEY").unwrap();
    let access_token: &str = &env::var("ACCESS_TOKEN").unwrap();
    let kiteconnect = KiteConnect::new(api_key, access_token);
    let holdings = kiteconnect.holdings().unwrap().to_string();

    jsonify(&holdings)
}

fn main() {
    let mut app = Pencil::new("/");
    app.get("/", "holdings", holdings);
    app.run("localhost:5000");
}
