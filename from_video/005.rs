fn takes_owership(some_string:String){
    println!("{}",some_string);
}

fn takes_owership2(some_string:String) -> String{
    println!("{}",some_string);
    some_string
}

fn makes_copy(i:i32){
    println!("i = {}",i);
}

fn test()
{
    let s = String::from("hello");
    takes_owership(s);
    // println!("{}",s); //s已经被释放

    let mut s = String::from("hello");
    let s = takes_owership2(s);
    println!("{}",s);

    let x = 5;
    makes_copy(x);
    println!("{}",x);
}

fn main(){
    let x:i32 = 1;
    {
        let y:i32 = 1;
        println!("x={}",x);
        println!("y={}",y);
    }
    {
        let mut s1 = String::from("hello");//String为堆
        s1.push_str(" world");
        println!("s1 = {}",s1); //String类型离开作用域的时候会调用drop方法

        let s2 = s1;//堆移交所有权 s1 move to s2
        println!("s2 = {}",s2);
        // println!("s1 = {}",s1);// s1丧失所有权，s1 invalid

        //clone
        let s3 = s2.clone();
        println!("s3 = {}",s3);
        println!("s2 = {}",s2);
    }
    //copy trait
    let a = 1;
    let b = a; //栈上copy
    println!("a = {}, b = {}",a,b);

    //trait(栈):
    //整型
    //浮点
    //浮点
    //bool
    //char
    //元组

    test();
}