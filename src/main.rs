//! Simple HTTP client example.

use core::convert::TryInto;
use std::env;
use embedded_svc::{
    http::{client::Client as HttpClient, Method},
    utils::io,
    wifi::{AuthMethod, ClientConfiguration, Configuration},
};

use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::http::client::EspHttpConnection;
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition};
//use byteorder::{ByteOrder, BigEndian, ReadBytesExt};
//use embedded_svc::io::Read;

use log::{error, info};


const SSID: &str = env!("WIFI_SSID");
const PASSWORD: &str = env!("WIFI_PASS");

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

    // Create HTTP(S) client
    let mut client = HttpClient::wrap(EspHttpConnection::new(&Default::default())?);

    // GET
    get_request(&mut client)?;



    Ok(())
}


fn get_request(client: &mut HttpClient<EspHttpConnection>) -> anyhow::Result<()> {
    // Prepare headers and URL
    let headers = [("accept", "text/plain")];
    let url = "http://192.168.178.35:5000/";

    // Send request
    //
    // Note: If you don't want to pass in any headers, you can also use `client.get(url, headers)`.
    let request = client.request(Method::Get, url, &headers)?;
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

/*
fn create_request(client: &mut HttpClient<EspHttpConnection>) {
    let headers = [("content-Type", "application/grpc+proto")];
    let url = "insert_url_here";

    let request = client.request(Method::Post, url, &headers)?;
    info!("-> POST {}", url);


}
*/

/*
fn write_message<W: Write>(mut w: W,message: &[u8]) -> anyhow::Result<()> {
    let mut prefix = [0u8; 5];
    BigEndian::write_u32(&mut prefix[1..], message.len() as u32);
    w.write_all(&prefix)?;
    w.write_all(message)?;
    Ok(())
}
*/
/*
fn read_message(mut body: impl Read) -> anyhow::Result<Vec<u8>> {
    let mut prefixes = [0u8; 5];
    body.read_exact(&mut prefixes)?;

    let msg_size = body.read_u32::<BigEndian>()? as usize;
    let mut buffer = vec![0u8; msg_size];
    body.read_exact(&mut buffer)?;

    Ok(buffer)
}
*/


/*
    0. HTTP-Client initialisieren
    1. Aufbauen einer Http Connection via http/2 im besten Fall
    2. Erstellen der Header? Möglicherweise ("Content-Type", "application/grpc+proto")
    3. Erstellen des Requests, POST mit der proto-message und Lesen der proto-response
 */