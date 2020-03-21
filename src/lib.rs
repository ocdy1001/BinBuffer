#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

//! This is a simple crate to read and write binairy data.
//! # Example:
//! ```
//! use bin_buffer::*;
//! let x = 16u16;
//! let y = String::from("hello");
//! let z = (0.0001f64,1.1111f64);
//! let mut buffer = Vec::new();
//! x.into_buffer(&mut buffer);
//! y.copy_into_buffer(&mut buffer);
//! z.into_buffer(&mut buffer);
//! let mut buffer = ReadBuffer::from_raw(buffer);
//! assert_eq!(Some(x), u16::from_buffer(&mut buffer));
//! assert_eq!(Some(y), String::from_buffer(&mut buffer));
//! assert_eq!(Some(z), <(f64,f64)>::from_buffer(&mut buffer));
//! ```
#[macro_use]
extern crate fnrs;
use fnrs::uworn;

use std::io::prelude::*;
use std::fs::OpenOptions;

/// Buffer: a Vector of bytes
pub type Buffer = Vec<u8>;

/// Buffer from which we can read.
pub struct ReadBuffer{
    buffer: Buffer,
    iter: usize,
}

impl ReadBuffer{
    /// Create ReadBuffer from Buffer.
    pub fn from_raw(vec: Buffer) -> Self{
        Self{
            buffer: vec,
            iter: 0,
        }
    }
    /// Turn ReadBuffer into Buffer.
    pub fn into_raw(self) -> Buffer{
        self.buffer
    }
    /// If the ReadBuffer is empty.
    pub fn is_empty(&self) -> bool{
        self.buffer.is_empty()
    }
}

/// Object can be read and written to a Buffer
pub trait Bufferable where Self: std::marker::Sized{
    /// Consume yourself and add to the end of the buffer
    fn into_buffer(self, vec: &mut Buffer);
    /// Copy yourself and add to the end of the buffer.
    fn copy_into_buffer(&self, vec: &mut Buffer);
    /// Read object from buffer
    fn from_buffer(buf: &mut ReadBuffer) -> Option<Self>;
}
/// Implements Bufferable for u64.
/// # Example
/// ```
/// use bin_buffer::*;
/// let x = 81234u64;
/// let mut buffer = Vec::new();
/// x.into_buffer(&mut buffer);
/// let mut buffer = ReadBuffer::from_raw(buffer);
/// let y = u64::from_buffer(&mut buffer);
/// ```
impl Bufferable for u64{
    fn into_buffer(self, vec: &mut Buffer){
        vec.push(((self >> 56) & 0xff) as u8);
        vec.push(((self >> 48) & 0xff) as u8);
        vec.push(((self >> 40) & 0xff) as u8);
        vec.push(((self >> 32) & 0xff) as u8);
        vec.push(((self >> 24) & 0xff) as u8);
        vec.push(((self >> 16) & 0xff) as u8);
        vec.push(((self >> 8) & 0xff) as u8);
        vec.push((self & 0xff) as u8);
    }

    fn copy_into_buffer(&self, vec: &mut Buffer){
        self.clone().into_buffer(vec);
    }

    fn from_buffer(buf: &mut ReadBuffer) -> Option<Self>{
        if buf.iter + 8 > buf.buffer.len() { return Option::None; }
        let mut val: u64 = 0;
        val += u64::from(buf.buffer[(buf.iter + 0)]) << 56;
        val += u64::from(buf.buffer[(buf.iter + 1)]) << 48;
        val += u64::from(buf.buffer[(buf.iter + 2)]) << 40;
        val += u64::from(buf.buffer[(buf.iter + 3)]) << 32;
        val += u64::from(buf.buffer[(buf.iter + 4)]) << 24;
        val += u64::from(buf.buffer[(buf.iter + 5)]) << 16;
        val += u64::from(buf.buffer[(buf.iter + 6)]) << 8;
        val += u64::from(buf.buffer[(buf.iter + 7)]);
        buf.iter += 8;
        Option::Some(val)
    }
}
/// Implements Bufferable for u32.
/// # Example
/// ```
/// use bin_buffer::*;
/// let x = 71u32;
/// let mut buffer = Vec::new();
/// x.into_buffer(&mut buffer);
/// let mut buffer = ReadBuffer::from_raw(buffer);
/// let y = u32::from_buffer(&mut buffer);
/// ```
impl Bufferable for u32{
    fn into_buffer(self, vec: &mut Buffer){
        vec.push(((self >> 24) & 0xff) as u8);
        vec.push(((self >> 16) & 0xff) as u8);
        vec.push(((self >> 8) & 0xff) as u8);
        vec.push((self & 0xff) as u8);
    }

    fn copy_into_buffer(&self, vec: &mut Buffer){
        self.clone().into_buffer(vec);
    }

    fn from_buffer(buf: &mut ReadBuffer) -> Option<Self>{
        if buf.iter + 4 > buf.buffer.len() { return Option::None; }
        let mut val: u32 = 0;
        val += u32::from(buf.buffer[(buf.iter + 0)]) << 24;
        val += u32::from(buf.buffer[(buf.iter + 1)]) << 16;
        val += u32::from(buf.buffer[(buf.iter + 2)]) << 8;
        val += u32::from(buf.buffer[(buf.iter + 3)]);
        buf.iter += 4;
        Option::Some(val)
    }
}
/// Implements Bufferable for u16.
/// # Example
/// ```
/// use bin_buffer::*;
/// let x = 31u16;
/// let mut buffer = Vec::new();
/// x.into_buffer(&mut buffer);
/// let mut buffer = ReadBuffer::from_raw(buffer);
/// let y = u16::from_buffer(&mut buffer);
/// ```
impl Bufferable for u16{
    fn into_buffer(self, vec: &mut Buffer){
        vec.push(((self >> 8) & 0xff) as u8);
        vec.push((self & 0xff) as u8);
    }

    fn copy_into_buffer(&self, vec: &mut Buffer){
        self.clone().into_buffer(vec);
    }

    fn from_buffer(buf: &mut ReadBuffer) -> Option<Self>{
        if buf.iter + 2 > buf.buffer.len() { return Option::None; }
        let mut val: u16 = 0;
        val += u16::from(buf.buffer[(buf.iter + 0)]) << 8;
        val += u16::from(buf.buffer[(buf.iter + 1)]);
        buf.iter += 2;
        Option::Some(val)
    }
}
/// Implements Bufferable for u8.
/// # Example
/// ```
/// use bin_buffer::*;
/// let x = 1u8;
/// let mut buffer = Vec::new();
/// x.into_buffer(&mut buffer);
/// let mut buffer = ReadBuffer::from_raw(buffer);
/// let y = u8::from_buffer(&mut buffer);
/// ```
impl Bufferable for u8{
    fn into_buffer(self, vec: &mut Buffer){
        vec.push(self);
    }

    fn copy_into_buffer(&self, vec: &mut Buffer){
        self.clone().into_buffer(vec);
    }

    fn from_buffer(buf: &mut ReadBuffer) -> Option<Self>{
        if buf.iter + 1 > buf.buffer.len() { return Option::None; }
        let val = buf.buffer[buf.iter];
        buf.iter += 1;
        Option::Some(val)
    }
}
/// Implements Bufferable for f64.
/// # Example
/// ```
/// use bin_buffer::*;
/// let x = 1.001f64;
/// let mut buffer = Vec::new();
/// x.into_buffer(&mut buffer);
/// let mut buffer = ReadBuffer::from_raw(buffer);
/// let y = f64::from_buffer(&mut buffer);
/// ```
impl Bufferable for f64{
    fn into_buffer(self, vec: &mut Buffer){
        let bytes = self.to_be_bytes();
        for b in bytes.iter(){
            vec.push(*b);
        }
    }

    fn copy_into_buffer(&self, vec: &mut Buffer){
        self.clone().into_buffer(vec);
    }

    fn from_buffer(buf: &mut ReadBuffer) -> Option<Self>{
        if buf.iter + 8 > buf.buffer.len() { return Option::None; }
        let mut bytes = [0u8; 8];
        for i in 0..8{
            bytes[i] = buf.buffer[buf.iter + i];
        }
        buf.iter += 8;
        return Option::Some(f64::from_be_bytes(bytes));
    }
}
/// Implements Bufferable for f32.
/// # Example
/// ```
/// use bin_buffer::*;
/// let x = 1.001f32;
/// let mut buffer = Vec::new();
/// x.into_buffer(&mut buffer);
/// let mut buffer = ReadBuffer::from_raw(buffer);
/// let y = f32::from_buffer(&mut buffer);
/// ```
impl Bufferable for f32{
    fn into_buffer(self, vec: &mut Buffer){
        let bytes = self.to_be_bytes();
        for b in bytes.iter(){
            vec.push(*b);
        }
    }

    fn copy_into_buffer(&self, vec: &mut Buffer){
        self.clone().into_buffer(vec);
    }

    fn from_buffer(buf: &mut ReadBuffer) -> Option<Self>{
        if buf.iter + 4 > buf.buffer.len() { return Option::None; }
        let mut bytes = [0u8; 4];
        for i in 0..4{
            bytes[i] = buf.buffer[buf.iter + i];
        }
        buf.iter += 4;
        return Option::Some(f32::from_be_bytes(bytes));
    }
}
/// Implements Bufferable for String.
/// # Example
/// ```
/// use bin_buffer::*;
/// let x = String::from("cool and good");
/// let mut buffer = Vec::new();
/// x.copy_into_buffer(&mut buffer);
/// let mut buffer = ReadBuffer::from_raw(buffer);
/// let y = String::from_buffer(&mut buffer);
/// ```
impl Bufferable for String{
    fn into_buffer(self, vec: &mut Buffer){
        self.copy_into_buffer(vec);
    }

    fn copy_into_buffer(&self, vec: &mut Buffer){
        let bytes = self.as_bytes();
        let len = bytes.len();
        (len as u64).into_buffer(vec);
        for b in bytes.iter(){
            vec.push(*b);
        }
    }

    fn from_buffer(buf: &mut ReadBuffer) -> Option<Self>{
        let len = uworn!(u64::from_buffer(buf)) as usize;
        if buf.iter + len > buf.buffer.len() { return Option::None; }
        let mut bytes = Vec::new();
        for i in 0..len{
            bytes.push(buf.buffer[buf.iter + i]);
        }
        buf.iter += len;
        return if let Ok(r) = String::from_utf8(bytes) { Some(r) }
        else { return Option::None; };
    }
}
/// Just copies the content of the second buffer to the end of the first buffer.
/// # Example
/// ```
/// use bin_buffer::*;
/// let mut buffer = vec![0,1,2,3];
/// let source = &vec![4,5];
/// buffer_append_buffer(&mut buffer, source);
/// assert_eq!(buffer,vec![0,1,2,3,4,5]);
/// ```
pub fn buffer_append_buffer(vec: &mut Buffer, string: &Buffer){
    for byte in string{
        vec.push(*byte);
    }
}
/// Writes a buffer to a file.
/// Will create a new file if none exists or overwrite otherwise.
/// # Example
/// ```
/// use bin_buffer::*;
/// let buf = vec![0,1,2];
/// let path = std::path::Path::new("./buffer0.ntbr");
/// buffer_write_file(&path, &buf);
/// let res = buffer_read_file(&path);
/// assert_eq!(res, Option::Some(buf));
/// ```
pub fn buffer_write_file(path: &std::path::Path, vec: &Buffer) -> bool{
    let file = if let Ok(f) =
        OpenOptions::new().write(true).create(true).truncate(true).open(path) { f }
    else { return false; };
    let mut opened = file;
    if opened.write_all(&vec).is_err() {return false;}
    true
}
/// Writes a buffer to the end of a file.
/// Will create a new file if none exists.
/// # Example
/// ```
/// use bin_buffer::*;
/// let a = vec![0,1,2];
/// let path = std::path::Path::new("./buffer1.ntbr");
/// buffer_write_file(&path, &a);
/// let b = vec![3,4];
/// buffer_write_file_append(&path, &b);
/// let res = buffer_read_file(&path);
/// assert_eq!(res, Option::Some(vec![0,1,2,3,4]));
/// ```
pub fn buffer_write_file_append(path: &std::path::Path, vec: &Buffer) -> bool{
    let file = if let Ok(f) =
        OpenOptions::new().write(true).create(true).append(true).open(path) { f }
    else { return false; };
    let mut opened = file;
    if opened.write_all(&vec).is_err() {return false;}
    true
}
/// Reads a buffer from a file.
/// # Example
/// ```
/// use bin_buffer::*;
/// let path = std::path::Path::new("./buffer2.ntbr");
/// let buffer = vec![0,1,2,3];
/// buffer_write_file(&path, &buffer);
/// let read_result = buffer_read_file(&path);
/// assert_eq!(read_result, Option::Some(buffer));
/// ```
pub fn buffer_read_file(path: &std::path::Path) -> Option<Buffer>{
    let file = if let Ok(f) =
        OpenOptions::new().read(true).open(path) { f }
    else { return Option::None; };
    let mut opened = file;
    let mut vec: Buffer = Vec::new();
    if opened.read_to_end(&mut vec).is_err() { return Option::None; }
    Option::Some(vec)
}
/// Implements Bufferable for Vec<Bufferable + Clone>
/// # Example
/// ```
/// use bin_buffer::*;
/// let x = vec![0.0f32,1.0,2.0,3.0,4.0,5.5];
/// let mut buffer = Vec::new();
/// x.copy_into_buffer(&mut buffer);
/// let mut buffer = ReadBuffer::from_raw(buffer);
/// let y = Vec::<f32>::from_buffer(&mut buffer);
/// ```
impl<T: Bufferable + Clone> Bufferable for Vec<T>{
    fn into_buffer(self, buf: &mut Buffer){
        let len = self.len() as u64;
        len.into_buffer(buf);
        for x in self{
            x.into_buffer(buf);
        }
    }

    fn copy_into_buffer(&self, buf: &mut Buffer){
        self.clone().into_buffer(buf);
    }

    fn from_buffer(buf: &mut ReadBuffer) -> Option<Self>{
        let len = uworn!(u64::from_buffer(buf));
        let mut vec = Vec::new();
        for _ in 0..len{
            let x = uworn!(T::from_buffer(buf));
            vec.push(x);
        }
        Option::Some(vec)
    }
}
/// Implements Bufferable for tuples where all U,V are Bufferable and Clone.
/// # Example
/// ```
/// use bin_buffer::*;
/// let x = (0.0f64,-12345.4321f64);
/// let mut buffer = Vec::new();
/// x.into_buffer(&mut buffer);
/// let mut buffer = ReadBuffer::from_raw(buffer);
/// let y =  <(f64,f64)>::from_buffer(&mut buffer);
/// ```
impl<U: Bufferable + Clone, V: Bufferable + Clone> Bufferable for (U,V){
    fn into_buffer(self, buf: &mut Buffer){
        self.0.into_buffer(buf);
        self.1.into_buffer(buf);
    }

    fn copy_into_buffer(&self, buf: &mut Buffer){
        self.clone().into_buffer(buf);
    }

    fn from_buffer(buf: &mut ReadBuffer) -> Option<Self>{
        let x = uworn!(U::from_buffer(buf));
        let y = uworn!(V::from_buffer(buf));
        Option::Some((x,y))
    }
}
/// Implements Bufferable for tuples (U,V,W) where all U,V,W are Bufferable and Clone.
/// # Example
/// ```
/// use bin_buffer::*;
/// let x = (0.0f64,-12345.4321f64,9999.0f64);
/// let mut buffer = Vec::new();
/// x.into_buffer(&mut buffer);
/// let mut buffer = ReadBuffer::from_raw(buffer);
/// let y= <(f64,f64,f64)>::from_buffer(&mut buffer);
/// ```
impl<U: Bufferable + Clone, V: Bufferable + Clone, W: Bufferable + Clone>
    Bufferable for (U,V,W){
    fn into_buffer(self, buf: &mut Buffer){
        self.0.into_buffer(buf);
        self.1.into_buffer(buf);
        self.2.into_buffer(buf);
    }

    fn copy_into_buffer(&self, buf: &mut Buffer){
        self.clone().into_buffer(buf);
    }

    fn from_buffer(buf: &mut ReadBuffer) -> Option<Self>{
        let x = uworn!(U::from_buffer(buf));
        let y = uworn!(V::from_buffer(buf));
        let z = uworn!(W::from_buffer(buf));
        Option::Some((x,y,z))
    }
}

#[cfg(test)]
mod tests{
    use crate::*;
    #[test]
    fn test_true(){
        assert_eq!(true, true);
    }
    #[test]
    fn test_u64(){
        let x = 81234u64;
        let mut buffer = Vec::new();
        x.into_buffer(&mut buffer);
        let mut buffer = ReadBuffer::from_raw(buffer);
        assert_eq!(x, u64::from_buffer(&mut buffer).unwrap());
        assert_eq!(Option::None, u64::from_buffer(&mut buffer));
    }
    #[test]
    fn test_u32(){
        let x = 71u32;
        let mut buffer = Vec::new();
        x.into_buffer(&mut buffer);
        let mut buffer = ReadBuffer::from_raw(buffer);
        assert_eq!(x, u32::from_buffer(&mut buffer).unwrap());
        assert_eq!(Option::None, u16::from_buffer(&mut buffer));
    }
    #[test]
    fn test_u16(){
        let x = 31u16;
        let y = 21u16;
        let mut buffer = Vec::new();
        x.into_buffer(&mut buffer);
        y.into_buffer(&mut buffer);
        let mut buffer = ReadBuffer::from_raw(buffer);
        assert_eq!(x, u16::from_buffer(&mut buffer).unwrap());
        assert_eq!(y, u16::from_buffer(&mut buffer).unwrap());
        assert_eq!(Option::None, u16::from_buffer(&mut buffer));
    }
    #[test]
    fn test_u8(){
        let x = 1u8;
        let y = 0u8;
        let mut buffer = Vec::new();
        x.into_buffer(&mut buffer);
        y.into_buffer(&mut buffer);
        let mut buffer = ReadBuffer::from_raw(buffer);
        assert_eq!(x, u8::from_buffer(&mut buffer).unwrap());
        assert_eq!(y, u8::from_buffer(&mut buffer).unwrap());
        assert_eq!(Option::None, u8::from_buffer(&mut buffer));
    }
    #[test]
    fn test_f64(){
        let x = 1.001f64;
        let y = 1.23456789;
        let mut buffer = Vec::new();
        x.into_buffer(&mut buffer);
        y.into_buffer(&mut buffer);
        let mut buffer = ReadBuffer::from_raw(buffer);
        assert_eq!(x, f64::from_buffer(&mut buffer).unwrap());
        assert_eq!(y, f64::from_buffer(&mut buffer).unwrap());
        assert_eq!(Option::None, f64::from_buffer(&mut buffer));
    }
    #[test]
    fn test_f32(){
        let x = 1.001f32;
        let y = 1.23456;
        let mut buffer = Vec::new();
        x.into_buffer(&mut buffer);
        y.into_buffer(&mut buffer);
        let mut buffer = ReadBuffer::from_raw(buffer);
        assert_eq!(x, f32::from_buffer(&mut buffer).unwrap());
        assert_eq!(y, f32::from_buffer(&mut buffer).unwrap());
        assert_eq!(Option::None, f32::from_buffer(&mut buffer));
    }
    #[test]
    fn test_string(){
        let x = String::from("haha yes cool and good");
        let y = 16u16;
        let z = String::from("another one");
        let mut buffer = Vec::new();
        x.copy_into_buffer(&mut buffer);
        y.into_buffer(&mut buffer);
        z.copy_into_buffer(&mut buffer);
        let mut buffer = ReadBuffer::from_raw(buffer);
        assert_eq!(x, String::from_buffer(&mut buffer).unwrap());
        assert_eq!(y, u16::from_buffer(&mut buffer).unwrap());
        assert_eq!(z, String::from_buffer(&mut buffer).unwrap());
        assert_eq!(Option::None, String::from_buffer(&mut buffer));
    }
    #[test]
    fn test_f64_tuple(){
        let x = (0.0f64,-12345.4321f64);
        let mut buffer = Vec::new();
        x.into_buffer(&mut buffer);
        let mut buffer = ReadBuffer::from_raw(buffer);
        assert_eq!(x, <(f64,f64)>::from_buffer(&mut buffer).unwrap());
    }
    #[test]
    fn test_f64_triple(){
        let x = (0.0f64,-12345.4321f64,9999.0f64);
        let mut buffer = Vec::new();
        x.into_buffer(&mut buffer);
        let mut buffer = ReadBuffer::from_raw(buffer);
        assert_eq!(Some(x), <(f64,f64,f64)>::from_buffer(&mut buffer));
        assert_eq!(None, u8::from_buffer(&mut buffer));
    }
    #[test]
    fn test_vec(){
        let x = vec![0.0f32,1.0,2.0,3.0,4.0,5.5];
        let mut buffer = Vec::new();
        x.copy_into_buffer(&mut buffer);
        let mut buffer = ReadBuffer::from_raw(buffer);
        assert_eq!(Some(x), Vec::<f32>::from_buffer(&mut buffer));
        assert_eq!(None, u8::from_buffer(&mut buffer));
    }
}
