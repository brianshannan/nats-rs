use std::str;

use Result;
use errors::Error;

#[derive(Debug, PartialEq, Eq)]
pub struct MessageArg<'a> {
    pub subject: &'a str,
    pub sid: u64,
    pub reply: Option<&'a str>,
    pub size: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseResult<'a> {
    NoOp,
    Okay,
    Error(&'a str),
    Ping,
    Pong,
    Message(MessageArg<'a>),
}

/// Parses a line of input from the server
pub fn parse_line<'a>(data: &'a str) -> Result<ParseResult<'a>> {
    let data = data.trim_right();
    if data.len() == 0 {
        return Ok(ParseResult::NoOp);
    }
    let upper = data.to_uppercase();
    if upper == "+OK" {
        return Ok(ParseResult::Okay);
    } else if upper == "PING" {
        return Ok(ParseResult::Ping);
    } else if upper == "PONG" {
        return Ok(ParseResult::Pong);
    } else if upper.starts_with("-ERR ") {
        return Ok(ParseResult::Error(&data[5..]));
    } else if upper.starts_with("MSG ") {
        return Ok(ParseResult::Message(try!(parse_message_arg(&data[4..]))));
    }

    Err(Error::ParseError)
}

fn parse_message_arg<'a>(data: &'a str) -> Result<MessageArg<'a>> {
    let vecs = data.split_whitespace()
        .filter(|v| v.len() > 0)
        .collect::<Vec<&str>>();

    match vecs.len() {
        3 => {
            Ok(MessageArg {
                subject: vecs[0],
                sid: try!(u64::from_str_radix(vecs[1], 10)),
                reply: None,
                size: try!(usize::from_str_radix(vecs[2], 10)),
            })
        },
        4 => {
            Ok(MessageArg {
                subject: vecs[0],
                sid: try!(u64::from_str_radix(vecs[1], 10)),
                reply: Some(vecs[2]),
                size: try!(usize::from_str_radix(vecs[3], 10)),
            })
        },
        _ => Err(Error::ParseError),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_ok() {
        let result = parse_line("+OK\r\n");
        assert_eq!(ParseResult::Okay, result.unwrap());
    }

    #[test]
    fn test_parse_ping() {
        let result = parse_line("PING\r\n");
        assert_eq!(ParseResult::Ping, result.unwrap());
    }

    #[test]
    fn test_parse_pong() {
        let result = parse_line("PONG\r\n");
        assert_eq!(ParseResult::Pong, result.unwrap());
    }

    #[test]
    fn test_parse_err() {
        let message = "some error message I'm getting";
        let formatted = format!("-ERR {}\r\n", message);
        let result = parse_line(&formatted);
        assert_eq!(ParseResult::Error(message), result.unwrap());
    }

    #[test]
    fn test_parse_msg_no_reply() {
        let subject = "topic1";
        let message = format!("MSG {} 47 18\r\n", subject);
        let result = parse_line(&message);
        let expected = MessageArg {
            subject: &subject,
            sid: 47,
            reply: None,
            size: 18,
        };
        assert_eq!(ParseResult::Message(expected), result.unwrap());
    }

    #[test]
    fn test_parse_msg_reply() {
        let subject = "topic1";
        let reply = "reply1";
        let message = format!("MSG {} 47 {} 18\r\n", subject, reply);
        let result = parse_line(&message);
        let expected = MessageArg {
            subject: &subject,
            sid: 47,
            reply: Some(&reply),
            size: 18,
        };
        assert_eq!(ParseResult::Message(expected), result.unwrap());
    }

    #[test]
    fn test_parse_invalid_op() {
        assert!(parse_line("BLAH\r\n").is_err());
    }

    #[test]
    fn test_parse_lowercase() {
        let result = parse_line("ping\r\n");
        assert_eq!(ParseResult::Ping, result.unwrap());
    }

    #[test]
    fn test_parse_msg_extra_spaces() {
        let subject = "topic1";
        let message = format!("MSG   {}   47    18\r\n", subject);
        let result = parse_line(&message);
        let expected = MessageArg {
            subject: &subject,
            sid: 47,
            reply: None,
            size: 18,
        };
        assert_eq!(ParseResult::Message(expected), result.unwrap());
    }

    #[test]
    fn test_parse_msg_invalid_args() {
        let subject = "topic1";
        let message = format!("MSG   {}       18\r\n", subject);
        assert!(parse_line(&message).is_err());
    }
}
