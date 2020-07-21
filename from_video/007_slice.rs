fn main(){
    let s = String::from("hello wolrd");
    let h = &s[0..5];//左闭右开
    println!("{}",h);
    let h = &s[0..=4];
    println!("{}",h);
    let h = &s[..5];
    println!("{}",h);
    let h = &s[..=4];
    println!("{}",h);

    let w = &s[6..11];
    println!("{}",w);
    let w = &s[6..=10];
    println!("{}",w);
    let w = &s[6..];
    println!("{}",w);
    let w = &s[..];
    println!("{}",w);

    let s2 = String::from("你好");
    let w1 = &s2[0..3];
    println!("{}",w1);

    let a = [1,2,3,4];
    let ss = &a[1..3];
    //ss是整型数组不是字符串，不能这样打印
    // println!("ss = {}",ss); 
    println!("ss = {}",ss[0]);
    println!("ss = {}",ss[1]);
    println!("ss = {}",ss[2]);
}