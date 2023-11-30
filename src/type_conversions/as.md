```rust
// 修复错误，填空
// 不要移除任何代码
fn main() {
    let decimal = 97.123_f32;

    let integer: u8 = decimal as u8;

    let c1: char = decimal as u8 as char;
    let c2 = integer as char;

    assert_eq!(integer, 'b' as u8);

    println!("Success!")
}
```

```rust
#[allow(overflowing_literals)]
fn main() {
    assert_eq!(u8::MAX, 255);
    // 如上所示，u8 类型允许的最大值是 255.
    // 因此以下代码会报溢出的错误： literal out of range for `u8`.
    // **请仔细查看相应的编译错误，从中寻找到解决的办法**
    // **不要修改 main 中的任何代码**
    let v = 1000 as u8;

    println!("Success!")
}
```

```rust
#[allow(overflowing_literals)]
fn main() {
    assert_eq!(1000 as u16, 1000 % u16::MAX);

    assert_eq!(1000 as u8, 1000 % u8::MAX);

    // 事实上，之前说的规则对于正整数而言，就是如下的取模
    println!("1000 mod 256 is : {}", 1000 % 256);

    assert_eq!(-1_i8 as u8, u8::MAX);
    

    // 从 Rust 1.45 开始，当浮点数超出目标整数的范围时，转化会直接取正整数取值范围的最大或最小值
    assert_eq!(300.1_f32 as u8, u8::MAX);
    assert_eq!(-100.1_f32 as u8, u8::MIN);
    

    // 上面的浮点数转换有一点性能损耗，如果大家对于某段代码有极致的性能要求，
    // 可以考虑下面的方法，但是这些方法的结果可能会溢出并且返回一些无意义的值
    // 总之，请小心使用
    unsafe {
        // 300.0 is 44
        println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
        // -100.0 as u8 is 156
        println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
        // nan as u8 is 0
        println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
    }
}
```

```rust
// 填空
fn main() {
    let mut values: [i32; 2] = [1, 2];
    let p1: *mut i32 = values.as_mut_ptr();
    let first_address: usize = p1 as usize; 
    let second_address = first_address + 4; // 4 == std::mem::size_of::<i32>()
    let p2: *mut i32 = second_address as *mut i32; // p2 指向 values 数组中的第二个元素
    unsafe {
        // 将第二个元素加 1
        *p2 += 1;
    }
    
    assert_eq!(values[1], 3);

    println!("Success!")
}
```

```rust
fn main() {
    let arr :[u64; 13] = [0; 13];
    assert_eq!(std::mem::size_of_val(&arr), 8 * 13);
    let a: *const [u64] = &arr;
    let b: *const [u8] = a as *const [u8];
    unsafe {
        assert_eq!(std::mem::size_of_val(&*b), arr.len());
    }
}
```