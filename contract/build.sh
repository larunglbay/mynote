ID=mynote.larunglabay.testnet

near create-account $ID --masterAccount larunglabay.testnet --initialBalance 50
near delete $ID larunglabay.testnet

cargo build --target wasm32-unknown-unknown --release

near deploy --accountId $ID --wasmFile target/wasm32-unknown-unknown/release/mynote.wasm

near call $ID new '{}' --accountId $ID

near call $ID insert_note '{"name": "title", "_content": "content"}' --accountId $ID
near call $ID edit_note '{"name": "title", "_content": "content4"}' --accountId $ID
near call $ID get_notes '' --accountId $ID
