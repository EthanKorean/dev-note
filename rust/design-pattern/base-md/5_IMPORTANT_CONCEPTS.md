# Rust의 중요한 개념

## 목차
1. [소유권](#소유권-OwnerShip)
2. [대여와 참조](#대여와-참조---borrowing--references)
3. [라이프타임](#라이프타임---lifetime)
4. [패턴매칭](#패턴-매칭---pattern-matching)

---

### 소유권-OwnerShip
- Rust의 핵심 **개념**으로 메모리 관리를 자동으로 처리한다.
- Garbage Collector 없이 메모리를 안전하게 관리 할 수 있도록 보장
- 소유권의 규칙
    - 각 값은 소유자 하나만 가짐
```rust
     fn main(){
        let x = String::from("Hello"); // x는 String 값을 소유
        println!("{}", x);             // 사용 가능
     }
```
- 소유권을 벗어나면 메모리가 자동 해재
- 값을 다른 변수로 이동하면 원래 변수는 더이상 유효하지 않음
```rust
     fn main(){
        let s1 = String::from("hello");
        let s2 = s1; // 소유권 이동
        // println!("{}", s1); // s1은 더 이상 유효하지 않기 때문에 오류 발생, 
        println!("{}", s2);
     }
 ```
- 소유자는 변수를 통해 값을 관리하지만, 모든 변수가 소유하지 않기 떄문에, 변수를 소유자라 할 수 없음

---

## 대여와 참조 - Borrowing & References
- 소유권을 이동하지 않고 데이터를 읽거나 수정 할 수 있도록 한다.
- **불변 참조**( Immutable References ): 데이터를 읽기만 할 수 있다.
    - 여러 참조가능
- **가변 참조**( Mutable References ) : 데이터를 수정 할 수 있다.
    - 단, 한번에 하나의 가변 참조만 허용
    - 가변 참조 중 원본에 접근 불가
```rust
fn main() {
   let  x = String::from("Hello");
   let _y1 = &x;      // 불변 참조
   let _y2 = &x;      // 또 다른 불변 참조 (가능)
   //let _y3 = &mut x;   // 에러! 불변 참조와 가변 참조를 동시에 사용할 수 없음
   println!("_y1 = {}", _y1);
   println!("_y2 = {}", _y2);
}
```
```rust
fn main(){
  let mut a = String::from("Hello");
  let _z = &mut a;   // 가변 참조
  //println!("x = {}", x); //에러! 가변 참조 중 원본에 접근 불가
  println!("_z = {}", _z);
  println!("x = {}", a);
}
```

---

## 라이프타임 - Lifetime
- 참조가 유효한 범위를 명시적으로 표현하여 참조가 유효하지 않은 상태에서의 접근을 방지
- 참조가 유효한 기간을 추적하고, 컴파일러가 참조가 유효한 범위 내에서만 사용되도록 보장하는 방식
- 소유권(ownership) 시스템과 빌려주기(borrowing) 규칙을 사용하여 메모리 안전성을 확보하는데, 라이프타임은 이 두 가지 규칙을 강화하는 역할
- 규칙
    - 모든 참조는 유효한 시간 범위(lifetime)를 가지고 있다.
    - 한 참조가 다른 참조보다 오래 살아있으면 안 된다.
    - 라이프타임이 긴 참조를 짧은 라이프타임을 가진 참조에 할당할 수 없다.
- 라이프타임은 'a, 'b와 같이 작은따옴표와 함께 쓰는 이름으로 나타내며, 함수의 매개변수나 반환값에 적용할 수 있음
- ```rust
   struct Boo<'a>{
    title : &'a str,
   }
   ```
---


### 동시성 - Concurrency
- Rust는 **std::thread**와 **async/await**을 통해 안전한 동시성과 병렬성을 제공
```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("Hello from thread: {}", i);
        }
    });

    handle.join().unwrap(); // 쓰레드가 끝날 때까지 기다림
}
```
```rust
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let task1 = async {
        sleep(Duration::from_secs(1)).await;
        println!("Task 1 completed");
    };

    let task2 = async {
        println!("Task 2 completed");
    };

    tokio::join!(task1, task2);
}
```

---

### 매크로 - Macros
- 코드 생성 도구로, println!, vec! 등 다양한 매크로가 내장
```rust
macro_rules! say_hello {
    () => {
        println!("Hello, macro!");
    };
}

fn main() {
    say_hello!();
}
```




