extern crate foo;

fn main() {
    // 如果直接使用 try 无法导入，因为 try 时关键字
    // foo::try();

    // 可以尝试使用下面方法来导入
    foo::r#try();
}
