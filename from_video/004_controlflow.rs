fn main(){
    //if
    let y = 1;
    if y==1 {
        println!("y=1");       
    } else {
        println!("y!=1");
    }

    println!("+++++");
    let y = 2;
    if y==1 {
        println!("y=1");       
    } else if y==2 {
        println!("y=2");
    } else {
        println!("other");
    }

    //let中使用if
    let condition = true;
    let x = if condition{
        "kdjlf"
    } else {
        "kdsl"
    };
    println!("{}",x);

    let mut counter = 0;

    loop {
        println!("in loop");
        if counter == 10{
            break;
        }
        counter += 1;
    }

    let result = loop{
        counter += 1;
        if counter ==20 {
            break counter*2;
        }
    };
    println!("{}",result);

    //while
    let mut i = 0;
    while i != 10 {
        i += 1;
    }
    println!("i = {}",i);

    //for
    let arr:[u32;5] = [1,2,3,4,5];
    for element in arr.iter() {
        println!("element = {}",element);
    }
}