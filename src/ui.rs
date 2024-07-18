use wasm_p2p::{wasm_bindgen, Closure, JsCast, JsValue};
use web_sys::{window, Element, HtmlInputElement};

pub enum ConnectionMode {
    Receive,
    Send,
}

#[wasm_bindgen]
extern "C" {
    pub fn alert(value: &str);
    pub fn prompt(value: &str) -> JsValue;
    pub fn confirm(value: &str) -> JsValue;
}

#[derive(Clone)]
pub struct UI;

impl UI {
    pub fn set_peer_id(id: &str) {
        get_element_by_id("peer-id").set_inner_html(id);
        get_element_by_id("copy-peer-id-button")
            .set_attribute("onclick", &format!("navigator.clipboard.writeText('{id}')"))
            .unwrap();
    }

    pub fn insert_message(name: &str, message: String) {
        let messages = get_element_by_id("messages");

        let element = create_element("p");
        element.set_inner_html(&format!("<b>{name}:</b> {message}"));

        messages.append_child(&element).unwrap();
        element.scroll_into_view();
    }

    pub fn ask_for_connection_mode() -> ConnectionMode {
        let result = confirm("Do you want to send a connection?")
            .as_bool()
            .unwrap();

        if result {
            return ConnectionMode::Send;
        } else {
            return ConnectionMode::Receive;
        }
    }

    pub fn ask_for_peer_id() -> String {
        loop {
            if let Some(id) =
                prompt("What is the ID of the peer you want to connect to?").as_string()
            {
                break id;
            }
        }
    }

    pub fn show_loading() {
        get_element_by_id("loading").set_class_name("");
    }

    pub fn hide_loading() {
        get_element_by_id("loading").set_class_name("hidden");
    }

    pub fn get_message() -> Option<String> {
        let dirty = get_element_by_id("chat-form")
            .get_attribute("dirty")
            .unwrap();

        if dirty == "true" {
            let message_input = get_element_by_id("message-input")
                .dyn_into::<HtmlInputElement>()
                .unwrap();

            let value = message_input.value();
            message_input.set_value("");

            get_element_by_id("chat-form")
                .set_attribute("dirty", "false")
                .unwrap();

            return Some(value);
        }

        None
    }

    pub fn setup_chat_input() {
        let callback = Closure::<dyn FnMut()>::new(move || {
            get_element_by_id("chat-form")
                .set_attribute("dirty", "true")
                .unwrap();
        });

        get_element_by_id("chat-form")
            .set_attribute("dirty", "false")
            .unwrap();

        get_element_by_id("chat-form")
            .add_event_listener_with_callback("submit", callback.as_ref().unchecked_ref())
            .unwrap();

        callback.forget();
    }
}

fn get_element_by_id(id: &str) -> Element {
    window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id(id)
        .unwrap()
}

fn create_element(name: &str) -> Element {
    window()
        .unwrap()
        .document()
        .unwrap()
        .create_element(name)
        .unwrap()
}
