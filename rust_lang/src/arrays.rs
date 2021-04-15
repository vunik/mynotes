/*
Arrays - fixed list where elements are of same data types
*/
use std::mem;

pub fn run(){
    let num:[i32;4]=[1,2,3,4];
    println!("{:?}",num);

    let mut numbers:[i32;4]=[1,2,3,4];
    
    //re-assigne value 
    numbers[2] = 20;
    println!("{:?}",numbers);

    //get single value
    println!("single value : {}",numbers[0]);

    //get array len
    println!("array len: {}",numbers.len());

    //array memory in use
    println!("array occupies {} bytes",std::mem::size_of_val(&numbers));
    println!("array occupies {} bytes",mem::size_of_val(&numbers));

    //slice array
    let slice:&[i32] = &numbers[0..2];
    println!("Slice: {:?}",slice);

    
}