
use common::example::Example;

fn main() {

    let ownership = ownership();
    ownership.execute();

    let borrowing_references = borrowing_references();
    borrowing_references.execute();

    //Factory 디자인 패턴
   // init_character_by_fatory_design();
}

fn ownership() -> Example {
   Example::new(
       "Owner ship",
       Box::new(|| {
           let x = String::from("Hello"); // x는 String 값을 소유합니다.
           println!("x = {}", x);             // 사용 가능

           let s1 = String::from("hello");
           let s2 = s1; // 소유권 이동
           // println!("{}", s1); // s1은 더 이상 유효하지 않기 때문에 오류 발생,
           println!("s1 = {}", s2);
        }),
    )
}

fn borrowing_references() -> Example {
    Example::new(
        "Borrowing & References",
        Box::new(|| {
            let  x = String::from("Hello");
            let _y1 = &x;      // 불변 참조
            let _y2 = &x;      // 또 다른 불변 참조 (가능)
            //let _y3 = &mut x;   // 에러! 불변 참조와 가변 참조를 동시에 사용할 수 없음
            println!("_y1 = {}", _y1);
            println!("_y2 = {}", _y2);

            let mut a = String::from("Hello");
            let _b = &mut a;   // 가변 참조
            //let _c = &mut a;   // 에러 ! 단, 한번에 하나의 가변 참조만 허용
            //println!("x = {}", x); //에러! 가변 참조 중 원본에 접근 불가
            println!("_b = {}", _b);
            println!("a = {}", a);

            let _c = &mut a;   // 에러 ! 단, 한번에 하나의 가변 참조만 허용
        }),
    )
}