use bytedata_pattern_core::*;

const VERB_HELLO: Test = Test::verbatim("hello");
const VERB_ALPHANUM: Test = Test::char_class(CharacterClass::Alphanumeric);
const VERB_REPEAT: Test = Test::repeat_exact(&VERB_ALPHANUM, 5);
const VERB_JOIN: Test = Test::join(&[VERB_HELLO, Test::char_class(CharacterClass::Whitespace), VERB_REPEAT]);
const VERB_JOIN_EXACT: Test = Test::join(&[Test::context_class(ContextClass::Start), VERB_HELLO, Test::char_class(CharacterClass::Whitespace), VERB_REPEAT, Test::context_class(ContextClass::End)]);

#[test]
fn simple_test_verb() {
    assert!(VERB_HELLO.test("hello".as_bytes()).unwrap());
    assert!(VERB_HELLO.test("hello to you".as_bytes()).unwrap());
    assert!(VERB_HELLO.test("well, hello".as_bytes()).unwrap());
    assert!(VERB_HELLO.test("well, hello to you".as_bytes()).unwrap());
    assert!(!VERB_HELLO.test("world".as_bytes()).unwrap());
    assert!(!VERB_HELLO.test("'ello world".as_bytes()).unwrap());
    assert!(!VERB_HELLO.test("hell".as_bytes()).unwrap());
    assert!(!VERB_HELLO.test("ello".as_bytes()).unwrap());
    assert!(!VERB_HELLO.test("ell".as_bytes()).unwrap());
}

#[test]
fn simple_test_charclass() {
    assert!(VERB_ALPHANUM.test("h".as_bytes()).unwrap());
    assert!(VERB_ALPHANUM.test("H".as_bytes()).unwrap());
    assert!(VERB_ALPHANUM.test("0".as_bytes()).unwrap());
    assert!(VERB_ALPHANUM.test("9".as_bytes()).unwrap());
    assert!(!VERB_ALPHANUM.test(" ".as_bytes()).unwrap());
    assert!(!VERB_ALPHANUM.test(".".as_bytes()).unwrap());
}

#[test]
fn simple_test_repeat() {
    // `VERB_REPEAT` is a repeat of 5 alphanumeric characters.
    assert!(VERB_REPEAT.test("hello".as_bytes()).unwrap());
    assert!(VERB_REPEAT.test("w0rld".as_bytes()).unwrap());
    assert!(VERB_REPEAT.test("hello world".as_bytes()).unwrap());
    assert!(VERB_REPEAT.test("hello ".as_bytes()).unwrap());
    assert!(VERB_REPEAT.test(" world".as_bytes()).unwrap());
    assert!(VERB_REPEAT.test(" world ".as_bytes()).unwrap());
    assert!(VERB_REPEAT.test(" worldsss ".as_bytes()).unwrap());
    assert!(!VERB_REPEAT.test("hell".as_bytes()).unwrap());
    assert!(!VERB_REPEAT.test("ello".as_bytes()).unwrap());
    assert!(!VERB_REPEAT.test(" ello ".as_bytes()).unwrap());
}

#[test]
fn simple_test_join() {
    // `VERB_JOIN` is a join of 'hello', whitespace, 5 alphanumeric characters, and the end of the input. `^hello \w{5}/`
    VERB_JOIN.assert("hello world".as_bytes(), "expected to match 'hello world'");
    VERB_JOIN.assert("hello\tworld".as_bytes(), "expected to match 'hello\\tworld'");
    VERB_JOIN.assert("hello\nworld".as_bytes(), "expected to match 'hello\\nworld'");
    VERB_JOIN.assert("hello\nw0rld".as_bytes(), "expected to match 'hello\\nw0rld'");
    VERB_JOIN.assert("hello world ".as_bytes(), "expected to match on trailing whitespace");
    VERB_JOIN.assert(" hello world".as_bytes(), "expected to match on leading whitespace");
    VERB_JOIN.assert(" hello world ".as_bytes(), "expected to match on wrapped whitespace");
    VERB_JOIN.assert("hello worldssss".as_bytes(), "expected to match on trailing characters");
    VERB_JOIN.assert_fail("hello_world".as_bytes(), "expected to fail on missing whitespace");
    VERB_JOIN.assert_fail("o world".as_bytes(), "expected to fail on missing 'hello'");
    VERB_JOIN.assert_fail("hello w".as_bytes(), "expected to fail on missing 'world'");
}

#[test]
fn simple_test_join_exact() {
    // `VERB_JOIN_EXACT` is a join of start, 'hello', whitespace, 5 alphanumeric characters, and the end of the input. `/^hello \w{5}$/`
    VERB_JOIN_EXACT.assert("hello world".as_bytes(), "expected to match 'hello world'");
    VERB_JOIN_EXACT.assert("hello\tworld".as_bytes(), "expected to match 'hello\\tworld'");
    VERB_JOIN_EXACT.assert("hello\nworld".as_bytes(), "expected to match 'hello\\nworld'");
    VERB_JOIN_EXACT.assert("hello\nw0rld".as_bytes(), "expected to match 'hello\\nw0rld'");
    VERB_JOIN_EXACT.assert_fail("hello world ".as_bytes(), "expected to fail on trailing whitespace");
    VERB_JOIN_EXACT.assert_fail(" hello world".as_bytes(), "expected to fail on leading whitespace");
    VERB_JOIN_EXACT.assert_fail(" hello world ".as_bytes(), "expected to fail on wrapped whitespace");
    VERB_JOIN_EXACT.assert_fail("hello worldssss".as_bytes(), "expected to fail on trailing characters");
    VERB_JOIN_EXACT.assert_fail("hello_world".as_bytes(), "expected to fail on missing whitespace");
    VERB_JOIN_EXACT.assert_fail("o world".as_bytes(), "expected to fail on missing 'hello'");
    VERB_JOIN_EXACT.assert_fail("hello w".as_bytes(), "expected to fail on missing 'world'");
}
