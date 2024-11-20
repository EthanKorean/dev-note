# 크레이트 Crate

- Rust 프로그램의 컴파일 및 패키징 단위 
- Rust의 프로젝트는 크레이트로 구성되며,
  코드를 모듈화하고 재사용성을 높이는 데 중요한 역할
- 크레이트는 두 가지 주요 유형
  1. Binary Crate
  2. Library Crate
---
## Binary Crate
- 실행 가능한 프로그램을 만드는 크레이트
- main 함수가 필수적으로 포함


---
## Library Crate
- 라이브러리를 생성하며 다른 크레이트에서 재사용
- src/lib.rs 파일로 시작
- cargo build를 통해 컴파일된 후 다른 크레이트에서 의존성으로 사용
```rust
pub fn greet(name: &str) {
    println!("Hello, {}!", name);
}
```
---
