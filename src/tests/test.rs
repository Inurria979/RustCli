use find_matches::find_matches;
fn answer() -> i32 {
  	42
}

#[test]
pub fn check_answer_validity() {
    assert_eq!(answer(), 42);
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}
