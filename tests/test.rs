#![no_std]
use bool_like::*;

#[bool_like]
#[derive(Debug, PartialEq)]
enum Player {
    Black,
    White,
}

#[test]
fn test_not_only() {
    assert_eq!(!Player::Black, Player::White);
    assert_eq!(!Player::White, Player::Black);
}

#[bool_like]
#[derive(Debug, PartialEq)]
enum Answer {
    Yes,
    #[into_false]
    No,
}

#[test]
fn test_into_from_bool() {
    assert_eq!(!Answer::Yes, Answer::No);
    assert_eq!(!Answer::No, Answer::Yes);

    assert!(!bool::from(Answer::No));
    assert!(bool::from(Answer::Yes));

    assert_eq!(Answer::from(false), Answer::No);
    assert_eq!(Answer::from(true), Answer::Yes);
}
