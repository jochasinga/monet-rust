use std::{io::{BufRead, BufReader, Error, Read}, u8};

use crate::predicate::{
    is_delimiter, 
    is_underscore,
    is_op,
};

/// Read a single character from the reader.
fn read_char(reader: &mut BufReader<&[u8]>) -> Option<u8> {
    reader
        .bytes()
        .next()
        .and_then(|result| result.ok())
}

/// Read a single character from the reader without consuming it.
fn peek_char(reader: &mut BufReader<&[u8]>) -> Option<u8> {
    let buf = reader.fill_buf().unwrap();
    let c = buf.get(0)
        .map(|c| *c);
    reader.consume(0);
    c
}

fn inner_fixnum(reader: &mut BufReader<&[u8]>, mut acc: Vec<u8>) -> u32 {
    match peek_char(reader) {
        None => fixnum(reader, acc),
        Some(c) => {
            let c = c as char;
            if is_delimiter(c) || is_op(c) {
                return_fixnum(acc)
            } else if is_underscore(c) {
                let _ = read_char(reader);
                inner_fixnum(reader, acc)
            } else {
                let c = read_char(reader).unwrap();
                acc.push(c);
                inner_fixnum(reader, acc)
            }
        }

        //     let c = read_char(reader).unwrap();
        //     let mut acc = acc;
        //     acc.push(c);
        //     fixnum(reader, acc)
        // },
    }
}

fn return_fixnum(acc: Vec<u8>) -> u32 {
    let a = acc.into_iter()
        // .rev()
        .into_iter()
        .collect();
    String::from_utf8(a).unwrap().parse::<u32>().unwrap()
}

// fn is_eof(c: u32) -> bool {
//     c == u8::MAX as u32
// }


fn fixnum(reader: &mut BufReader<&[u8]>, acc: Vec<u8>) -> u32 {
    match peek_char(reader) {
        None => return_fixnum(acc),
        Some(_) => {
            let c = read_char(reader).unwrap();
            let mut acc = acc;
            acc.push(c);
            fixnum(reader, acc)
        },
    }
}

pub fn get_fixnum(_reader: BufReader<&[u8]>) -> i32 {
    42
}

#[cfg(test)]
mod tests {
    use std::io::{BufReader, Bytes, Error, Read};
    use super::*;
            
    #[test]
    fn test_inner_fixnum() {
        let mut reader = BufReader::new("42_000_000".as_bytes());
        let mut acc = Vec::new();
        assert_eq!(42_000_000, inner_fixnum(&mut reader, acc));

        reader = BufReader::new("42 ".as_bytes());
        acc = Vec::new();
        assert_eq!(42, inner_fixnum(&mut reader, acc));

        reader = BufReader::new("42+".as_bytes());
        acc = Vec::new();
        assert_eq!(42, inner_fixnum(&mut reader, acc));
    }

    #[test]
    fn test_read_char() {
        let mut reader = BufReader::new("42".as_bytes());
        assert_eq!(Some(b'4'), read_char(&mut reader));
        assert_eq!(Some(b'2'), read_char(&mut reader));
        assert_eq!(None, read_char(&mut reader));
    }

    #[test]
    fn test_peek_char() {
        let mut reader = BufReader::new("42a".as_bytes());
        assert_eq!(Some(b'4'), peek_char(&mut reader));
        assert_eq!(b'4', read_char(&mut reader).unwrap());
        assert_eq!(Some(b'2'), peek_char(&mut reader));
        assert_eq!(b'2', read_char(&mut reader).unwrap());
        assert_eq!(Some(b'a'), peek_char(&mut reader));
        assert_eq!(b'a', read_char(&mut reader).unwrap());
        assert_eq!(None, peek_char(&mut reader));
        assert_eq!(None, read_char(&mut reader));
    }

    #[test]
    fn test_return_fixnum() {
        let acc = vec![b'4', b'2'];
        assert_eq!(return_fixnum(acc), 42);
    }

    #[test]
    fn test_fixnum() {
        let mut reader = BufReader::new("42".as_bytes());
        let num = fixnum(&mut reader, Vec::new());
        assert_eq!(42, num);
        // assert_eq!('4', char::from_u32(c).unwrap());
    }
}