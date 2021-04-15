/*
primitive types:
Integer: u8,i8,u16,i16,u32,i32,u64,i64,u128,i128(number of bits they take in memory)
Float: f32,f64
Boolean (bool)
Character (char)
Tuples
Arrays

Note: Rust is a statically typed language, which means that it must know
the types of all varibles at compile time, however the compiler can usually interpret
what type we want to use based on the value and how we use it. 
*/

pub fn run(){
    //default i32
    let x = 1; 

    //default f64
    let y = 2.5;

    //explicitly specify type
    let z:i64 = 43453434343434;
    
    //find max size
    println!("max i32: {}",std::i32::MAX);
    println!("max i64: {}",std::i64::MAX);

    // boolean
    let is_active = true;

    let is_greater = 10>5;

    let a1 = "a";
    let face = "\u{1F600}";

    println!("{:?}",(x,y,z,is_active, is_greater,a1, face));
}