use std::{cell::RefCell, rc::Rc};

use p2p::{
    console_log, wasm_bindgen, wasm_bindgen_futures, Closure, ConnectionUpdate, JsCast, JsValue,
    P2P,
};
use web_sys::{window, HtmlInputElement};

#[wasm_bindgen]
extern "C" {
    pub fn alert(value: &str);
    pub fn prompt(value: &str) -> JsValue;
    pub fn confirm(value: &str) -> JsValue;
}

fn main() {
    wasm_bindgen_futures::spawn_local(main_async());
}

async fn main_async() {
    console_error_panic_hook::set_once();
    let mut p2p = P2P::new("ws://127.0.0.1:9001");

    let id = p2p.id().await;
    console_log!("Your peer id: {}", id);

    let answer = confirm("Do youa wnt to send a connection request?")
        .as_bool()
        .unwrap();

    if answer {
        let peer_id = prompt("What is the ID of the peer you want to connect to?")
            .as_string()
            .unwrap();

        p2p.connect(&peer_id).await;
    }

    let document = window().unwrap().document().unwrap();

    let other_peer_id: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(None));

    let callback_p2p = p2p.clone();
    let callback_other_peer_id = other_peer_id.clone();

    let callback = Closure::<dyn FnMut()>::new(move || {
        let document = window().unwrap().document().unwrap();
        let message_input = document
            .get_element_by_id("message-input")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap();

        let message = message_input.value();
        message_input.set_value("");

        if let Some(peer_id) = callback_other_peer_id.borrow().as_ref() {
            callback_p2p.send(&peer_id, &message);
        }

        insert_message(message, "You", "blue");
    });

    let chat_form = document.get_element_by_id("chat-form").unwrap();
    chat_form
        .add_event_listener_with_callback("submit", callback.as_ref().unchecked_ref())
        .unwrap();
    callback.forget();

    loop {
        let (messages, connections) = p2p.update().await;

        for connection in connections {
            match connection {
                ConnectionUpdate::Connected(peer_id) => {
                    console_log!("Peer {} connected.", peer_id);
                    other_peer_id.borrow_mut().replace(peer_id);
                }
                ConnectionUpdate::Disconnected(peer_id) => {
                    console_log!("Peer {} disconnected.", peer_id);
                }
            }
        }

        for (peer_id, message) in messages {
            console_log!("Peer {} says {}", peer_id, message);
            insert_message(message, "Peer", "red");
        }
    }
}

fn insert_message(message: String, name: &str, color: &str) {
    let document = window().unwrap().document().unwrap();
    let messages_div = document.get_element_by_id("messages").unwrap();

    let element = document.create_element("p").unwrap();
    element.set_inner_html(&format!(
        r#"
    <p>
        <b style="color:{color};">{}: </b> <span>{}</span>
    </p>
    "#,
        name, message
    ));

    messages_div.append_child(&element).unwrap();
    element.scroll_into_view();
}
