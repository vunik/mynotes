/*
Vectors - Resizable array 
*/
use std::mem;

pub fn run(){

    let mut numbers: Vec<i32> = vec![1,2,3,4];
    
    //re-assigne value 
    numbers[2] = 20;
    println!("{:?}",numbers);

    //get single value
    println!("single value : {}",numbers[0]);

    //get vector len
    println!("vector len: {}",numbers.len());

    //vector memory in use
    println!("vector occupies {} bytes",std::mem::size_of_val(&numbers));
    println!("vector occupies {} bytes",mem::size_of_val(&numbers));

    //slice vector
    let slice:&[i32] = &numbers[0..2];
    println!("Slice: {:?}",slice);

    numbers.push(20);
    numbers.push(30);
    println!("{:?}", numbers);
    numbers.pop();
    println!("{:?}", numbers);

    // loop through vector values
    for x in numbers.iter(){
        println!("Number: {}", x);
    }

    //loop mutate values, multiply x by 2
    for x in numbers.iter_mut(){
        *x *= 2;
    }
    println!("Vec : {:?}", numbers);
    
}