# Wialon protocol implementation 

Simple implementation [Wialon](doc/WialonIPS.pdf) protocol written on pure Rust.

## Install

```
cargo install
```

## Run

Run command format:
```
wialon-protocol <listner_local_addr> <buf_size>
```

where
- ```listner_local_addr``` - local address which will be listened incoming connection
- ```buf_size``` - size of internal queue for saving packet

For example:

```
wialon-protocol 0.0.0.0:5555 1000
```