fn answer() -> i32 {
  	42
}



#[test]
pub fn check_answer_validity() {
    assert_eq!(answer(), 42);
}