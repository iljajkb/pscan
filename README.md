# PSCAN - Simple Port Scanner, written in Rust

This is a small hobby project of mine. I like hacking/pentesting and wanted to learn more about networking.
Moreover, this gave me the chance to learn a new and cool programming language - Rust! 

## Install and Run!

- `cd pscan`
- `cargo install --path .`
- `pscan <HOST>`

or run without installing:

- `cargo run -- <HOST>`
- `cargo run -- <HOST> -p- -s 1024` (full scan of 65k ports, takes ~11s)

## Current Features

- connect scan (3-way-handshake via TCP)
- async scanning with *tokio*, you can set the speed of the scan via `-s`, where the parameter is the amount of parallel requests (no more than `1024` recommended/needed)
- `-p` allows to specify the ports to scan, default is `1-1000`
  - `-p-` scans all `65535`ports
  - `-p 80,443,22`
  - `-p 10-100`
- service detection via `.json`-map
- Stealthy SYN scan: this is currently not working on my macOs-machine, because the macOS kernel is much more restrictive about TCP packets and working with raw sockets is not working currently

## Upcomping Features

- make SYN scans work for macOS, may need to work with Layer 2 (datalink) sniffing, which makes this much more complex and not very stable for VPN connections 
