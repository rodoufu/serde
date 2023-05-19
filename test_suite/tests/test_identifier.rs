//! Tests for `#[serde(field_identifier)]` and `#[serde(variant_identifier)]`

#![allow(clippy::derive_partial_eq_without_eq)]

use serde_derive::Deserialize;
use serde_test::{assert_de_tokens, assert_de_tokens_error, Token};

mod variant_identifier {
    use super::*;

    #[derive(Deserialize, Debug, PartialEq)]
    #[serde(variant_identifier)]
    enum V {
        Aaa,
        Bbb,
    }

    #[test]
    fn variant1() {
        assert_de_tokens(&V::Aaa, &[Token::U8(0)]);
        assert_de_tokens(&V::Aaa, &[Token::U16(0)]);
        assert_de_tokens(&V::Aaa, &[Token::U32(0)]);
        assert_de_tokens(&V::Aaa, &[Token::U64(0)]);
        assert_de_tokens(&V::Aaa, &[Token::Str("Aaa")]);
        assert_de_tokens(&V::Aaa, &[Token::Bytes(b"Aaa")]);
    }

    #[test]
    fn unknown() {
        assert_de_tokens_error::<V>(
            &[Token::U8(42)],
            "invalid value: integer `42`, expected variant index 0 <= i < 2",
        );
        assert_de_tokens_error::<V>(
            &[Token::U16(42)],
            "invalid value: integer `42`, expected variant index 0 <= i < 2",
        );
        assert_de_tokens_error::<V>(
            &[Token::U32(42)],
            "invalid value: integer `42`, expected variant index 0 <= i < 2",
        );
        assert_de_tokens_error::<V>(
            &[Token::U64(42)],
            "invalid value: integer `42`, expected variant index 0 <= i < 2",
        );
        assert_de_tokens_error::<V>(
            &[Token::Str("Unknown")],
            "unknown variant `Unknown`, expected `Aaa` or `Bbb`",
        );
        assert_de_tokens_error::<V>(
            &[Token::Bytes(b"Unknown")],
            "unknown variant `Unknown`, expected `Aaa` or `Bbb`",
        );
    }
}

mod field_identifier {
    use super::*;

    #[derive(Deserialize, Debug, PartialEq)]
    #[serde(field_identifier, rename_all = "snake_case")]
    enum F {
        Aaa,
        Bbb,
    }

    #[test]
    fn field1() {
        assert_de_tokens(&F::Aaa, &[Token::U8(0)]);
        assert_de_tokens(&F::Aaa, &[Token::U16(0)]);
        assert_de_tokens(&F::Aaa, &[Token::U32(0)]);
        assert_de_tokens(&F::Aaa, &[Token::U64(0)]);
        assert_de_tokens(&F::Aaa, &[Token::Str("aaa")]);
        assert_de_tokens(&F::Aaa, &[Token::Bytes(b"aaa")]);
    }

    #[test]
    fn unknown() {
        assert_de_tokens_error::<F>(
            &[Token::U8(42)],
            "invalid value: integer `42`, expected field index 0 <= i < 2",
        );
        assert_de_tokens_error::<F>(
            &[Token::U16(42)],
            "invalid value: integer `42`, expected field index 0 <= i < 2",
        );
        assert_de_tokens_error::<F>(
            &[Token::U32(42)],
            "invalid value: integer `42`, expected field index 0 <= i < 2",
        );
        assert_de_tokens_error::<F>(
            &[Token::U64(42)],
            "invalid value: integer `42`, expected field index 0 <= i < 2",
        );
        assert_de_tokens_error::<F>(
            &[Token::Str("unknown")],
            "unknown field `unknown`, expected `aaa` or `bbb`",
        );
        assert_de_tokens_error::<F>(
            &[Token::Bytes(b"unknown")],
            "unknown field `unknown`, expected `aaa` or `bbb`",
        );
    }

    #[test]
    fn unit_fallthrough() {
        #[derive(Deserialize, Debug, PartialEq)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum F {
            Aaa,
            Bbb,
            #[serde(other)]
            Other,
        }

        assert_de_tokens(&F::Other, &[Token::U8(42)]);
        assert_de_tokens(&F::Other, &[Token::U16(42)]);
        assert_de_tokens(&F::Other, &[Token::U32(42)]);
        assert_de_tokens(&F::Other, &[Token::U64(42)]);
        assert_de_tokens(&F::Other, &[Token::Str("x")]);
    }

    #[test]
    fn newtype_fallthrough() {
        #[derive(Deserialize, Debug, PartialEq)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum F {
            Aaa,
            Bbb,
            Other(String),
        }

        assert_de_tokens(&F::Other("x".to_owned()), &[Token::Str("x")]);
    }

    #[test]
    fn newtype_fallthrough_generic() {
        #[derive(Deserialize, Debug, PartialEq)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum F<T> {
            Aaa,
            Bbb,
            Other(T),
        }

        assert_de_tokens(&F::Other(42u8), &[Token::U8(42)]);
        assert_de_tokens(&F::Other(42u16), &[Token::U16(42)]);
        assert_de_tokens(&F::Other(42u32), &[Token::U32(42)]);
        assert_de_tokens(&F::Other(42u64), &[Token::U64(42)]);
        assert_de_tokens(&F::Other("x".to_owned()), &[Token::Str("x")]);
    }
}
