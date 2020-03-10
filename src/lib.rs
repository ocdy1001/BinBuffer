use std::io::prelude::*;
use std::fs::OpenOptions;

pub type Buffer = Vec<u8>;

pub trait Bufferable where Self: std::marker::Sized{
    fn into_buffer(self, vec: &mut Buffer);
    fn copy_into_buffer(&self, vec: &mut Buffer);
    fn from_buffer(vec: &Buffer, iter: &mut usize) -> Option<Self>;
}

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

    fn from_buffer(vec: &Buffer, iter: &mut usize) -> Option<Self>{
        if *iter + 4 > vec.len() { return Option::None; }
        let mut val: u32 = 0;
        val += u32::from(vec[(*iter + 0)]) << 24;
        val += u32::from(vec[(*iter + 1)]) << 16;
        val += u32::from(vec[(*iter + 2)]) << 8;
        val += u32::from(vec[(*iter + 3)]);
        *iter += 4;
        Option::Some(val)
    }
}

impl Bufferable for u16{
    fn into_buffer(self, vec: &mut Buffer){
        vec.push(((self >> 8) & 0xff) as u8);
        vec.push((self & 0xff) as u8);
    }

    fn copy_into_buffer(&self, vec: &mut Buffer){
        self.clone().into_buffer(vec);
    }

    fn from_buffer(vec: &Buffer, iter: &mut usize) -> Option<Self>{
        if *iter + 2 > vec.len() { return Option::None; }
        let mut val: u16 = 0;
        val += u16::from(vec[(*iter + 0)]) << 8;
        val += u16::from(vec[(*iter + 1)]);
        *iter += 2;
        Option::Some(val)
    }
}

impl Bufferable for u8{
    fn into_buffer(self, vec: &mut Buffer){
        vec.push(self);
    }

    fn copy_into_buffer(&self, vec: &mut Buffer){
        self.clone().into_buffer(vec);
    }

    fn from_buffer(vec: &Buffer, iter: &mut usize) -> Option<Self>{
        if *iter + 1 > vec.len() { return Option::None; }
        let val = vec[*iter];
        *iter += 1;
        Option::Some(val)
    }
}

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

    fn from_buffer(vec: &Buffer, iter: &mut usize) -> Option<Self>{
        if *iter + 8 > vec.len() { return Option::None; }
        let mut bytes = [0u8; 8];
        for i in 0..8{
            bytes[i] = vec[*iter + i];
        }
        *iter += 8;
        return Option::Some(f64::from_be_bytes(bytes));
    }
}

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

    fn from_buffer(vec: &Buffer, iter: &mut usize) -> Option<Self>{
        if *iter + 4 > vec.len() { return Option::None; }
        let mut bytes = [0u8; 4];
        for i in 0..4{
            bytes[i] = vec[*iter + i];
        }
        *iter += 4;
        return Option::Some(f32::from_be_bytes(bytes));
    }
}

impl Bufferable for String{
    fn into_buffer(self, vec: &mut Buffer){
        self.copy_into_buffer(vec);
    }

    fn copy_into_buffer(&self, vec: &mut Buffer){
        let bytes = self.as_bytes();
        let len = bytes.len();
        (len as u32).into_buffer(vec);
        for b in bytes.iter(){
            vec.push(*b);
        }
    }

    fn from_buffer(vec: &Buffer, iter: &mut usize) -> Option<Self>{
        let len = u32::from_buffer(vec, iter); //TODO: use usize
        if len.is_none() { return Option::None; }
        let len = len.unwrap() as usize;
        if *iter + len > vec.len() { return Option::None; }
        let mut bytes = Vec::new();
        for i in 0..len{
            bytes.push(vec[*iter + i]);
        }
        *iter += len;
        let res = String::from_utf8(bytes);
        if res.is_err() { return Option::None; }
        return Some(res.unwrap());
    }
}

impl Bufferable for (f64,f64){
    fn into_buffer(self, vec: &mut Buffer){
         self.0.into_buffer(vec);
         self.1.into_buffer(vec);
    }

    fn copy_into_buffer(&self, vec: &mut Buffer){
        self.into_buffer(vec);
    }

    fn from_buffer(vec: &Buffer, iter: &mut usize) -> Option<Self>{
        if *iter + 16 > vec.len() { return Option::None; }
        let x = f64::from_buffer(vec, iter);
        if x.is_none() { return Option::None; }
        let y = f64::from_buffer(vec, iter);
        if y.is_none() { return Option::None; }
        Option::Some((x.unwrap(),y.unwrap()))
    }
}

pub fn buffer_append_buffer(vec: &mut Buffer, string: &Buffer){
    for byte in string{
        vec.push(*byte);
    }
}

pub fn buffer_write_file(path: &std::path::Path, vec: &Buffer) -> bool{
    let file = OpenOptions::new().write(true).create(true).truncate(true).open(path);
    if file.is_err() { return false; }
    let mut opened = file.unwrap();
    if opened.write_all(&vec).is_err() {return false;}
    true
}

pub fn buffer_write_file_append(path: &std::path::Path, vec: &Buffer) -> bool{
    let file = OpenOptions::new().write(true).create(true).append(true).open(path);
    if file.is_err() { return false; }
    let mut opened = file.unwrap();
    if opened.write_all(&vec).is_err() {return false;}
    true
}

pub fn buffer_read_file(path: &std::path::Path) -> Option<Buffer>{
    let file = OpenOptions::new().read(true).open(path);
    if file.is_err() {return Option::None;}
    let mut opened = file.unwrap();
    let mut vec: Buffer = Vec::new();
    if opened.read_to_end(&mut vec).is_err() {return Option::None;}
    Option::Some(vec)
}

#[cfg(test)]
mod tests{
    use crate::Bufferable;
    #[test]
    fn test_true(){
        assert_eq!(true, true);
    }
    #[test]
    fn test_u32(){
        let x = 71u32;
        let mut buffer = Vec::new();
        x.into_buffer(&mut buffer);
        let mut iter = 0;
        assert_eq!(x, u32::from_buffer(&buffer, &mut iter).unwrap());
        assert_eq!(Option::None, u16::from_buffer(&buffer, &mut iter));
    }
    #[test]
    fn test_u16(){
        let x = 31u16;
        let y = 21u16;
        let mut buffer = Vec::new();
        x.into_buffer(&mut buffer);
        y.into_buffer(&mut buffer);
        let mut iter = 0;
        assert_eq!(x, u16::from_buffer(&buffer, &mut iter).unwrap());
        assert_eq!(y, u16::from_buffer(&buffer, &mut iter).unwrap());
        assert_eq!(Option::None, u16::from_buffer(&buffer, &mut iter));
    }
    #[test]
    fn test_u8(){
        let x = 1u8;
        let y = 0u8;
        let mut buffer = Vec::new();
        x.into_buffer(&mut buffer);
        y.into_buffer(&mut buffer);
        let mut iter = 0;
        assert_eq!(x, u8::from_buffer(&buffer, &mut iter).unwrap());
        assert_eq!(y, u8::from_buffer(&buffer, &mut iter).unwrap());
        assert_eq!(Option::None, u8::from_buffer(&buffer, &mut iter));
    }
    #[test]
    fn test_f64(){
        let x = 1.001f64;
        let y = 1.23456789;
        let mut buffer = Vec::new();
        x.into_buffer(&mut buffer);
        y.into_buffer(&mut buffer);
        let mut iter = 0;
        assert_eq!(x, f64::from_buffer(&buffer, &mut iter).unwrap());
        assert_eq!(y, f64::from_buffer(&buffer, &mut iter).unwrap());
        assert_eq!(Option::None, f64::from_buffer(&buffer, &mut iter));
    }
    #[test]
    fn test_f32(){
        let x = 1.001f32;
        let y = 1.23456;
        let mut buffer = Vec::new();
        x.into_buffer(&mut buffer);
        y.into_buffer(&mut buffer);
        let mut iter = 0;
        assert_eq!(x, f32::from_buffer(&buffer, &mut iter).unwrap());
        assert_eq!(y, f32::from_buffer(&buffer, &mut iter).unwrap());
        assert_eq!(Option::None, f32::from_buffer(&buffer, &mut iter));
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
        let mut iter = 0;
        assert_eq!(x, String::from_buffer(&buffer, &mut iter).unwrap());
        assert_eq!(y, u16::from_buffer(&buffer, &mut iter).unwrap());
        assert_eq!(z, String::from_buffer(&buffer, &mut iter).unwrap());
        assert_eq!(Option::None, String::from_buffer(&buffer, &mut iter));
    }
    #[test]
    fn test_f64_tuple(){
        let x = (0.0f64,-12345.4321f64);
        let mut buffer = Vec::new();
        x.into_buffer(&mut buffer);
        let mut iter = 0;
        assert_eq!(x, <(f64,f64)>::from_buffer(&buffer, &mut iter).unwrap());
    }
}
