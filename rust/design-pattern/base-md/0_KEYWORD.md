# Rust 주요 키워드

## 목차
1. [let](#let)
2. [mut](#mut)
3. [fn](#fn)
4. [impl](#impl)
5. [trait](#trait)
7. [use](#use)
8. [mod](#mod)
9. [unsafe](#unsafe)
10. [self, Self](#self-self)
11. [const](#const)
12. [async / await](#async--await)
13. [dyn](#dyn)

### let
- 변수를 선언할 때 사용
- 기본적으로 불변(immutable) 변수를 선언
```rust
fn main(){
    let x = 5;
    // x = 10; 불가능
}
```

---

### mut
- 변수를 가변(mutable)으로 선언할 때 사용
- mut를 사용하여 값을 변경할 수 있는 변수를 선언
```rust
fn main(){
    let mut x = 5;
    x = 10; 
}
```

---

### fn
- 함수를 정의할 때 사용
```rust
fn greet() {
    println!("Hello, World!");
}
```

--- 

### enum
- 열거형을 정의할 때 사용
```rust
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
```
---

### impl
- 구조체나 열거형 등에 메서드를 구현할 때 사용
```rust
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn greet(&self) {
        println!("Hello, my name is {}", self.name);
    }
}
```
---

### trait
- 인터페이스, 특정 기능을 구현할 수 있는 규칙을 정의
```rust
trait Greet {
    fn greet(&self);
}

struct Person {
    name: String,
}

impl Greet for Person {
    fn greet(&self) {
        println!("Hello, {}", self.name);
    }
}
```

---

### use
- 다른 모듈이나 라이브러리의 기능을 사용할 때 사용
```rust
use std::io;
use std::collections::HashMap;
```

---

### mod 
- 모듈을 정의할 때 사용
```rust
mod my_module {
    pub fn hello() {
        println!("Hello from the module!");
    }
}
```

---

### unsafe
- Rust에서 안전하지 않은 코드를 작성할 때 사용
- 기본적으로 Rust는 메모리 안전성을 보장하지만, 
unsafe를 사용하여 직접 메모리를 조작하거나 외부 라이브러리와 상호작용할 수 있음
```rust
fn main(){
    unsafe {
        let r: *const i32 = &10;
        println!("{}", *r);
    }
}
```

---

### self, Self
- self: 메서드에서 현재 객체를 참조하는 데 사용
- Self: 타입 자체를 참조하는 데 사용
```rust
struct Person {
    name: String,
}

impl Person {
    fn new(name: String) -> Self {
        Self { name }
    }

    fn greet(&self) {
        println!("Hello, {}", self.name);
    }
}
```

---

### const
- 값을 변경할 수 없는 상수를 선언
```rust
static HELLO: &str = "Hello, World!";
```

---

### async / await
- 비동기 프로그래밍을 위해 사용
- async는 비동기 함수 정의에 사용
- await는 비동기 함수의 결과를 기다릴 때 사용
```rust
async fn fetch_data() -> String {
    "data".to_string()
}

async fn main() {
    let data = fetch_data().await;
    println!("{}", data);
}
```

---

### box
- 값을 힙에 할당하여 Box를 생성하는 데 사용
- Rust에서는 기본적으로 스택을 사용하나, Box를 사용하면 값이 힙에 할당
```rust
fn main(){
    let b = Box::new(5);
}
```

---

### dyn
- 동적 디스패치를 사용하는 타입을 나타낼 때 사용
- 주로 트레이트 객체를 사용하거나 동적으로 처리할 때 사용
```rust
fn main(){
    let x: Box<dyn ExampleExecutor> = Box::new(MyExampleExecutor);
}
```

--- 
#### 디스패치란?
함수 호출을 어떻게 결정하고 실행할 것인지에 대한 내용

---

##### 정적 디스패치
컴파일 시점에 어떤 함수가 호출될지 결정
```rust

fn main(){
    let static_mage = Mage::new();
    level_up_by_static_dispatch(static_mage);
}

fn level_up_by_static_dispatch<T:Character>( character:  T) {
    println!("Created a {}!=====================================", character.get_role());
}
```

--- 

##### 동적 디스패치
런타임 시점에 어떤 함수가 호출될지 결정
* 동적 디스패치는 함수 호출 시 가상 테이블(vtable)을 사용하므로 약간의 성능 오버헤드

* 코드의 유연성을 높여 다형성(polymorphism)을 구현
```rust

fn level_up_by_dynamic_dispatch(character: &mut dyn Character) {
    character.level_up();
}
```