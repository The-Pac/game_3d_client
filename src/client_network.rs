use std::net::{SocketAddr, UdpSocket};
use std::time::SystemTime;
use bevy::prelude::*;
use bevy_renet::renet::{ClientAuthentication, RenetClient, RenetConnectionConfig};
use bevy_renet::RenetClientPlugin;
use crate::{PROTOCOL_ID, SERVER_ADDRESS, SERVER_PORT};

pub struct ClientNetworkPlugin;

impl Plugin for ClientNetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RenetClientPlugin)
            .insert_resource(create_client);
    }
}

fn create_client() -> RenetClient {
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH).unwrap();

    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    let client_id = current_time.as_millis() as u64;

    let connection_config = RenetConnectionConfig::default();

    let server_addr = SocketAddr::new(SERVER_ADDRESS.parse().unwrap(), SERVER_PORT);

    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr,
        user_data: None,
    };

    RenetClient::new(
        current_time,
        socket,
        client_id,
        connection_config,
        authentication,
    ).unwrap()
}

fn setup_client_network() {}