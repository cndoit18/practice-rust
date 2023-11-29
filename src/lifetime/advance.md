```rust
/* 使用生命周期注释结构体
1. `r` 和 `s` 必须是不同生命周期
2. `s` 的生命周期需要大于 'r'
*/
struct DoubleRef<'r, 's: 'r, T> {
    r: &'r T,
    s: &'s T
}
fn main() {
    println!("Success!")
}
```

```rust
/* 添加类型约束使下面代码正常运行 */
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a:'b, 'b> ImportantExcerpt<'a> {
    fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    println!("Success!")
}
```

```rust
/* 添加类型约束使下面代码正常运行 */
fn f<'a, 'b>(x: &'a i32, mut y: &'b i32) 
    where 'a: 'b {
    y = x;                      
    let r: &'b &'a i32 = &&0;   
}

fn main() {
    println!("Success!")
}
```

```rust
/* 添加 HRTB 使下面代码正常运行！ */
fn call_on_ref_zero<F>(f: F) 
    where for<'a> F: Fn(&'a i32) {
    let zero = 0;
    f(&zero);
}

fn main() {
    println!("Success!")
}
```

```rust
/* 通过重新排序一些代码使下面代码正常运行 */
fn main() {
    let mut data = 10;
    let ref1 = &mut data;
    *ref1 += 1;
    
    let ref2 = &mut *ref1;

    *ref2 += 2;

    println!("{}", data);
}
```

```rust
/* 使下面代码正常运行 */
struct Interface<'a: 'b, 'b> {
    manager: &'b mut Manager<'a>
}

impl<'a, 'b> Interface<'a, 'b> {
    pub fn noop(self) {
        println!("interface consumed");
    }
}

struct Manager<'a> {
    text: &'a str
}

struct List<'a> {
    manager: Manager<'a>,
}

impl<'a> List<'a> {
    pub fn get_interface<'b>(&'b mut self) -> Interface<'a, 'b> {
        Interface {
            manager: &mut self.manager
        }
    }
}

fn main() {
    let mut list = List {
        manager: Manager {
            text: "hello"
        }
    };

    list.get_interface().noop();

    println!("Interface should be dropped here and the borrow released");

    use_list(&list);
}

fn use_list(list: &List) {
    println!("{}", list.manager.text);
}
```