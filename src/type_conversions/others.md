```rust
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The point is ({}, {})", self.x, self.y)
    }
}

fn main() {
    let origin = Point { x: 0, y: 0 };
    // 填空
    assert_eq!(origin.to_string(), "The point is (0, 0)");
    assert_eq!(format!("{}", origin), "The point is (0, 0)");

    println!("Success!")
}
```

```rust
// 为了使用 `from_str` 方法, 你需要引入该特征到当前作用域中
use std::str::FromStr;
fn main() {
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = i32::from_str("10").unwrap();
    let from_str = i32::from_str("20").unwrap();
    let sum = parsed + turbo_parsed + from_str;
    assert_eq!(sum, 35);

    println!("Success!")
}
```

```rust
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s
            .trim_matches(|p| p == '(' || p == ')')
            .split(',')
            .collect();

        let x_fromstr = coords[0].parse::<i32>()?;
        let y_fromstr = coords[1].parse::<i32>()?;

        Ok(Point {
            x: x_fromstr,
            y: y_fromstr,
        })
    }
}
fn main() {
    // 使用两种方式填空
    // 不要修改其它地方的代码
    let p = Point::from_str("(3,4)");
    assert_eq!(p.unwrap(), Point { x: 3, y: 4 });

    println!("Success!")
}
```
