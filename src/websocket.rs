use std::{cell::RefCell, collections::VecDeque, sync::Mutex};

use bevy::prelude::*;
use web_sys::{WebSocket, MessageEvent, Blob, FileReader, ProgressEvent};
use wasm_bindgen::prelude::*;
use std::rc::Rc;

use crate::{boards::Board, console_log};

static MESSAGE_QUEUE: Mutex<VecDeque<QueueItem>> = Mutex::new(VecDeque::new());
pub static OUTBOUND_QUEUE: Mutex<VecDeque<Vec<u8>>> = Mutex::new(VecDeque::new());

thread_local! {
    static WS: Rc<RefCell<Option<WebSocket>>> = Rc::new(RefCell::new(None));
}

enum QueueItem {
    UnlockBoard,
    Board(Vec<u8>)
}

pub struct WebSocketPlugin;

impl Plugin for WebSocketPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Board::new())
            .add_systems(Update, update_variables);
    }
}

fn update_variables (
    mut board: ResMut<Board>,
) {

    let mut q = MESSAGE_QUEUE.lock().unwrap();

    while let Some(a) = q.pop_front() {
        match a {
            QueueItem::UnlockBoard => board.locked = false,
            QueueItem::Board(bombs) => board.start(bombs),
        }
    }

}

fn read_blob(blob: Blob) {
    let file_reader = FileReader::new().unwrap();
    let fr_c = file_reader.clone();
    let onloadend = Closure::wrap(Box::new(move |_ev: ProgressEvent| {
        let array = fr_c.result().unwrap();
        let array_buffer: js_sys::ArrayBuffer = array.dyn_into().unwrap();
        let uint8_array = js_sys::Uint8Array::new(&array_buffer);
        let mut vec: Vec<u8> = uint8_array.to_vec();
        // Now you have the bytes!
        // console_log!("Binary data: {:?}", vec);

        let mut q = MESSAGE_QUEUE.lock().unwrap();

        if let Some(player_pos) = vec.pop() {
            console_log!("player position: {}", player_pos);
            if player_pos == 0 {
                q.push_back(QueueItem::UnlockBoard);
            }
        }
        q.push_back(QueueItem::Board(vec));

    }) as Box<dyn FnMut(_)>);
    file_reader.set_onloadend(Some(onloadend.as_ref().unchecked_ref()));
    file_reader.read_as_array_buffer(&blob).unwrap();
    onloadend.forget();
}

#[wasm_bindgen]
pub fn init_ws() {

    let ws = Rc::new(WebSocket::new("ws://localhost:9001/").unwrap());

    // Send a message after the connection opens
    let ws_onopen = ws.clone();
    let onopen = Closure::wrap(Box::new(move |_: JsValue| {
        ws_onopen.send_with_u8_array(&[0,0,0,1]).unwrap();
        ws_onopen.send_with_str("Hello from WASM!").unwrap();
    }) as Box<dyn FnMut(JsValue)>);

    ws.set_onopen(Some(onopen.as_ref().unchecked_ref()));
    onopen.forget();

    let ws_onmessage = ws.clone();
    let onmessage = Closure::wrap(Box::new(move |e: MessageEvent| {
        let data = e.data();

        console_log!("{:?}", data);

        if let Ok(txt) = data.clone().dyn_into::<js_sys::JsString>() {
            // Text message
            console_log!("{}", txt);
        } else if let Ok(blob) = e.data().dyn_into::<Blob>() {
            console_log!("got binary");
            read_blob(blob);
        } else {
            console_log!("No message what")
        }
    }) as Box<dyn FnMut(_)>);

    ws_onmessage.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
    onmessage.forget();

    let ws_dequeue = ws.clone();
    // Set up a timer to periodically flush queue
    let cb = Closure::wrap(Box::new(move || {

        match OUTBOUND_QUEUE.lock() {
            Ok(mut q) => {
                if let Some(message) = q.front() {
                    match ws_dequeue.send_with_u8_array(message) {
                        Ok(_) => {
                            // success → pop it
                            q.pop_front();
                        }
                        Err(err) => {
                            // failed → leave it in queue
                            console_log!("Send failed: {:?}", err);
                        }
                    }
                }
            },
            Err(e) => console_log!("OUTBOUND QUEUE ERROR: {}", e),
        }

    }) as Box<dyn FnMut()>);
    web_sys::window()
        .unwrap()
        .set_interval_with_callback_and_timeout_and_arguments_0(cb.as_ref().unchecked_ref(), 50)
        .unwrap();
    cb.forget();
}