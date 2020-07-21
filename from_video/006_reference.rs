fn main(){
    let s1 = give_owership();
    println!("{}",s1);

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    // println!("{}",s2);//error s2被move 已经被释放
    println!("{}",s3);

    //引用 &
    //创建一个指向值的引用，但不会拥有，因为不拥有，所以，当引用结束生命周期不会被释放
    let s1 = String::from("test");
    let len = calcute_length(s1);
    println!("{}",len); 
    // println!("{}",s1);//被move释放，不能使用

    let s2 = String::from("test");
    let len = calcute_length2(&s2);//使用引用,引用是不能修改
    println!("{}",len); 
    println!("{}",s2);

    //借用，可以修改
    let mut s1 = String::from("hello");
    modify_s(&mut s1);
    println!("s1 = {}",s1);

    let ms = &mut s1;
    modify_s(ms);
    println!("s1 = {}",s1);

    // 借用后不能使用引用量，因为借用会导致引用的值出现变化，为不安全，非预期
    // let r1 = &s1;
    // let r2 = &s1;
    // let r3 = &mut s1;
    // println!("{},{},{}",r1,r2,r3);

    // 引用量不使用了，那便可以借用，因为没有超出预期的变化
    let r1 = &s1;
    let r2 = &s1;
    println!("{},{}",r1,r2);
    let r3 = &mut s1;
    println!("{}",r3);
    //注意不能引用被释放的值
}

fn give_owership() -> String {
    let s = String::from("hello");
    s
}

fn takes_and_gives_back(s:String) ->String {
    s
}

fn calcute_length(s:String) -> usize{
    s.len()
}

fn calcute_length2(s:&String) -> usize{
    s.len()
}

fn modify_s(s:&mut String){
    s.push_str(",world");
}
