// loops - used to iterate until a condition is met
pub fn run(){
    let mut count = 0;

    // loop{
    //     count+=1;
    //     println!("{}",count);
    //     if count == 20{
    //         break;
    //     }
    // }

    //while loop(fizzbuzz)
    while count < 20{
        if count % 15 == 0{
            println!("fizzbuzz");
        }
        else if count % 3 == 0 {
            println!("fizz");
        }
        else if count % 5 == 0{
            println!("buzz");
        }
        else{
            println!("{}", count)
        }
        count+=1   
    }

    // for range
    for x in 0..20{
        println!("{}",x);
    }

}

