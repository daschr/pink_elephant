# Build
- `sudo apt install qemu-user-static qemu-system-mips`
- `docker pull messense/rust-musl-cross:mipsel-musl`
- `alias rust-musl-builder='docker run --rm -it -v "$(pwd)":/home/rust/src messense/rust-musl-cross:mipsel-musl'`
- `rust-musl-builder cargo build  --release --target mipsel-unknown-linux-musl`

# Running
- `qemu-mipsel-static  ./target/mipsel-unknown-linux-musl/release/pink_elephant`
- you should see something like
  ```f0f0f0f00f0f0f0f
  0f0f0f0ff0f0f0f0
  f0f0f0f00f0f0f0f
  f0f0f0f00f0f0f0f
  f0f0f0f00f0f0f0f
  f0f0f0f00f0f0f0f
  f0f0f0f00f0f0f0f
  f0f0f0f00f0f0f0f
  0f0f0f0ff0f0f0f0
  f0f0f0f00f0f0f0f
  f0f0f0f00f0f0f0f
  f0f0f0f00f0f0f0f
  f0f0f0f00f0f0f0f
  f0f0f0f00f0f0f0f
  f0f0f0f00f0f0f0f
  ```
