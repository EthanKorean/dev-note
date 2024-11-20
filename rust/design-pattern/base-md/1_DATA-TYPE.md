# 데이터 타입
---
### 스칼라 타입
하나의 값만을 나타내는 타입

| 타입               | 표기                          |
|--------------|-----------------------------------|
| 정수          | `i8`, `i16`, `i32`, `i64`, `i128` |
| 부호 없는 정수 | `u8`, `u16`, `u32`, `u64`, `u128` |
| 부동소수점     | `f32`, `f64 (기본값)`              |
| 부호 없는 정수 | `u8`, `u16`, `u32`, `u64`, `u128` |
| 문자          | `char`                            |
| 불리언        | `bool`                            |

---

### 구성 타입
여러 값을 묶어서 하나의 값으로 만드는 타입(튜플과 배열)

| 타입         | 표기                                                   |
|---------------|----------------------------------------------------|
| 튜플           | let tup = `(500, 6.4, 'A');`<br>let `(x, y, z)` = tup;|
| 배열          | let arr = `[1, 2, 3, 4, 5];`                         |

---

### 기타 기본 타입

| 타입               | 표기                              |
|--------------------|---------------------------------|
| String(가변적 길이) |  let s: `String = String::from("Hello");`        |
| &str(불변 문자열)   |  let s: `&str = "Hello"; `      |
| Option             |  let some_number: `Option<i32> = Some(10)`<br>let no_number: `Option<i32> = None;`|
| Result  |  let success: `Result<i32, String> = Ok(200)`;<br> let error: `Result<i32, String> = Err("Error occurred".to_string())`;|

### 구조체와 열거형
| 타입    | 표기                                          |
|--------|-----------------------------------------------|
| struct | `struct Person { name: String, age: u32 }`    |
| enum   |  `enum Direction { Up, Down, Left, Right,} `  |

생성자나 함수를 포함하는 열거형
```rust
// 함수나 데이터를 포함하는 열거형
enum Shape {
    Circle(f64),     // 원: 반지름을 저장
    Rectangle(f64, f64), // 사각형: 너비와 높이를 저장
    Square(f64),     // 정사각형: 한 변의 길이를 저장
}

impl Shape {
    // 생성자 함수: 원을 생성
    fn new_circle(radius: f64) -> Self {
        Shape::Circle(radius)
    }

    // 생성자 함수: 사각형을 생성
    fn new_rectangle(width: f64, height: f64) -> Self {
        Shape::Rectangle(width, height)
    }

    // 생성자 함수: 정사각형을 생성
    fn new_square(side: f64) -> Self {
        Shape::Square(side)
    }

    // 넓이를 계산하는 메서드
    fn area(&self) -> f64 {
        match *self {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Rectangle(width, height) => width * height,
            Shape::Square(side) => side * side,
        }
    }
}
```
