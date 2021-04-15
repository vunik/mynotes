/* 
variables hold primitive data or references to data
varibles are immutable by default
rust is a block-scoped language
*/

pub fn run(){
    let name = "vishal";
    let mut age = 27;
    age = 28;
    println!("my name is {}, i am {} yrs old",name, age);

    //define constant, also const should be in caps
    const ID: i32 = 001;
    println!("{}",ID);

    // multiple assignment
    let (x,y)=(1,2);
    println!("{} {}",x,y);
}