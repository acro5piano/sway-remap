# sway-remap

A keyboard remapper tool for Sway window manager. Still active development.

# Motivation

Sway and Wayland is awesome. It brings lots of benefit to Linux desktop environment.

When I was using X desktop envionment, there is an awesome tool called `xremap` which remap keys **based on current focused application**.

https://github.com/k0kubun/xremap

I was looking for something similar to `xremap`, but not found, so I decided to create on my own.

# Run

For Wayland security model, we have to do execute key remapping as root.

```sh
cargo build && sudo ./target/debug/sway-remap
```
