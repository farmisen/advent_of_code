extern crate nom;

use std::cmp::Ordering;

use nom::{
    branch::alt,
    bytes::complete::take_while,
    character::complete::{char, one_of},
    combinator::{cut, map, recognize},
    error::{ErrorKind, ParseError},
    multi::{many1, separated_list0},
    sequence::{preceded, terminated},
    IResult, ParseTo,
};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Value {
    Int(u32),
    List(Vec<Value>),
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a.cmp(b),
            (Value::List(a), Value::List(b)) => a.cmp(b),
            (Value::Int(a), Value::List(_)) => Value::List(vec![Value::Int(*a)]).cmp(other),
            (Value::List(_), Value::Int(b)) => self.cmp(&Value::List(vec![Value::Int(*b)])),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn sp<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    let chars = " \t\r\n";
    take_while(move |c| chars.contains(c))(input)
}

fn int<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, u32, E> {
    let (i, s) = recognize(many1(one_of("0123456789")))(input)?;
    match s.parse_to() {
        Some(int) => Ok((i, int)),
        None => Err(nom::Err::Error(E::from_error_kind(i, ErrorKind::Fail))), // TODO use specific error kind
    }
}

fn value<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Value, E> {
    preceded(sp, alt((map(int, Value::Int), map(list, Value::List))))(input)
}

fn list<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Vec<Value>, E> {
    preceded(
        char('['),
        cut(terminated(
            separated_list0(preceded(sp, char(',')), value),
            preceded(sp, char(']')),
        )),
    )(input)
}

pub fn parse_list(input: &str) -> Result<Vec<Value>, &'static str> {
    match list::<(&str, ErrorKind)>(input) {
        Ok((_, list)) => Ok(list),
        _ => Err("oops"),
    }
}
#[cfg(test)]
mod tests {
    use nom::error::ErrorKind;

    use crate::nom13::{int, list, sp, value, Value};
    #[test]
    fn parse_spaces() {
        assert_eq!(
            sp::<(&str, ErrorKind)>("   \n\t  \r"),
            Ok(("", "   \n\t  \r"))
        );
    }

    #[test]
    fn parse_int_numbers() {
        assert_eq!(int::<(&str, ErrorKind)>("1234"), Ok(("", 1234)));
    }

    #[test]
    fn parse_int_values() {
        assert_eq!(
            value::<(&str, ErrorKind)>("1234"),
            Ok(("", Value::Int(1234)))
        );
    }

    #[test]
    fn parse_list() {
        assert_eq!(
            list::<(&str, ErrorKind)>("[1, 2, 3]"),
            Ok(("", vec!(Value::Int(1), Value::Int(2), Value::Int(3))))
        );
    }

    #[test]
    fn parse_nested_list() {
        assert_eq!(
            list::<(&str, ErrorKind)>("[1, 2, 3, [4 , 5]]"),
            Ok((
                "",
                vec!(
                    Value::Int(1),
                    Value::Int(2),
                    Value::Int(3),
                    Value::List(vec!(Value::Int(4), Value::Int(5)))
                )
            ))
        );
    }
}
