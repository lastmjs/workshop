<p align="center">
<img src="sfbw.png" width="200">
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
    * Switch to nightly compiler:
        ```bash
        rustup toolchain install nightly
        rustup default nightly
        ```
    * Install a Rust IDE. We recommend using CLion with Rust plugin: https://www.jetbrains.com/clion/

* It is advisable to have docker installed too: http://docker.io
* Clone this github repo: https://github.com/nearprotocol/nearcore
* Install [near-shell](https://github.com/nearprotocol/near-shell):
    ```bash
    npm install -g near-shell
    ```

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
