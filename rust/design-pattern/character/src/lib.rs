pub mod character; 
pub mod mage; 
pub mod role;
pub mod warrior; 


// 필요한 항목을 외부에서 사용할 수 있도록 re-export
pub use character::Character; // Character 트레이트
pub use mage::Mage;           // Mage 구조체
pub use warrior::Warrior;     // Warrior 구조체
pub use role::Role;           // Role 열거형