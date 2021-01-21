mod var;

fn main() {
    // let x = 12;
    // x is a immuable variable,
    //x = 32;

    let s = String::from("hello");
    take_variables(s);
    // let s1 = s;//s is moved,
    //
    // println!("s is taken {}", s);//compile error


    let s2 = String::from("world");
    borrow_variables(&s2);
    println!("s2 value {}", s2);
    let i = 30;
    let i1 = i;
    println!("i is copied {}", i);


    // 可变引用，可修改其指向的值
    let mut  str = String::from("de");
    change_mut_refer_str(&mut str)
}

pub fn change_mut_refer_str(s:&mut String){
    s.push_str("mo");
    println!(s)
}

pub fn take_variables(s: String) {
    println!("i am taking s {}", s)
}


pub fn borrow_variables(s: &String) {
    println!("i am borrow s {}", s)
}