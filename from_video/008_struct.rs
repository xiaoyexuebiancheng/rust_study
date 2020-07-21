fn main(){
    #[derive(Debug)] //打印结构体语句
    struct User {
        name:String,
        count:String,
        nonce:u64,
        active:bool,
    };

    let mut xiaoming = User {
        name:String::from("xiaoming"),
        count:String::from("123456"),
        nonce:1000,
        active:true,
    };
    println!("{}{}{}{}",xiaoming.name,xiaoming.count,xiaoming.nonce,xiaoming.active);
    xiaoming.nonce = 2000000;

    let name = String::from("xiaoming");
    let count = String::from("8903333");
    let nonce = 200000;
    let active = false;
    //变量与结构体元素同名可以直接带入
    let user1 = User{
        name,
        count,
        nonce,
        active,
    };
    println!("{}{}{}{}",user1.name,user1.count,user1.nonce,user1.active);

    let user2 = User{
        ..user1
    };
    println!("{}{}{}{}",user2.name,user2.count,user2.nonce,user2.active);

    //打印
    println!("user2={:?}",user2);
    println!("user2={:#?}",user2);
    //元组结构体
    //字段没有名字
    //圆括号
    struct Point(i32, i32);
    let a = Point(10,20);
    println!("a.x={},a.y={}",a.0,a.1);

    //没有如何字段的结构体
    struct A{};

}
