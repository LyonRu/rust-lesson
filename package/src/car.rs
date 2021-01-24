
//mod car
pub fn create() {
    println!("create a car")
}


//不添加pub，但是模块内可以访问
mod seat{
 pub  fn create_seat(){
        println!("create seats");
    }
}

pub use crate::car::seat::create_seat;  //重导出，外部模块可以通过  car::create_seat()访问

pub fn use_create_seat(){
    seat::create_seat();//模块内访问
}