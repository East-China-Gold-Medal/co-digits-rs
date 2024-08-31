# [Developing] 通过 Rust 编程来学习计算机组成原理中的数字表示

## 项目介绍

这个项目想通过模拟对无符号，有符号数字的描述，来构造一个自己的无符号数和有符号数结构体，
目前实现的功能有从字面量构造数字，这些数字暂时用32位 bool 数组存储

```rust
pub struct UInt32 {
    bits: [bool; 32]
}

pub struct Int32 {
    bits: [bool; 32]
}
```

下一步，添加8，16，64，128位数字，并对其进行加减乘除操作
在下一步，加入浮点数的支持

## 项目测试

在主函数中

```rust
use co_digits_rs::{Int32, Number, UInt32};


fn main() {
    let number_1 = UInt32::from(2 as u32);
    let number_2 = Int32::from(1 as i32);

    println!("original code: {}", number_1.decode_original_code());
    println!("ones complement: {}", number_1.decode_ones_complement());
    println!("twos complement: {}", number_1.decode_twos_complement());

    println!("debug, check the bits: {}", number_1.bits_string());

    println!("original code: {}", number_2.decode_original_code());
    println!("ones complement: {}", number_2.decode_ones_complement());
    println!("twos complement: {}", number_2.decode_twos_complement());

    println!("debug, check the bits: {}", number_2.bits_string());
}
```

有运行结果

```bash
original code: 2
ones complement: 2
twos complement: 2
debug, check the bits: 00000000000000000000000000000010
original code: 1
ones complement: 1
twos complement: 1
debug, check the bits: 00000000000000000000000000000001
```
