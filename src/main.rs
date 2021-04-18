use std::env;

#[macro_use]
extern crate serde_derive;
extern crate gotham;

use gotham::hyper::{Body, Response, StatusCode};

use gotham::handler::IntoResponse;
use gotham::helpers::http::response::create_response;
use gotham::router::builder::*;
use gotham::router::Router;
use gotham::state::State;

extern crate pnet;
use pnet::datalink;

#[derive(Serialize)]
struct Device {
    name: String,
}

/// Implements `gotham::handler::IntoResponse` trait for `Product`
///
/// `IntoResponse` represents a type which can be converted to a response. This trait is used in
/// converting the return type of a function into a response.
///
/// This trait implementation uses the Serde project when generating responses. You don't need to
/// know about Serde in order to understand the response that is being created here but if you're
/// interested you can learn more at `http://serde.rs`.
impl IntoResponse for Device {
    fn into_response(self, state: &State) -> Response<Body> {
        create_response(
            state,
            StatusCode::OK,
            mime::APPLICATION_JSON,
            serde_json::to_string(&self).expect("serialized product"),
        )
    }
}


/// Function to handle the `GET` requests coming to `/products/t-shirt`
///
/// Note that this function returns a `(State, Product)` instead of the usual `(State, Response)`.
/// As we've implemented `IntoResponse` above Gotham will correctly handle this and call our
/// `into_response` method when appropriate.
fn get_device_handler(state: State) -> (State, Device) {
    let device = Device {
        name: sys_info::hostname().unwrap(),
    };

    (state, device)
}


/// Create a `Router`
///
/// /products/t-shirt            --> GET
fn router() -> Router {
    build_simple_router(|route| {
        route.get("/").to(get_device_handler);
    })
}


// pub fn handler(state: State) -> (State, String) {
//     let hostname = sys_info::hostname().unwrap();
//     let boottime = sys_info::boottime().unwrap();
//     let cpu_num = sys_info::cpu_num().unwrap();
//     let cpu_speed = sys_info::cpu_speed().unwrap();
//     let disk_info = sys_info::disk_info().unwrap();
//     let loadavg = sys_info::loadavg().unwrap();
//     let mem_info = sys_info::mem_info().unwrap();
//     let os_release = sys_info::os_release().unwrap();
//     let os_type = sys_info::os_type().unwrap();
//
//     // for iface in datalink::interfaces() {
//     //     println!("{:?}", iface.ips);
//     // }
//
//     // println!("{}", hostname);
//     // // print!("{:?}", boottime);
//     // println!("{:?}", cpu_num);
//     // println!("{:?}", cpu_speed);
//     // println!("{:?}", disk_info);
//     // println!("{:?}", loadavg);
//     // println!("{:?}", mem_info);
//     // println!("{:?}", os_release);
//     // println!("{:?}", os_type);
//
//     (state, hostname)
// }


fn main() {

    println!("Kicking off@");
    println!("{}", env::consts::OS); // Prints the current OS.


    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router())

}
