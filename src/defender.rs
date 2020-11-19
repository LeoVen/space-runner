use crate::game_interface::GameInterface;
use crate::types::ClientType;
use std::net::TcpStream;
use std::io::Write;
use crate::game_engine::GameEngine;
use std::sync::Arc;

pub fn client_defender(mut stream: TcpStream) {
    let mut game = GameInterface::new(ClientType::Attacker);

    while game.is_running() {
        game.key_pressed();

        game.next_event();
    }
}

pub fn server_defender(mut stream: TcpStream, game_engine: Arc<GameEngine>) {
    // while match stream.read(&mut data) {
    //     Ok(size) => {
    //         stream.write(&data[0..size]).unwrap();
    //         true
    //     },
    //     Err(_) => {
    //         false
    //     }
    // } {}
}
