use js_sys::Uint8Array;
use log::debug;
use wasm_bindgen::prelude::*;
fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    wasm_bindgen_futures::spawn_local(async_main());
}

async fn async_main() {
    let get_pub_key = pub_key().await;

    let array = Uint8Array::new(&get_pub_key);
    let bytes: Vec<u8> = array.to_vec();

    let nostr_pub_key = std::str::from_utf8(&bytes).unwrap();
    debug!("lool {:?}", nostr_pub_key);
}

#[wasm_bindgen(inline_js = "export async function pub_key() { 
    const encoder = new TextEncoder()
const view = encoder.encode(await window.nostr.getPublicKey())
console.log(view)
return view;
}
 ")]
extern "C" {
    async fn pub_key() -> JsValue;
}
