# Kasumin player
Kasumin plays music using a client-server architecture.

# Overview
* [kasumin](kasumin) is both a library as well an implementation of the server.
* [yohane](yohane) contains the data types used by the server and client
* [hoshizora](hoshizora) implements basic user interfaces to communicate with the server

# Architecture

The server plays music on the host while the client controls what the server plays. The server supports playing music from a myriad of sources including local files, Spotify [via librespot](https://github.com/librespot-org/librespot), or YouTube.

Clients are identified by a UUID which the server assigns upon connection. All communication is serialized with [MessagePack](https://msgpack.org/) which follows a small transport that indicates the size of the data being received.

The envelope is pretty simple: `kasu:BIG-ENDIAN-U32\r\n`

For example, `kasu:16\r\n`. The `u32` is big endian which is also host byte order. Client-server messages are small, so the server checks the data size to avoid DOS attacks.
