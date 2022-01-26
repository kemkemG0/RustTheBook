fn test(st:String)->String{
    println!("{st}");
    let test = st;
    let test = test;
    let test = test;
    test
}


fn main() {
    let mut a = String::from("test");
    a = test(a);
    println!("{a}");
}
