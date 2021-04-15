pub fn run(){
    //print to console
    println!("hello from print from function ");
    
    // basic formating
    println!("{}",1);

    //postitional arguments
    println!("i am {}, working with {}","vishal", "redhat");

    //named arguments
    println!("i am {name}, working with {work}",name="vishal", work="redhat");

    // placeholder traits
    println!("{:?}", (12, true , "hello"));

    //basic math
    println!("10 + 10 = {}", 10+10);
}