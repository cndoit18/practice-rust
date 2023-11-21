```rust
// 修复错误
struct Array<T, const N: usize> {
    data : [T; N]
}

fn main() {
    let arrays = [
        Array{
            data: [1, 2, 3],
        },
        Array{
            data: [1, 2, 3],
        },
        Array {
            data: [1, 2, 3]
        }
    ];
}
```

```rust
// 填空
fn print_array<T: std::fmt::Debug, const U: usize>(arr: [T; U]) {
    println!("{:?}", arr);
}
fn main() {
    let arr = [1, 2, 3];
    print_array(arr);

    let arr = ["hello", "world"];
    print_array(arr);
}
```

```rust
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
fn check_size<T>(val: T)
where
    Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue,
{
    //...
}

// 修复 main 函数中的错误
fn main() {
    check_size([0u8; 767]);
    check_size([0i32; 191]);
    check_size(["hello你好"; 47]); // size of &str ?
    check_size([(); 31].map(|_| "hello你好".to_string()));  // size of String?
    check_size(['中'; 111]); // size of char ?
}



pub enum Assert<const CHECK: bool> {}

pub trait IsTrue {}

impl IsTrue for Assert<true> {}
```
