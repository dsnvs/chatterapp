# Experimental project

This project is a learning exercise.
I am experimenting with Rust, Docker, distributed systems and p2p networks.

At the time of writing this readme page, anyone should be able to test this program by cloning the repo and running `cargo run chatterapp`.

The program will create a floodsub network and discover peers using mdns, it will also listen to input on a terminal and publish two types of messages:

- For input "A" it will submit a hardcoded string of text with an specific node as the recipient. Currently the recipient is hardcoded and is the same node that is submitting the messages. All nodes will discard this message as it is not meant for them.

- For any other input, it will submit a hardcoded string of text with no specific node as a recipient, thus, the message will be printed to the terminal.

There is also a rudimentary crud app written with python on this repo, but it is currently not used for this project.

On the long run, this project should become a rudimentary RPC framework running on top of a p2p stack.

As it currently is, there are many problems with how this program was coded and I am confident the serialization and deserialization is both inefficient and *not* memory-safe.

My next few commits should be focused on:
- Addressing memory safety.
- Replacing the serialization process with bebop
- Adding tests

Further ahead I will:
- Improve the user interface by either using a CLI utility (like enquirer) or making a web app for the UI.
- Encrypt RPC in such a way that only the recipient would be able to decrypt them.
- Maybe also experiment with adding a different discovery service as an alternative to mDNS. I was thinking maybe a leader election algorithm designating an entry-point to the network and then using magicDNS to resolve who is the leader (?)