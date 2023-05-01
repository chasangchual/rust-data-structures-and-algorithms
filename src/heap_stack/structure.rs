use std::mem;

struct MyStructure {
    a: u32,
    b: u8,
    c: u8
}

pub fn size_of_structure() {
    println!("mem::size_of::<u8>() is {}", mem::size_of::<u8>());
    println!("mem::size_of::<u8>() is {}", mem::size_of::<u32>());
    println!("mem::size_of::<MyStructure>() is {}", mem::size_of::<MyStructure>());
    println!("mem::size_of::<MyStructure>() is {}", mem::size_of::<[MyStructure; 2]>());
}