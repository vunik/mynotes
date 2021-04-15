/*
primitive str = immutable fixed-length string
String = growable, heap-allocated data structure - Used when you need to modify or own string data 
*/
pub fn run(){
    //primitive str
    let hello = "Hello";

    // String
    let new_hello = String::from("Helloooo");
    println!("{}", new_hello);
    
    // str length
    println!("{}",new_hello.len());

    let mut new_mut_hello = String::from("Hello ");
    // push char
    new_mut_hello.push('W');
    // push string
    new_mut_hello.push_str("orld!!");
    
    println!("{}",new_mut_hello);

    //capacity in bytes
    println!("capacity: {}",new_mut_hello.capacity());

    //check str empty
    println!("is empty: {}",new_mut_hello.is_empty());

    // contains sub string
    println!("Containes 'Hello' = {}", new_mut_hello.contains("Hello"));

    // replace
    println!("Replace: {}", new_mut_hello.replace("World","There"));

    // loop throung string by whitespace
    for word in new_mut_hello.split_whitespace(){
        println!("{}", word);
    }

    // create string with capacity
    let mut s = String::with_capacity(10);    
    s.push('a');
    s.push('b');

    //assertion testing
    assert_eq!(2,s.len());
    assert_eq!(10, s.capacity());
    
}