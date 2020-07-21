const MAX_POINTS:u32 = 100_00;//定义一个常量
// let mut a = 5; //rust没有全局变量，所以函数外定义变量报错
fn main() {
    println!("const number:{}",MAX_POINTS);
    
    let a = 6;  //创建变量，并默认自动识别变量类型，把6绑定给a
                //rust里=号不是叫赋值，叫绑定或修改绑定
    // a = 7;   //没有加mut是不可修改绑定的值
    println!("a = {}",a);

    let mut b = 6;  //加了mut关键字用来表示b的绑定值可以修改
    println!("b = {}", b);    
    b = 8;
    println!("b = {}", b);

    let c = 6;
    println!("c = {}",c);
    // c = 7;       //报错
    let c = 7;      //此处用了let方式去隐藏前面的c变量
    println!("c = {}",c);   //此方式会创建新的变量 
    let c:f32 = 5.4;    //因此隐藏类型可以用来改变c的类型
    println!("c = {}",c);

    let s = "String";
    println!("s = {}",s);
    // s = s.len();     //类型不匹配，报错
    let s = s.len();    //用隐藏类型来实现改变s类型的例子
    println!("s = {}",s);
    println!("\n");

    //另一个隐藏与解隐藏的例子
    let x = 6;
    {
        let x = "String";   //x = 6被隐藏
        println!("{}",x);
    }
    println!("{}",x);   //x = "String"生命周期结束，x = 6被解隐藏

    //四种基本type
    //整形推荐i32，总体最快
    //浮点推荐f64，与f32速度一样但是精度更高
    //布尔型，单字节空间(8bit)
    //字符型，rust中char是4字节(32bit)，一个utf-8码
    

    //复合类型:元组和数组
    //元组
    let tup:(i32,f64,u8) = (500,6.4,1);
    println!("tup0 = {}",tup.0);
    println!("tup1 = {}",tup.1);
    println!("tup2 = {}\n",tup.2);

    //解析元组
    let (x,y,z) = tup;
    println!("x = {}",x);
    println!("y = {}",y);
    println!("z = {}\n",z);

    //数组
    let a = [1,2,3,4,5];
    println!("a[0] = {}",a[0]);
    println!("a = {:?}",a); //通过{:?}单行输出全部a元素
    println!("a = {:#?}",a); //而通过{:#?}是分行输出全部元素
    let a = ["ab","cd","efg"];
    println!("a = {:?}",a);
    let a:[i32;5]; //(与其他主流语言数组内几个元素表示方法不一样)
    // println!("{}",a[0]);//输出没有初始化元素，报错
    //let mut b:[i32;5];//数组不需要用mut,加mut报warning
    
    a = [6,7,8,9,10];
    println!("{:?}\n",a);
    // println!("{}",a[10]);//越界访问，报错

    fun();//调用函数

    //语句与表达式(语言末尾有分号)
    // let y = (let x = 6); //rust语句没有返回值,报错
    //表达式：5+6是表达式返回11，let y = 6;中6也是表达式返回自己
    let y = {
        let x = 3;
        // x = x+1  //语句无值，报错
        x+1 //表达式
    };
    println!("y = {}\n",y);

    //控制流(if,loop,while,for)
    //if
    let number = 3;
    //多重判断
    if number < 5 {
        println!("number < 5");
    } else if number == 5 {
        println!("number = 5");
    } else {
        println!("number > 5");
    }
    // 一条判断 if{}   二选判断 if{}else{}
    // if 5{println!("5")} //报错，if判断的必须是bool型
                //直接判断0与1一样报错，会被判断为整型，必须是true与false

    //if在let语句中运用
    let x = if number == 5{
        5
    } else{
        3
    };  //一直到这个分号才算一条let语句
    println!("x = {}\n",x);

    // let x = if number == 5{
    //     "six"
    // } else{
    //     3
    // };   //表达式类型不匹配，报错

    // loop {
    //     println!("this is loop");
    // }    //loop 会不断循环除非有break
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 10 {
            break count *2  //此处break可加可不加分号
        }
    };
    println!("result = {}\n",result);

    //while
    let a = [10,20,30,40,50];
    let mut index = 0;
    while index < 5{
        println!("a[{}] = {}", index, a[index]);
        index += 1;
    }
    println!("\n");

    //for
    let a = [10,20,30,40,50];
    for a in a.iter(){  //按顺序每次循环都取一个元素给a
        println!("value is:{}",a);
    }
    println!("a[1] = {}",a[1]);

    for a in (1..4).rev(){  //(1..4)左闭右开区间，rev()逆序
        println!("{}",a);
    }
}


//××××××××××××××××××××××××××
//函数
fn fun(){
    println!("This is function!");

    //函数的嵌套调用
    fun2(2,2.0);//fun(2,2);报错，rust很注重类型，浮点必须有x.x
}

//带参函数
fn fun2(x:i32, y:f32){
    println!("function2");
    println!("x = {},y = {}\n",x,y);
    let x = fun3(5);
    println!("fun3 = {}\n",x);
}

//有返回值的带参函数
fn fun3(x:i32) -> i32{
    x   //rust返回值是操作数或者return x;用c语言风格
}
