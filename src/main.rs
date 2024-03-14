// sendet der ESP in HTTP/2
// wireshark für die Header
//
pub mod greeter {
    include!(concat!(env!("OUT_DIR"), "/greeter.rs"));
}
use prost::Message;

use core::convert::TryInto;
use embedded_svc::{
    http::{client::Client as HttpClient, Method},
    utils::io,
    wifi::{AuthMethod, ClientConfiguration, Configuration},
};
use std::env;
use std::error::Error;
use std::io::{Cursor, Read, Write};
use std::net::TcpStream;
use embedded_svc::http::client::Request;

use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::http::client::EspHttpConnection;
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition};
use log::{error, info};

const SSID: &str = env!("WIFI_SSID");
const PASSWORD: &str = env!("WIFI_PASS");
const SERVER_ADDRESS: &str = "192.168.2.191";
const SERVER_PORT: &str = "5000";
pub fn create_hello_request(name: String) -> greeter::HelloRequest {
    let mut hello_request = greeter::HelloRequest::default();
    hello_request.name = name;
    hello_request
}

pub fn serialize_greeter(hello: &greeter::HelloRequest) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.reserve(hello.encoded_len());

    hello.encode(&mut buf).unwrap();
    buf
}

// pub fn deserialize_greeter(buf: &[u8]) -> Result<greeter::HelloRequest, prost::DecodeError> {
//     greeter::HelloRequest::decode(&mut Cursor::new(buf))
// }

fn send_plain_grpc_request(message: &[u8], host: &str, port: &str)
                           -> anyhow::Result<Vec<u8>, Box<dyn Error>> {
    let mut stream = TcpStream::connect(format!("{}:{}", host, port))?;

    stream.write_all(b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n")?;

    // the headers needed for the connection
    let headers: Vec<&'static [u8]> = vec![
        b":method: POST\r\n",
        b":scheme: http\r\n",
        b":path: /greeter/Greeter/SayHello\r\n",
        b"te: trailers\r\n",
        b"content-type: application/grpc\r\n",
        b"\r\n",
    ];

    stream.write_all(&headers.concat())?;
    stream.write_all(message)?;
    stream.flush()?;

    let mut response = Vec::new();
    stream.read_to_end(&mut response)?;


    Ok(response)
}

fn send_grpc_request(
    client: &mut HttpClient<EspHttpConnection>,
    message: &[u8]) -> anyhow::Result<()> {

    let s = match std::str::from_utf8(message) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    println!("length of message: {}", s.len().to_string());


    let headers = [("content-type", "application/grpc+proto"),
        ("Content-Length", &s.len().to_string()),
        ("path", "greeter/Greeter/SayHello")];

    let url = "http://192.168.2.191:5000/";

    let mut request = client.post(url, &headers)?;
    request.write(message)?;

    info!("-> POST {}", url);
    let mut response = request.submit()?;

    // Process response
    let status = response.status();
    info!("<- {}", status);

    let mut buf = [0u8; 1024];
    let bytes_read = io::try_read_full(&mut response, &mut buf).map_err(|e| e.0)?;
    info!("Read {} bytes", bytes_read);

    match std::str::from_utf8(&buf[0..bytes_read]) {
        Ok(body_string) => info!(
            "Response body (truncated to {} bytes): {:?}",
            buf.len(),
            body_string
        ),
        Err(e) => error!("Error decoding response body: {}", e),
    };

    // Drain the remaining response bytes
    while response.read(&mut buf)? > 0 {}


    Ok(())
}


fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;
    let sys_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(peripherals.modem, sys_loop.clone(), Some(nvs))?,
        sys_loop,
    )?;

    connect_wifi(&mut wifi)?;

    // // Create HTTP(S) client
    let mut client = HttpClient::wrap(EspHttpConnection::new(&Default::default())?);
    //
    // // GET
    get_request(&mut client)?;

    // let request = String::from("Hello, World!");
    //
    // let greeter_request = create_hello_request(request);
    // let request_vector = serialize_greeter(&greeter_request);
    //
    // let response = send_grpc_request(
    //     &mut client,
    //     &request_vector);
    //
    // // let decoded_response =  deserialize_greeter(&response.unwrap());
    // //
    // // print!("Decoded response from server was: {:?}", decoded_response);
    //
    // println!("Response from server was: {:?}", response);

    Ok(())
}

fn get_request(client: &mut HttpClient<EspHttpConnection>) -> anyhow::Result<()> {
    // Prepare headers and URL
    let headers = [("accept", "text/plain")];
    let url = "http://192.168.2.191:5000/";

    // Send request
    //
    // Note: If you don't want to pass in any headers, you can also use `client.get(url, headers)`.
    let mut request = client.request(Method::Get, url, &headers)?;
    request.write(b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n").expect("TODO: panic message");
    info!("-> GET {}", url);
    let mut response = request.submit()?;

    // Process response
    let status = response.status();
    info!("<- {}", status);
    let mut buf = [0u8; 1024];
    let bytes_read = io::try_read_full(&mut response, &mut buf).map_err(|e| e.0)?;
    info!("Read {} bytes", bytes_read);
    match std::str::from_utf8(&buf[0..bytes_read]) {
        Ok(body_string) => info!(
            "Response body (truncated to {} bytes): {:?}",
            buf.len(),
            body_string
        ),
        Err(e) => error!("Error decoding response body: {}", e),
    };

    // Drain the remaining response bytes
    while response.read(&mut buf)? > 0 {}

    Ok(())
}

fn connect_wifi(wifi: &mut BlockingWifi<EspWifi<'static>>) -> anyhow::Result<()> {
    let wifi_configuration: Configuration = Configuration::Client(ClientConfiguration {
        ssid: SSID.try_into().unwrap(),
        bssid: None,
        auth_method: AuthMethod::WPA2Personal,
        password: PASSWORD.try_into().unwrap(),
        channel: None,
    });

    wifi.set_configuration(&wifi_configuration)?;

    wifi.start()?;
    info!("Wifi started");

    wifi.connect()?;
    info!("Wifi connected");

    wifi.wait_netif_up()?;
    info!("Wifi netif up");

    Ok(())
}
