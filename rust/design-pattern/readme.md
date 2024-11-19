# Rust Study를 위한 Project

* Design Pattern을 활용하여, Rust의 기본 기능에 대해서 숙지

<details>
<summary>데이터 타입</summary>

### 스칼라 타입
하나의 값만을 나타내는 타입
| 타입         | 표기                              |
|---------------|------------------------------------|
| 정수          |  `i8`, `i16`, `i32`, `i64`, `i128` |
| 부호 없는 정수 |  `u8`, `u16`, `u32`, `u64`, `u128` |
| 부동소수점     |  `f32`, `f64 (기본값)`             |
| 부호 없는 정수 |  `u8`, `u16`, `u32`, `u64`, `u128` |
| 문자           | `char`                            |
| 불리언         | `bool`                            |

### 구성 타입
여러 값을 묶어서 하나의 값으로 만드는 타입(튜플과 배열)
| 타입         | 표기                                                   |
|---------------|------------------------------------------------------|
 튜플           | let tup = `(500, 6.4, 'A');`<br>let `(x, y, z)` = tup;|
| 배열          | let arr = `[1, 2, 3, 4, 5];`                         |


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
``` rust
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

</details>

<details>
    <summary>팩토리 패턴 (Facttory Pattern)</summary>

| 키워드     | 기능                                                       |
|------------|------------------------------------------------------------|
| mod        | 모듈을 정의                                               |
| use        | 다른 모듈에서 함수, 구조체, 트레이트 등을 가져오는 데 사용 |
| pub mod    | 모듈을 공개하여 외부에서 접근할 수 있게 함 (기본은 private) |
| pub use    | 다른 모듈에서 가져온 항목을 재공개                        |
| trait      | Rust의 인터페이스, 특정 기능을 정의                        |
| use crate  | 현재 크레이트의 루트                                      |
| Box        | 힙에 데이터를 저장하는 스마트 포인터                      |
| *          | 역참조 연산자, 포인터나 참조 타입에 대해 역참조 수행     |
| &mut *     | 가변 참조로 접근한 값을 변경할 때 사용                    |
| dyn        | 동적 디스패치를 사용하기 위해 `dyn` 키워드 사용            |
| &mut dyn   | 트레이트 객체에 대한 mutable reference를 의미             |


## 디스패치란?
함수 호출을 어떻게 결정하고 실행할 것인지에 대한 내용

### 정적 디스패치
컴파일 시점에 어떤 함수가 호출될지 결정
``` rust
//본 디자인 패턴에서는 사용 할수 없음
fn main(){
    let static_mage = Mage::new();
    level_up_by_static_dispatch(static_mage);
}

fn level_up_by_static_dispatch<T:Character>( character:  T) {
    println!("Created a {}!=====================================", character.get_role());
}
```

### 동적 디스패치
런타임 시점에 어떤 함수가 호출될지 결정
* 동적 디스패치는 함수 호출 시 가상 테이블(vtable)을 사용하므로 약간의 성능 오버헤드

* 코드의 유연성을 높여 다형성(polymorphism)을 구현
```rust

fn level_up_by_dynamic_dispatch(character: &mut dyn Character) {
    character.level_up();
}
```
</details>

