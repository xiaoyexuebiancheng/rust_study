fn main(){
    //borol
    let is_true:bool = true;
    println!("is_true={}",is_true);

    let is_false = false;
    println!("is_false= {},{}",is_false,is_true);

    //char 在rust里，char是32位
    let a = 'a';
    println!("a={}",a);

    let b='你';
    println!("b={}",b);

    //i8,i16,i32,i64,u8,u16,u32,u64,f32,f64
    let c:i8 = -127;  //不能写128，正负区分；
    println!("c={}",c);

    let d:f32 = 0.12345678999;
    println!("d={}",d); 

    //自适应类型isize,usize
    println!("x={}",usize::max_value());

    //数组
    //[type; size]
    let arr:[u32;5] = [1,2,3,4,5];
    println!("arr[0]={}",arr[0]);
    show(arr);

    //元组
    let tup:(i32,f32,char) = (-3,3.69,'好');
    println!("{},{},{}",tup.0,tup.1,tup.2);
    
    let (x,y,z) = tup;//元组拆解
    println!("{},{},{}",x,y,z);
}

fn show(arr:[u32;5]){
    for i in &arr{
        println!("{}",i);
    }
}