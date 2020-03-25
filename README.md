![Rust](https://github.com/ocdy1001/bin-buffer/workflows/Rust/badge.svg)
[docs](https://docs.rs/bin_buffer/)
# BinBuffer
Simple lightweight crate for translating objects into binairy buffers.
It supports reading and writing these types: u64, u32, u16, u8, f64, f32, (f64,f64), String, Vec<Bufferable + Clone>, (U,V), (U,V,W), (U,V,W,X) where U,V,W are Bufferable.
This is a simple crate to read and write binairy data.
## Example:
```rust
use bin_buffer::*;
let x = 16u16;
let y = String::from("hello");
let z = (0.0001f64,1.1111f64);
let mut buffer = Vec::new();
x.into_buffer(&mut buffer);
y.copy_into_buffer(&mut buffer);
z.into_buffer(&mut buffer);
let mut buffer = ReadBuffer::from_raw(buffer);
assert_eq!(Some(x), u16::from_buffer(&mut buffer));
assert_eq!(Some(y), String::from_buffer(&mut buffer));
assert_eq!(Some(z), <(f64,f64)>::from_buffer(&mut buffer));
```
