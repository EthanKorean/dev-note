
use common::example::Example;
use design::sample::{init_character_by_factory_design, init_character_by_factory_method_design};

fn main() {

    let _ownership = ownership();
    _ownership.execute();

    let _borrowing_references = borrowing_references();
    _borrowing_references.execute();
    
    //Factory 디자인 패턴
    let _factory_design = init_character_by_factory_design();
    _factory_design.execute();

    let _factory_method_design = init_character_by_factory_method_design();
    _factory_method_design.execute();

}

fn ownership() -> Example {
   Example::new( "Owner ship",
       Box::new(|| {
           let x = String::from("Hello"); // x는 String 값을 소유
           println!("x = {}", x);             // 사용 가능

           let s1 = String::from("hello");
           let s2 = s1; // 소유권 이동
           // println!("{}", s1); // s1은 더 이상 유효하지 않기 때문에 오류 발생,
           println!("s2 = {}", s2);
        }),
    )
}

fn borrowing_references() -> Example {
    Example::new( "Borrowing & References",
        Box::new(|| {
            let  x = String::from("Immutable");
            let _y1 = &x;      // 불변 참조
            let _y2 = &x;      // 또 다른 불변 참조 (가능)
            println!("_y1 = {}", _y1);
            println!("_y2 = {}", _y2);
            //let _y3 = &mut x;   // 에러! 불변 참조와 가변 참조를 동시에 사용할 수 없음

            //let _c = &mut a;   // 에러 ! 단, 한번에 하나의 가변 참조만 허용
            //println!("a = {}", a); //에러! 가변 참조 중 원본에 접근 불가
            let mut a = String::from("Mutable");
            let _b = &mut a;   // 가변 참조

            println!("_b = {}", _b);
        }),
    )
}