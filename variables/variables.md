### rust变量

> rust 默认变量是不可变的，如让变量可变加上mut关键词

```rust

let x = 12 
x = 13 // compile error , x is a immutable

let mut x = 12
x = 13  // compile success


```