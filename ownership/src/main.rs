fn main() {
    println!("Hello, world!");


    let s1 = String::from("origin");
    let s2 = s1;//var s2 own "orgin" ,s1 was moved

    // println!("{}",s1);//compile error,s1 was moved
    println!("s2 : {}",s2);

    let s3 = String::from("taking by fn");
    taking_ownership(s3);//s3 was borrowd
    // println!("{}",s3);//compile error

    let s4 = give_ownership();
    println!("after give ownership,s4 value : {}",s4);

    let s5 = String::from("something to borrow");
    borrow_var(&s5);
    println!("s5 was borrowd,{}",s5);

    let mut s6 = String::from("hello");
    borrow_and_change(&mut s6);
    println!("s6 : {}",s6);
}
pub fn give_ownership()->String{
    let _s = String::from("give ownership");
    _s
}
pub fn taking_ownership(name:String){
    println!("taking {} 's ownership",name)
}

pub fn borrow_var (name :&String){
    println!("borrow value : {} ",name);
    //name is immutable reference , it can not be modified
}

pub fn borrow_and_change(name:&mut String){
    name.push_str(",world");
}
