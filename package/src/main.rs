use package_test::tesla::drive;//有lib.rs 的引用


use package_test::car;

mod factory;//模块引用
mod utils;//模块引用





fn main() {
    println!("Hello, world!");
    car::create();
    drive();
    factory::wheel_fac::create_wheel();

    car::use_create_seat();
    car::create_seat();
}
