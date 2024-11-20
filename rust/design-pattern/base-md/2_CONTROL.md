# 제어문

| 제어문 종류                      | 키워드           |
|--------------------------------|-----------------|
| 조건문(Conditional statements)  | if, match   |
| 반복문(Loop statements)         | loop, while, for |

---

## 조건문

### if , else, if-else 문
- if 뒤에 조건문을 괄호로 감싸지 않는다.

```rust
fn main(){
    let number = 5;

    if number > 10 {
        println!("The number is greater than 10.");
    } else if number > 5 {
        println!("The number is greater than 5 but less than or equal to 10.");
    } else {
        println!("The number is 5 or less.");
    }    
}
```

---

### 조건문을 표현식으로 사용하기(삼항연산자)
```rust
fn main(){
    let number = 3;

    let result = if number % 2 == 0 {
        "Even"
    } else {
        "Odd"
    };
    println!("The number is {}.", result);
}
```

---

### if let 문
- if let은 값이 특정 패턴에 맞는지 검사하고 맞으면 해당 블록을 실행
```rust
fn main(){
    let some_option = Some(5);

    if let Some(value) = some_option {
        println!("The value is {}", value);
    } else {
        println!("No value found.");
    }
}
```

--- 

### match
- 스위치문
```rust
fn main(){
    let number = 3;

    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Other"),
    }
}
```

---
## 반복문

### loop
- 조건 없이 무한히 반복되는 반복문
- 특정 조건에서 종료하려면 break를 사용
```rust
fn main(){
    let mut count = 0;

    loop {
        count += 1;
        if count == 5 {
            break; // count가 5가 되면 루프 종료
        }
        println!("Count: {}", count);
    }  
}
```

---

### while
```rust
fn main() {
    while number != 0 {
        println!("Number: {}", number);
        number -= 1; // number가 0이 되면 반복 종료
    }
}
```

---

###  while let
- while let은 특정 패턴이 일치하는 동안 루프를 실행
```rust
fn main() {
    let mut some_option = Some(5);

    while let Some(value) = some_option {
        println!("Value: {}", value);
        some_option = None; // 반복 종료
    }
}
```

---

### for
```rust
fn main(){
    for i in 1..5 { // 1..5는 1 이상 5 미만의 범위
        println!("i: {}", i);
    }

    for i in 1..=5 { // 1 이상 5 이하의 범위
        println!("i: {}", i);
    }
    
    //배열을 사용한 반복
    let numbers = [10, 20, 30, 40, 50];

    for num in numbers {
        println!("Number: {}", num);
    }

    //벡터를 사용한 반복
    let fruits = vec!["Apple", "Banana", "Cherry"];

    for (index, fruit) in fruits.iter().enumerate() {
        println!("{}: {}", index, fruit);
    }

    //라벨을 사용하여 중첩 루프 제어
    'outer: for i in 1..=3 { 
        for j in 1..=3 {
            if i == 2 && j == 2 {
                break 'outer; // 바깥쪽 루프 종료
            }
            println!("i: {}, j: {}", i, j);
        }
    }

    //continue
    for i in 1..10 {
        if i % 2 == 0 {
            continue; // 짝수는 건너뛰기
        }
        println!("Odd number: {}", i);
    }
    
}
```