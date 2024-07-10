use std::{cell::RefCell, rc::Rc};
use wasm_p2p::{
    wasm_bindgen,
    wasm_bindgen_futures::{self, spawn_local},
    Closure, ConnectionUpdate, JsCast, JsValue, P2P,
};
use web_sys::{window, HtmlInputElement};

#[wasm_bindgen]
extern "C" {
    pub fn alert(value: &str);
    pub fn prompt(value: &str) -> JsValue;
    pub fn confirm(value: &str) -> JsValue;
}

fn main() {
    console_error_panic_hook::set_once();
    wasm_bindgen_futures::spawn_local(main_async());
}

async fn main_async() {
    let mut p2p = P2P::new("wss://signaling.luisherasme.com");
    let id = p2p.id().await;
    let other_peer_id: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(None));

    window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("peer-id")
        .unwrap()
        .set_inner_html(&id);
    setup_connect_button(p2p.clone());
    setup_chat_input(p2p.clone(), other_peer_id.clone());

    loop {
        let (messages, connections) = p2p.update().await;

        for connection in connections {
            match connection {
                ConnectionUpdate::Connected(peer_id) => {
                    insert_message(format!("Peer {} connected.", peer_id), "SYSTEM", "black");
                    other_peer_id.borrow_mut().replace(peer_id);
                }
                ConnectionUpdate::Disconnected(peer_id) => {
                    insert_message(format!("Peer {} disconnected.", peer_id), "SYSTEM", "black");
                }
            }
        }

        for (_peer_id, message) in messages {
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

fn setup_connect_button(p2p: P2P) {
    let document = window().unwrap().document().unwrap();

    let callback = Closure::<dyn FnMut()>::new(move || {
        let mut p2p = p2p.clone();
        spawn_local(async move {
            if let Some(peer_id) =
                prompt("What is the ID of the peer you want to connect to?").as_string()
            {
                p2p.connect(&peer_id).await;
            }
        });
    });

    document
        .get_element_by_id("connect")
        .unwrap()
        .add_event_listener_with_callback("click", callback.as_ref().unchecked_ref())
        .unwrap();

    callback.forget();
}

fn setup_chat_input(p2p: P2P, other_peer_id: Rc<RefCell<Option<String>>>) {
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

    window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("chat-form")
        .unwrap()
        .add_event_listener_with_callback("submit", callback.as_ref().unchecked_ref())
        .unwrap();

    callback.forget();
}
