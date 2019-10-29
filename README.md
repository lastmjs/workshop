<p align="center">
<img src="logo.svg" width="240">
</p>

## MapReduce with Asynchronous Smart Contracts
*SFBW19 Near Protocol Workshop*

### Pre-requisites

* This workshop assumes you have a Rust development environment. If you don't then please do the following:
    * Install rustup:
        ```bash
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
        ```
    * Add Wasm toolchain:
        ```bash
        rustup target add wasm32-unknown-unknown
        ```
    * Install a Rust IDE. We recommend using CLion with Rust plugin: https://www.jetbrains.com/clion/

* It is advisable to have docker installed too: http://docker.io
* Clone this github repo: https://github.com/nearprotocol/nearcore
* Install [near-shell](https://github.com/nearprotocol/near-shell):
    ```bash
    npm install -g near-shell
    ```
    
### Resources
Throughout this workshop you might find the following resource helpful:
* [near-bindgen](https://github.com/nearprotocol/near-bindgen/) Rust API repository with some minimal documentation;
* [examples from near-bindgen](https://github.com/nearprotocol/near-bindgen/tree/master/examples) contains various
smart contract examples;
* [cross-contract example](https://github.com/nearprotocol/near-bindgen/tree/master/examples/cross-contract-high-level) is
an example of using high-level cross-contract API, including executing distributed merge sort;

Additionally, though not needed, you might find the following resources helpful for the overall understanding of Near Protocol:
* [Runtime high-level documentation](https://docs.google.com/document/d/1VRef627Y-Md1qAdRn0RFUojPxOs5nQmHyHBFYewwbZQ/edit);
* [Nearnomicon](https://nearprotocol.github.io/nomicon/).

## Exercise 0: Start a node and issue a transaction

### If you chose to install docker
To start your local node, go to `nearcore` and run the following script:
```bash
./scripts/start_localnet.py
```
This will pull the docker image and start a single local node. Enter an `<account id>` that you want to be associated with.

Then execute the following to follow the block production logs:
```bash
docker logs --follow nearcore
```

Create a new project:
```bash
near new_project myproject
cd myproject
```

Then in `src/config.json` modify `nodeUrl` to point to your local node:
```js
case 'development':
    return {
        networkId: 'default',
        nodeUrl: 'http://localhost:3030',
        contractName: CONTRACT_NAME,
        walletUrl: 'https://wallet.nearprotocol.com',
    };
```

Then copy the key that the node generated upon starting in your local project to use for transaction signing.
```bash
cp ~/.near/validator_key.json ./neardev/default/<account id>.json
```

## If you chose to not install docker
Go to `nearcore` and run:
```bash
cargo run --package keypair-generator --bin keypair-generator -- --account-id=<account id> --generate-config signer-keys --num-keys=1
```
where `<account id>` is your chosen account id you want to be associated with.

Open `~/.near/signer0_key.json` and give values of `account_id` and `public_key` to Max so that he can create an account for you.

Create a new project:
```bash
near new_project myproject
cd myproject
```

Then in `src/config.json` modify `nodeUrl` to point to your local node:
```js
case 'development':
    return {
        networkId: 'default',
        nodeUrl: 'http://35.230.56.29:3030',
        contractName: CONTRACT_NAME,
        walletUrl: 'https://wallet.nearprotocol.com',
    };
```

Then copy the key that the node generated upon starting in your local project to use for transaction signing.
```bash
cp ~/.near/signer0_key.json ./neardev/default/<account id>.json
```

## Issuing a transaction
First, let's check whether your account exists:
```bash
near state <account id>
```
It should print your account info, including the amount of tokens.

Now, let's create another account user our account:
```bash
near create_account friend_of_<account id> --masterAccount <account id> --initialBalance 100000000000000000
```
and check that it was successfully created:
```bash
near state friend_of_<account id>
```
You should see the tokens that you have transferred during the creation.

Let's send a couple of more tokens to that account:
```bash
near send <account id> friend_of_<account id> 10
```

Success! You have completed the first exercise!

## Exercise 1: Chat

Let's write a smart contract that multiple users will use to leave messages for each other. This contract has the following
requirements:
* We want Alice to be able to call the `leave_message` method on this contract and leave `Hey!` message for Bob;
* We want Bob to be able to call `get_unread_messages` and get a list of messages left for him from everyone;
* We want Bob to be able to call `mark_all_as_read` and mark all unread messages as read.

See [exercises/chat](https://github.com/nearprotocol/workshop/tree/master/exercises/chat) for a project template.

First we create a structure representing the state of our smart contract:
```rust
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Chat {
    // receiver_id -> list of (sender_id, message).
    unread_messages: HashMap<String, Vec<(String, String)>>,
}
```

Let's walk over the code here:
* `#[near_bindgen]` decorator marks it as a structure representing the contract state;
* `#[derive(Default)]` creates a function that will be automatically used to initialize the contact upon its first execution;
* `#[derive(BorshSerialize, BorshDeserialize)]` allows the state of the contract to be serialized with our binary serializer [borsh](http://borsh.io/);
* The structure itself contains a single field that maps message receiver to a list of messages together with whoever sent them.

Then we create smart contract methods:
```rust
#[near_bindgen]
impl Chat {
    pub fn leave_message(&mut self, receiver_id: String, message: String) {
        let sender_id = env::signer_account_id();
        // Add message to records.
    }
    
    ...
}
```

Let's walk over the code:
* `#[near_bindgen]` decorator marks public methods in the `impl` section as smart contract methods;
* Each method is a regular method, with one nuance, if it uses `&mut self` this method is expected to mutate the state of the contract
and can only be called using a transaction. If it uses `&self` then it can also be called as a "view" method without transactions;
* Internally each method is able to retrieve the account id of whoever called it, using `env::signer_account_id()`. See the full list of `env::*` methods [here](https://github.com/nearprotocol/near-bindgen/blob/master/near-bindgen/src/environment/env.rs).

Now go ahead and implement the rest of the code! You can check your solution by running `cargo test` from the `chat` folder.
You can also find solutions in [exercises/chat-solution](https://github.com/nearprotocol/workshop/tree/master/exercises/chat-solution)

You can build your smart contract with
```bash
cargo build --target wasm32-unknown-unknown --release
```

### Deploying and running the contract
First, let's create several accounts for the playground. Go to `myproject` and run:
```bash
near create_account chat_<account id> --masterAccount <account id> --initialBalance 10000000000000000
near create_account alice_<account id> --masterAccount <account id> --initialBalance 10000000000000000
near create_account bob_<account id> --masterAccount <account id> --initialBalance 10000000000000000
near create_account carol_<account id> --masterAccount <account id> --initialBalance 10000000000000000
```

Let's deploy your smart contract to `chat_<account id>`:
```bash
near deploy --accountId=chat_<account id> --wasmFile=../Projects/workshop/exercises/chat-solution/target/wasm32-unknown-unknown/release/chat_solution.wasm
```
(You would have to modify the Wasm path)

Now, let's use Alice's account to leave a message for Bob:
```bash
near call chat_<account id> leave_message '{"receiver_id": "bob_<account id>", "message": "Hey!"}' --accountId=alice_<account id>
```
And use Carol's account to leave a message for Bob:
```bash
near call chat_<account id> leave_message '{"receiver_id": "bob_<account id>", "message": "Hi!"}' --accountId=carol_<account id>
```
Finally, let's use Bob's account to lookup unread messages:
```bash
near call chat_<account id> get_unread_messages '' --accountId=bob_<account id>
```
