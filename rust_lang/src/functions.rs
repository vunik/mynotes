pub fn run(){
    greeting("Hello", "Jane");
    println!("sum: {}",add(1,2));

    //closure
    let add_nums = |x:i32,y:i32|x+y;
    println!("c sum: {}",add_nums(3,3));
}

fn greeting(greet:&str, name:&str){
    println!("{} {}, nice to meet you!", greet, name);
}

fn add(x:i32, y:i32) -> i32{
    x+y
}