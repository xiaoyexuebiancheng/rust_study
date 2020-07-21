fn other_fun(){
    println!("this is a function");
}

fn other_fun1(a:i32,b:i32){//与c类似必须有类型
    println!("{},{}",a,b);
}

fn other_fun2(a:i32,b:i32)->i32{
    let result = a + b;
    return result;
}

// fn other_fun3(a:i32,b:u32)->i32{
//     let result = a + b;
//     result
// }

fn main(){
    other_fun();

    // let a:i32 = 1;
    // let b:i32 = 2;
    let a:i32 = 1;
    let b:i32 = 2;
    // other_fun1(a, b);
    let d = other_fun2(a, b);
    println!("{}",d);
    println!("{}",other_fun2(a, b));
    //other_fun3(1, 2);
    let y = 1;//rust语句是操作指令，无返回值
    // let x = (let y =1);  错误

    //表达式会计算一些值
    let y = {
        let x =1;
        x+1};
    println!("{}",y);
}