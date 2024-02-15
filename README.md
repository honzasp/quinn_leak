# Quinn memory leak reproducer

This repository contains a minimal reproducing example for a memory leak in Quinn. Run

    cargo r --bin server

to start a server on 127.0.0.1:2024, and

    cargo r --bin client

to start a client that connects to the server. The server will send a large amount of datagrams to the client,
and the client memory grows steadily.
