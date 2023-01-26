export async function pub_key() { 
    const encoder = new TextEncoder()
const view = encoder.encode(await window.nostr.getPublicKey())
console.log(view)
return view;
}
 