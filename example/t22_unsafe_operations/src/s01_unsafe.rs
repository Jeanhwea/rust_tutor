// 在 Rust 中，不安全代码块用于避开编译器的保护策略，具体地说，不安全代码块主要
// 用于四件事情：
//   - 解引用裸指针
//   - 通过 FFI 调用函数（这已经在之前的章节介绍过了） Foreign Function Interface
//   - 调用不安全的函数
//   - 内联汇编（inline assembly）

use std::arch::asm;
use std::slice;

fn main() {
    let some_vector = vec![1, 2, 3, 4];

    let pointer = some_vector.as_ptr();
    let length = some_vector.len();

    // 调用不安全的函数
    unsafe {
        let my_slice: &[u32] = slice::from_raw_parts(pointer, length);

        assert_eq!(some_vector.as_slice(), my_slice);
    }

    // 解引用裸指针
    let raw_p: *const u32 = &10;
    unsafe {
        assert!(*raw_p == 10);
    }

    // 内联汇编
    let x: u64 = 3;
    let y: u64;
    unsafe {
        asm!("add {0}, 5", inout(reg) x => y);
    }
    assert_eq!(y, 8);
}
