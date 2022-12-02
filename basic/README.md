# Rust Smart Contract

To get started:

1. `near delete password.nicolad.testnet nicolad.testnet` // delete sub account
2. `near create-account password.nicolad.testnet --masterAccount nicolad.testnet --initialBalance 1` // recreate sub account
3. `near deploy password.nicolad.testnet --wasmFile res/password.wasm --initFunction 'new' --initArgs '{"solution": "8f968bdd5667276cc026dac352e423474564b3968f0d5e0a9023d8ef8f1f75c1"}'`

Contract deployment transaction: https://explorer.testnet.near.org/transactions/HeSbPazbirkU7Hu3bwiUpKSsibrBxDxjv9GhQyAMne9M
