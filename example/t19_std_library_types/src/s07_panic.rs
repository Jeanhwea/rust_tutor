// Re-implementation of integer division (/)
fn division(dividend: i32, divisor: i32) -> i32 {
    if divisor == 0 {
        // Division by zero triggers a panic
        panic!("division by zero");
    } else {
        dividend / divisor
    }
}

// The `main` task
fn main() {
    // Heap allocated integer
    let _x = Box::new(0i32);

    // This operation will trigger a task failure
    division(3, 0);

    println!("This point won't be reached!");

    // `_x` should get destroyed at this point
}

// panic 导致进程回退时 rust 会自动释放所有内存
// WSL(t19_std_library_types/src)% valgrind ./debug.run
// ==2415== Memcheck, a memory error detector
// ==2415== Copyright (C) 2002-2017, and GNU GPL'd, by Julian Seward et al.
// ==2415== Using Valgrind-3.18.1 and LibVEX; rerun with -h for copyright info
// ==2415== Command: ./debug.run
// ==2415==
// 4 / 2 = 2
// 1 / 0 failed!
// Some(0.0) unwraps to 0.0
// thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', s05_option.rs:41:49
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
// ==2415==
// ==2415== HEAP SUMMARY:
// ==2415==     in use at exit: 0 bytes in 0 blocks
// ==2415==   total heap usage: 13 allocs, 13 frees, 3,253 bytes allocated
// ==2415==
// ==2415== All heap blocks were freed -- no leaks are possible
// ==2415==
// ==2415== For lists of detected and suppressed errors, rerun with: -s
// ==2415== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 0 from 0)
