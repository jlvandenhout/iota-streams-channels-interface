# IOTA Streams Channels Interface
A browser based graphical user interface to the IOTA Streams Channels application. The intention is to have a graphical way to explore the possibilities of the application and visualize the message structure.

|⚠️ WORK IN PROGRESS ⚠️|
|-----------------------|
|This repository is actively developed and does not contain a usable program yet.|

## Prerequisites
To build and run the server and interface, we need:

- [Rust](https://www.rust-lang.org/tools/install).
- [npm](https://www.npmjs.com/get-npm).

## Getting started
To clone and serve on default port 3030:

```bash
git clone https://github.com/jlvandenhout/iota-streams-channels-interface
cd iota-streams-channels-interface
cargo run
```

To use an other port:

```bash
cargo run -- -p <PORT>
```

To just serve the user interface on default port 3000 without server functionality:

```bash
cd interface
npm start
```

## User flow
1. Connect to a Node. Choose between:
    - Testnet.
    - Mainnet.
    - Custom Node URL.
2. Initialize a User instance:
    1. Provide a Seed.
    2. Choose between:
        - Become an Author to a Single Branch Channel.
        - Become an Author to a Multi Branch Channel.
        - Become a Recipient to a Channel.
3. Initialize a Channel:
    - As an Author: Announce a Channel.
    - As a Recipient: Provide an Announcement Link.
4. Send and receive messages.

## Implementation
The graphical user interface is built in Javascript using React and communicates to the server in JSON. User input is sent in the body of POST requests. Server output will be received using a web socket connection. The server is implemented in Rust using the `warp` crate and provides a mapping between the user interface and the Streams Channels interface.

## TODO list
- [x] Implement a way to set up the prerequisites like initializing a Transport and a User instance.
- [ ] Map basic functionality to graphical components:
    - [ ] Announcing a Channel as an Author.
    - [ ] Sending a Tagged Message as an Author.
    - [ ] Participate in a Channel as a Recipient.
    - [ ] Receive Messages as a Recipient.
- [ ] Map remaining functionality to graphical components:
    - [ ] Sending Keyload Messages as an Author.
    - [ ] Subscribe and Unsubscribe as a Recipient.
    - [ ] Become a Publisher as Recipient.
    - [ ] Sending Tagged Messages as a Publisher.
- [ ] Allow to persist and recall User states (client side or server side?).
- [ ] Visualize the message structure:
    - [ ] Listen for message updates.
    - [ ] Display messages in a graph.
    - [ ] Figure out how to find the next linked message ID from any current message.
    - [ ] Connect linked messages in the graph.
- [ ] Improve error handling:
    - [ ] Provide a way to communicate errors to the interface (can be regular HTTP status codes in response to the POST request).
    - [ ] Validate user input ([validator](https://docs.rs/validator) is a good candidate for this).
    - [ ] Monitor Channels messaging errors.

## Contribute
Any feedback on code quality, suggestions or help is welcome. Feel free to open an issue, send a PR or [contact me on the IOTA Discord server](https://discordapp.com/users/453235678386585601/).
