use ui::{ConnectionMode, UI};
use wasm_p2p::{utils::sleep, wasm_bindgen_futures::spawn_local, P2P};
mod ui;

fn main() {
    console_error_panic_hook::set_once();
    spawn_local(main_async());
}

async fn main_async() {
    let mut p2p = P2P::new("wss://signaling.luisherasme.com").await.unwrap();

    let id = p2p.id();

    UI::set_peer_id(&id);
    UI::setup_chat_input();

    let connection_mode = UI::ask_for_connection_mode();

    let mut connections = Vec::new();

    connections.push(match connection_mode {
        // Send connection request
        ConnectionMode::Send => {
            let peer_id = UI::ask_for_peer_id();

            UI::show_loading();
            let connection = p2p.connect(&peer_id).await.unwrap();
            UI::hide_loading();

            connection
        }
        // Receive connection request
        ConnectionMode::Receive => loop {
            if let Some(offer) = p2p.receive_offer() {
                break p2p.create_connection(offer).await.unwrap();
            }

            sleep(0).await;
        },
    });

    loop {
        for message in connections[0].receive() {
            UI::insert_message("Peer", message);
        }

        if let Some(message) = UI::get_message() {
            connections[0].send(&message).unwrap();
            UI::insert_message("You", message);
        }

        sleep(0).await;
    }
}
