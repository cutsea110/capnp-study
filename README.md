# study capnp

## try!

term 0

``` shell
sudo tcpdump -i lo
```

term 1

``` shell
RUST_LOG=trace cargo run -- -r server -p 8001
```

term 2

``` shell
RUST_LOG=trace cargo run -- -r client -p 8001
```
