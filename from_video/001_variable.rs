const M:u32 =10000;
fn main(){
    //定义变量 let name:type = data
    let a = 1;
    println!("a={}",a);

    //变量没用 mut 不可变
    let mut b:u32 = 1;
    println!{"b={}",b};
    b =2;
    println!{"b={}",b};

    //隐藏
    let b:f32 = 1.1;
    println!{"b={}",b};

    //常量
    println!{"M={}",M};
}