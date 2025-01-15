use std::{
    net::{Ipv4Addr, SocketAddrV4},
    str::FromStr,
};

use anyhow::Ok;
use dotenv::dotenv;
use server::{set_up_tracing_subscriber, ApiSettings};
use tracing::error;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    set_up_tracing_subscriber();

    let api = ApiSettings::build().map_err(|e| {
        error!("{:?}", e);
        e
    })?;

    let socket_addr_v4 = SocketAddrV4::new(Ipv4Addr::from_str(&api.host)?, api.port.parse()?);

    Ok(())
}
