```rust
/* 使用两种方法填空 */
fn main() {
    let v: &str = "hello";
    need_static(v);

    println!("Success!")
}

fn need_static(r : &'static str) {
    assert_eq!(r, "hello");
}
```

```rust
#[derive(Debug)]
struct Config {
    a: String,
    b: String,
}
static mut config: Option<&mut Config> = None;

/* 让代码工作，但不要修改函数的签名 */
fn init() -> Option<&'static mut Config> {
    Some(Box::leak(Box::new(Config{
        a: "A".to_string(),
        b: "B".to_string()
    })))
}


fn main() {
    unsafe {
        config = init();

        println!("{:?}",config)
    }
}
```

```rust
fn main() {
    let static_string;
    {
        // 字符串字面量能跟程序活得一样久，因此 `static_string` 的生命周期是 `'static`
        static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);

        // 当 `static_string` 超出作用域时，该引用就无法再被使用，但是引用指向的数据( 字符串字面量 ) 依然保存在二进制 binary 所占用的内存中
    }

    println!("static_string reference remains alive: {}", static_string);
}
```

```rust
/* 让代码工作 */
use std::fmt::Debug;

fn print_it<T: Debug>( input: T) {
    println!( "'static value passed in is: {:?}", input );
}

fn print_it1( input: impl Debug ) {
    println!( "'static value passed in is: {:?}", input );
}


fn print_it2<T: Debug + 'static>( input: &T) {
    println!( "'static value passed in is: {:?}", input );
}

fn main() {
    // i 是有所有权的数据，并没有包含任何引用，因此它是 'static
    let i = 5;
    print_it(i);

    // 但是 &i 是一个引用，生命周期受限于作用域，因此它不是 'static
    print_it(&i);

    print_it1(&i);

    // 但是下面的代码可以正常运行 !
    print_it2(&i);
}
```

```rust
use std::fmt::Display;

fn main() {
  let mut string = "First".to_owned();

  string.push_str(string.to_uppercase().as_str());
  print_a(&string);
  print_b(&string);
  print_c(&string); // Compilation error
  print_d(&string); // Compilation error
  print_e(&string);
  print_f(&string);
  print_g(&string); // Compilation error
}

fn print_a<T: Display + 'static>(t: &T) {
  println!("{}", t);
}

fn print_b<T>(t: &T)
where
  T: Display + 'static,
{
  println!("{}", t);
}

fn print_c(t: &dyn Display) {
  println!("{}", t)
}

fn print_d(t: &impl Display) {
  println!("{}", t)
}

fn print_e(t: &(dyn Display + 'static)) {
  println!("{}", t)
}

fn print_f(t: &(impl Display + 'static)) {
  println!("{}", t)
}

fn print_g(t: &String) {
  println!("{}", t);
}
```
