# study capnp

## try!

term 0

``` shell
sudo tcpdump -i lo
```

term 1

``` shell
RUST_LOG=trace cargo run -- server
```

term 2

``` shell
RUST_LOG=trace cargo run -- client
```
