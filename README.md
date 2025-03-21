# socks-to-http-proxy ![Rust](https://github.com/KaranGauswami/socks-to-http-proxy/workflows/Rust/badge.svg) ![release](https://img.shields.io/github/v/release/KaranGauswami/socks-to-http-proxy?include_prereleases)

An executable to convert SOCKS5 proxy into HTTP proxy

## About

`sthp` purpose is to create HTTP proxy on top of the Socks 5 Proxy

## How it works

It uses hyper library HTTP proxy [example](https://github.com/hyperium/hyper/blob/master/examples/http_proxy.rs) and adds functionality to connect via Socks5

## Compiling

Follow these instructions to compile

1.  Ensure you have current version of `cargo` and [Rust](https://www.rust-lang.org) installed `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2.  Add env `source $HOME/.cargo/env`
3.  Clone the project `$ git clone https://github.com/KaranGauswami/socks-to-http-proxy.git && cd socks-to-http-proxy`
4.  Build the project `$ cargo build --release`
5.  Once complete, the binary will be located at `target/release/sthp`
6.  Windows Binary `rustup target add x86_64-pc-windows-gnu`
7.  Compile Windows Binary `cargo build --release --target x86_64-pc-windows-gnu`

## Usage

```bash
sthp -p 8080 -s 127.0.0.1:1080
```

This will create proxy server on 8080 and use localhost:1080 as a Socks5 Proxy

```bash
sthp -p 8080 -s example.com:8080
```

```bash
.\sthp.exe -p 9080 -s ip:8080 -u uname -P pwd
```

This will create proxy server on 8080 and use example:1080 as a Socks5 Proxy

> [!NOTE]  
> The --socks-address (-s) flag does not support adding a schema at the start (e.g., socks:// or socks5h://). Currently, it only supports socks5h, which means DNS resolution will be done on the SOCKS server.

> [!WARNING]
> After v5, Changed default listening IP from `0.0.0.0` to `127.0.0.1`. This change restricts the application access to the local machine only.

### Options

There are a few options for using `sthp`.

```text
Usage: sthp [OPTIONS]

Options:
  -p, --port <PORT>                        port where Http proxy should listen [default: 8080]
      --listen-ip <LISTEN_IP>              [default: 127.0.0.1]
  -u, --username <USERNAME>                Socks5 username
  -P, --password <PASSWORD>                Socks5 password
  -s, --socks-address <SOCKS_ADDRESS>      Socks5 proxy address [default: 127.0.0.1:1080]
      --allowed-domains <ALLOWED_DOMAINS>  Comma-separated list of allowed domains
      --http-basic <HTTP_BASIC>            HTTP Basic Auth credentials in the format "user:passwd"
  -d, --detached                           Run process in background ( Only for Unix based systems)
  -h, --help                               Print help
  -V, --version                            Print version
```
