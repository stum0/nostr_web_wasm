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
    debug!("{:?}", nostr_pub_key);
}

//get pub key example
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


//encrypt message example
// #[wasm_bindgen(inline_js = "export async function encrypt_msg(pubkey, plaintext) { 
//     const encoder = new TextEncoder()
//     const view = encoder.encode(
//         await window.nostr.nip04.encrypt(pubkey, plaintext))
//     return view;
// }
//  ")]
// extern "C" {
//     async fn encrypt_msg(pubkey: String, ciphertext: String) -> wasm_bindgen::JsValue;
// }


//decrypt message example
// #[wasm_bindgen(inline_js = "export async function decrypt_msg(pubkey, ciphertext) { 
//     const encoder = new TextEncoder()
//     const view = encoder.encode(
//         await window.nostr.nip04.decrypt(pubkey, ciphertext))
//     return view;
// }
//  ")]
// extern "C" {
//     async fn decrypt_msg(pubkey: String, ciphertext: String) -> wasm_bindgen::JsValue;
// }


//sign event example 

// #[derive(Serialize, Deserialize, Debug)]
// pub struct NostrEv {
//     pub id: Option<String>,
//     pub kind: u32,
//     pub pubkey: Option<String>,
//     pub content: String,
//     pub tags: Vec<nostr::Tag>,
//     created_at: u64,
//     pub sig: Option<String>,
// }

// pub async fn send_to_js(content: String) -> NostrEv {
//     let nostr_tag = "0905c0ff-9da3-48c7-906e-88883fdf14b3".to_string();

    
//     let created_at = timestamp();
//     let tags = vec![Tag::Generic(
//         TagKind::Custom("t".to_string()),
//         vec![nostr_tag],
//     )];

//     let nostr_ev = NostrEv {
//         id: None,
//         kind: 1,
//         pubkey: None,
//         content,
//         tags,
//         created_at,
//         sig: None,
//     };

//     let event = serde_wasm_bindgen::to_value(&nostr_ev).unwrap();

//     let signed_event = make_event(event).await;

//     let event: NostrEv = JsValue::into_serde(&signed_event).unwrap();

//     event
    

// }

// #[wasm_bindgen(inline_js = "export async function make_event(Event) { 
//     const event = await window.nostr.signEvent(Event)
  
//     return event;
// }
//  ")]
// extern "C" {
//     async fn make_event(event: JsValue) -> wasm_bindgen::JsValue;
// }
