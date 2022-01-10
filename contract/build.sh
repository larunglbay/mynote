ID=vbi-near-cource.quanglien.testnet
ID=vbi-near-course-mynote.nghilt.testnet

cargo build --target wasm32-unknown-unknown --release

near deploy --accountId $ID --wasmFile target/wasm32-unknown-unknown/release/mynote.wasm

near call $ID new '{}' --accountId $ID

near call $ID insert_note '{"name": "title2", "_content": "content2"}' --accountId $ID
near call $ID insert_note '{"name": "title2", "_content": "content2"}' --accountId vbi-near-cource-football-bet.quanglien.testnet
near call $ID insert_note '{"name": "title3", "_content": "content2"}' --accountId vbi-near-cource-football-bet.quanglien.testnet
near call $ID insert_note '{"name": "title4", "_content": "content2"}' --accountId vbi-near-cource-football-bet.quanglien.testnet
near call $ID edit_note '{"name": "title4", "_content": "content4"}' --accountId vbi-near-cource-football-bet.quanglien.testnet
near call $ID get_notes '' --accountId $ID
near call $ID get_notes '' --accountId vbi-near-cource-football-bet.quanglien.testnet
