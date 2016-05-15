use std::cmp;

use Result;
// use connection::NatsCoreConn;
use connection::MessageProcessor;
use errors::Error;

// TODO this whole thing is terrible

#[derive(Debug)]
pub enum NatsOp {
    Start,
    Plus,
	PlusO,
	PlusOk,
	Minus,
	MinusE,
	MinusEr,
	MinusErr,
	MinusErrSpace,
	MinusErrArg,
	M,
	Ms,
	Msg,
	MsgSpace,
	MsgArg,
	MsgPayload,
	MsgEnd,
	P,
	Pi,
	Pin,
	Ping,
	Po,
	Pon,
	Pong,
}

#[derive(Debug)]
pub struct MessageArg {
    pub subject: Vec<u8>,
    pub reply: Option<Vec<u8>>,
    pub sid: u64,
    pub size: usize,
}

#[derive(Debug)]
pub struct Parser {
    state: NatsOp,
    after_space: usize,
    drop: usize,
    arg_buf: Vec<u8>,
    msg_arg: Option<MessageArg>,
    msg_buf: Vec<u8>,
}

// TODO check
fn u64_from_bytes(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0, |x, &i| x * 10 + (i - b'0') as u64)
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            state: NatsOp::Start,
            after_space: 0,
            drop: 0,
            // TODO check this out, what default size?
            arg_buf: Vec::with_capacity(1024),
            msg_arg: None,
            msg_buf: Vec::with_capacity(1024),
        }
    }

    pub fn parse_message_arg(&self, data: &[u8]) -> Result<MessageArg> {
        // TODO check performance
        // ' ', '\t', '\r', '\n':
        let vecs = data.split(|b| {
                let c = *b as char;
                c == ' ' || c == '\t' || c == '\r' || c == '\n'
            })
            .filter(|v| v.len() > 0)
            .collect::<Vec<&[u8]>>();

        match vecs.len() {
            3 => {
                let mut subject = Vec::<u8>::with_capacity(vecs[0].len());
                subject.extend_from_slice(vecs[0]);
                let msg_arg = MessageArg {
                    subject: subject,
                    reply: None,
                    sid: u64_from_bytes(vecs[1]),
                    size: u64_from_bytes(vecs[2]) as usize,
                };
                Ok(msg_arg)
            },
            4 => {
                let mut subject = Vec::<u8>::with_capacity(vecs[0].len());
                subject.extend_from_slice(vecs[0]);
                let mut reply = Vec::<u8>::with_capacity(vecs[2].len());
                reply.extend_from_slice(vecs[2]);
                let msg_arg = MessageArg {
                    subject: subject,
                    reply: Some(reply),
                    sid: u64_from_bytes(vecs[1]),
                    size: u64_from_bytes(vecs[3]) as usize,
                };
                Ok(msg_arg)
            },
            _ => {
                Err(Error::ParseError)
            },
        }
    }

    pub fn parse<P>(&mut self, processor: &mut P, data: &[u8]) -> Result<()> where P: MessageProcessor{
        self.after_space = 0;

        let mut idx = 0;
        while idx < data.len() {
            let b = data[idx] as char;
            match self.state {
                NatsOp::Start => {
                    match b {
                        'm' | 'M' => self.state = NatsOp::M,
                        'p' | 'P' => self.state = NatsOp::P,
                        '+' => self.state = NatsOp::Plus,
                        '-' => self.state = NatsOp::Minus,
                        _ => return Err(Error::ParseError),
                    };
                },
                NatsOp::Plus => {
                    match b {
                        'o' | 'O' => self.state = NatsOp::PlusO,
                        _ => return Err(Error::ParseError),
                    };
                },
                NatsOp::PlusO => {
                    match b {
                        'k' | 'K' => self.state = NatsOp::PlusOk,
                        _ => return Err(Error::ParseError),
                    };
                },
                NatsOp::PlusOk => {
                    match b {
                        '\n' => {
                            self.state = NatsOp::Start;
                            processor.process_ok();
                        },
                        _ => (),
                    };
                },

                NatsOp::Minus => {
                    match b {
                        'e' | 'E' => self.state = NatsOp::MinusE,
                        _ => return Err(Error::ParseError),
                    };
                },
                NatsOp::MinusE => {
                    match b {
                        'r' | 'R' => self.state = NatsOp::MinusEr,
                        _ => return Err(Error::ParseError),
                    };
                },
                NatsOp::MinusEr => {
                    match b {
                        'r' | 'R' => self.state = NatsOp::MinusErr,
                        _ => return Err(Error::ParseError),
                    };
                },
                NatsOp::MinusErr => {
                    match b {
                        ' ' | '\t' => self.state = NatsOp::MinusErrSpace,
                        _ => return Err(Error::ParseError),
                    };
                },
                NatsOp::MinusErrSpace => {
                    match b {
                        ' ' | '\t' => (),
                        _ => {
                            self.state = NatsOp::MinusErrArg;
                            self.after_space = idx;
                        },
                    };
                },
                NatsOp::MinusErrArg => {
                    match b {
                        '\r' => self.drop = 1,
                        '\n' => {
                            let arg = &data[self.after_space..idx-self.drop];
                            if self.arg_buf.is_empty() {
                                // Entire arg value is in this buffer
                                processor.process_err(arg);
                            } else {
                                // End of the arg value, but it was split over buffers
                                self.arg_buf.extend_from_slice(arg);
                                processor.process_err(&self.arg_buf);
                                self.arg_buf.clear();
                            }
                            self.state = NatsOp::Start;
                            self.drop = 0;
                        },
                        _ => (),
                    };
                },
                NatsOp::P => {
                    match b {
                        'i' | 'I' => self.state = NatsOp::Pi,
                        'o' | 'O' => self.state = NatsOp::Po,
                        _ => return Err(Error::ParseError),
                    };
                },
                NatsOp::Pi => {
                    match b {
                        'n' | 'N' => self.state = NatsOp::Pin,
                        _ => return Err(Error::ParseError),
                    };
                },
                NatsOp::Pin => {
                    match b {
                        'g' | 'G' => self.state = NatsOp::Ping,
                        _ => return Err(Error::ParseError),
                    };
                },
                NatsOp::Ping => {
                    match b {
                        '\n' => {
                            self.state = NatsOp::Start;
                            processor.process_ping();
                        },
                        _ => {},
                    };
                },
                NatsOp::Po => {
                    match b {
                        'n' | 'N' => self.state = NatsOp::Pon,
                        _ => return Err(Error::ParseError),
                    };
                },
                NatsOp::Pon => {
                    match b {
                        'g' | 'G' => self.state = NatsOp::Pong,
                        _ => return Err(Error::ParseError),
                    };
                },
                NatsOp::Pong => {
                    match b {
                        '\n' => {
                            self.state = NatsOp::Start;
                            processor.process_pong();
                        },
                        _ => {},
                    };
                },
                NatsOp::M => {
                    match b {
                        's' | 'S' => self.state = NatsOp::Ms,
                        _ => return Err(Error::ParseError),
                    };
                },
                NatsOp::Ms => {
                    match b {
                        'g' | 'G' => self.state = NatsOp::Msg,
                        _ => return Err(Error::ParseError),
                    };
                },
                NatsOp::Msg => {
                    match b {
                        ' ' | '\t' => self.state = NatsOp::MsgSpace,
                        _ => return Err(Error::ParseError),
                    };
                },
                NatsOp::MsgSpace => {
                    match b {
                        ' ' | '\t' => (),
                        _ =>  {
                            self.state = NatsOp::MsgArg;
                            self.after_space = idx;
                        },
                    };
                },
                NatsOp::MsgArg => {
                    match b {
                        '\r' => self.drop = 1,
                        '\n' => {
                            let arg = &data[self.after_space..idx-self.drop];
                            if self.arg_buf.is_empty() {
                                // TODO
                                self.msg_arg = Some(try!(self.parse_message_arg(arg)));
                            } else {
                                self.arg_buf.extend_from_slice(arg);
                                self.msg_arg = Some(try!(self.parse_message_arg(&self.arg_buf)));
                                self.arg_buf.clear();
                            }

                            self.state = NatsOp::MsgPayload;
                            self.drop = 0;
                            self.after_space = idx + 1;

                            // has to be Some at this point
                            idx = self.after_space + self.msg_arg.as_ref().unwrap().size - 1;
                        },
                        _ => (),
                    };
                },
                NatsOp::MsgPayload => {
                    if !self.msg_buf.is_empty() {
                        if self.msg_buf.len() >= self.msg_arg.as_ref().unwrap().size {
                            processor.process_message(self.msg_arg.as_ref().unwrap(), &self.msg_buf);
                            self.arg_buf.clear();
                            self.msg_buf.clear();
                            self.state = NatsOp::MsgEnd;
                        } else {
                            let to_copy = cmp::min(self.msg_arg.as_ref().unwrap().size - self.msg_buf.len(), data.len() - idx);
                            self.msg_buf.extend_from_slice(&data[idx..idx+to_copy]);
                            idx += to_copy - 1;
                        }
                    } else if idx - self.after_space >= self.msg_arg.as_ref().unwrap().size {
                        processor.process_message(self.msg_arg.as_ref().unwrap(), &data[self.after_space..idx]);
                        self.arg_buf.clear();
                        self.msg_buf.clear();
                        self.state = NatsOp::MsgEnd;
                    }
                },
                NatsOp::MsgEnd => {
                    match b {
                        '\n' => {
                            self.drop = 0;
                            self.after_space = idx + 1;
                            self.state = NatsOp::Start;
                        },
                        _ => (),
                    };
                },
            };

            idx += 1;
        }

        match self.state {
            NatsOp::MinusErrArg | NatsOp::MsgArg => self.arg_buf.extend_from_slice(&data[self.after_space..]),
            NatsOp::MsgPayload => {
                if self.msg_buf.is_empty() {
                    self.msg_buf.extend_from_slice(&data[self.after_space..]);
                }
            },
            _ => {},
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use connection::MessageProcessor;
    use std::str;
    use std::string::String;

    enum ParseEvent {
        NOk,
        NErr(String),
        Ping,
        Pong,
        Message(MessageArg, String),
    }

    struct MockMessageProcessor {
        events: Vec<ParseEvent>,
    }

    impl MockMessageProcessor {
        fn new() -> MockMessageProcessor {
            MockMessageProcessor {
                events: Vec::new(),
            }
        }
    }

    impl MessageProcessor for MockMessageProcessor {
        fn process_ok(&mut self) {
            self.events.push(ParseEvent::NOk);
        }

        fn process_err(&mut self, message: &[u8]) {
            let s = String::from(str::from_utf8(message).unwrap());
            self.events.push(ParseEvent::NErr(s));
        }

        fn process_ping(&mut self) {
            self.events.push(ParseEvent::Ping);
        }

        fn process_pong(&mut self) {
            self.events.push(ParseEvent::Pong);
        }

        fn process_message(&mut self, args: &MessageArg, message: &[u8]) {
            let s = String::from(str::from_utf8(message).unwrap());
            let a = MessageArg {
                subject: args.subject.to_vec(),
                reply: args.reply.as_ref().map(|v| v.to_vec()),
                sid: args.sid,
                size: args.size,
            };

            self.events.push(ParseEvent::Message(a, s));
        }
    }

    #[test]
    fn test_parse_ok_basic() {
        let mut parser = Parser::new();
        let mut processor = MockMessageProcessor::new();
        parser.parse(&mut processor, b"+OK\r\n").unwrap();
        assert_eq!(1, processor.events.len());
        match processor.events[0] {
            ParseEvent::NOk => {},
            _ => assert!(false),
        };
    }

    #[test]
    fn test_parse_err_basic() {
        let mut parser = Parser::new();
        let mut processor = MockMessageProcessor::new();
        parser.parse(&mut processor, b"-ERR some error message\r\n").unwrap();
        assert_eq!(1, processor.events.len());
        match processor.events[0] {
            ParseEvent::NErr(ref s) => assert_eq!("some error message", s),
            _ => assert!(false),
        };
    }

    #[test]
    fn test_parse_ping_basic() {
        let mut parser = Parser::new();
        let mut processor = MockMessageProcessor::new();
        parser.parse(&mut processor, b"PING\r\n").unwrap();
        assert_eq!(1, processor.events.len());
        match processor.events[0] {
            ParseEvent::Ping => {},
            _ => assert!(false),
        };
    }

    #[test]
    fn test_parse_pong_basic() {
        let mut parser = Parser::new();
        let mut processor = MockMessageProcessor::new();
        parser.parse(&mut processor, b"PONG\r\n").unwrap();
        assert_eq!(1, processor.events.len());
        match processor.events[0] {
            ParseEvent::Pong => {},
            _ => assert!(false),
        };
    }

    #[test]
    fn test_parse_message_no_reply() {
        let mut parser = Parser::new();
        let mut processor = MockMessageProcessor::new();
        parser.parse(&mut processor, b"MSG topic1 47 16\r\na sample message\r\n").unwrap();
        assert_eq!(1, processor.events.len());
        match processor.events[0] {
            ParseEvent::Message(ref a, ref s) => {
                assert_eq!(b"topic1", a.subject.as_slice());
                assert_eq!(None, a.reply);
                assert_eq!(47, a.sid);
                assert_eq!(16, a.size);
                assert_eq!("a sample message", s);
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn test_parse_message_reply() {
        let mut parser = Parser::new();
        let mut processor = MockMessageProcessor::new();
        parser.parse(&mut processor, b"MSG topic1 47 reply_topic 16\r\na sample message\r\n").unwrap();
        assert_eq!(1, processor.events.len());
        match processor.events[0] {
            ParseEvent::Message(ref a, ref s) => {
                assert_eq!(b"topic1", a.subject.as_slice());
                assert_eq!(b"reply_topic", a.reply.as_ref().unwrap().as_slice());
                assert_eq!(47, a.sid);
                assert_eq!(16, a.size);
                assert_eq!("a sample message", s);
            },
            _ => assert!(false),
        };
    }

    #[test]
    fn test_parse_multiple_operations() {
        let mut parser = Parser::new();
        let mut processor = MockMessageProcessor::new();
        let bytes = b"+OK\r\n-ERR some error message\r\nPING\r\nPONG\r\nMSG topic1 47 reply_topic 16\r\na sample message\r\n+OK\r\n";
        parser.parse(&mut processor, bytes).unwrap();
        assert_eq!(6, processor.events.len());
        match processor.events[0] {
            ParseEvent::NOk => {},
            _ => assert!(false),
        };
        match processor.events[1] {
            ParseEvent::NErr(ref s) => assert_eq!("some error message", s),
            _ => assert!(false),
        };
        match processor.events[2] {
            ParseEvent::Ping => {},
            _ => assert!(false),
        };
        match processor.events[3] {
            ParseEvent::Pong => {},
            _ => assert!(false),
        };
        match processor.events[4] {
            ParseEvent::Message(ref a, ref s) => {
                assert_eq!(b"topic1", a.subject.as_slice());
                assert_eq!(b"reply_topic", a.reply.as_ref().unwrap().as_slice());
                assert_eq!(47, a.sid);
                assert_eq!(16, a.size);
                assert_eq!("a sample message", s);
            },
            _ => assert!(false),
        };
        match processor.events[5] {
            ParseEvent::NOk => {},
            _ => assert!(false),
        };
    }

    #[test]
    fn test_parse_error_message_split_buffer() {
        let mut parser = Parser::new();
        let mut processor = MockMessageProcessor::new();
        parser.parse(&mut processor, b"-ER").unwrap();
        parser.parse(&mut processor, b"R some error mes").unwrap();
        parser.parse(&mut processor, b"sage\r\n+OK\r\n").unwrap();
        assert_eq!(2, processor.events.len());
        match processor.events[0] {
            ParseEvent::NErr(ref s) => assert_eq!("some error message", s),
            _ => assert!(false),
        };
        match processor.events[1] {
            ParseEvent::NOk => {},
            _ => assert!(false),
        };
    }

    #[test]
    fn test_parse_message_arg_split_buffer() {
        let mut parser = Parser::new();
        let mut processor = MockMessageProcessor::new();
        parser.parse(&mut processor, b"MS").unwrap();
        parser.parse(&mut processor, b"G topic1 4").unwrap();
        parser.parse(&mut processor, b"7 reply_topic 16\r\na sample message\r\n+OK\r\n").unwrap();

        assert_eq!(2, processor.events.len());
        match processor.events[0] {
            ParseEvent::Message(ref a, ref s) => {
                assert_eq!(b"topic1", a.subject.as_slice());
                assert_eq!(b"reply_topic", a.reply.as_ref().unwrap().as_slice());
                assert_eq!(47, a.sid);
                assert_eq!(16, a.size);
                assert_eq!("a sample message", s);
            },
            _ => assert!(false),
        };
        match processor.events[1] {
            ParseEvent::NOk => {},
            _ => assert!(false),
        };
    }

    #[test]
    fn test_parse_message_msg_split_buffer() {
        let mut parser = Parser::new();
        let mut processor = MockMessageProcessor::new();
        parser.parse(&mut processor, b"MSG topic1 47 reply_topic 16\r\na ").unwrap();
        parser.parse(&mut processor, b"samp").unwrap();
        parser.parse(&mut processor, b"le message\r\n+OK\r\n").unwrap();

        assert_eq!(2, processor.events.len());
        match processor.events[0] {
            ParseEvent::Message(ref a, ref s) => {
                assert_eq!(b"topic1", a.subject.as_slice());
                assert_eq!(b"reply_topic", a.reply.as_ref().unwrap().as_slice());
                assert_eq!(47, a.sid);
                assert_eq!(16, a.size);
                assert_eq!("a sample message", s);
            },
            _ => assert!(false),
        };
        match processor.events[1] {
            ParseEvent::NOk => {},
            _ => assert!(false),
        };
    }

    #[test]
    fn test_parse_message_arg_msg_split_buffer() {
        let mut parser = Parser::new();
        let mut processor = MockMessageProcessor::new();
        parser.parse(&mut processor, b"MS").unwrap();
        parser.parse(&mut processor, b"G topic1 4").unwrap();
        parser.parse(&mut processor, b"7 reply_topic 1").unwrap();
        parser.parse(&mut processor, b"6\r\na sampl").unwrap();
        parser.parse(&mut processor, b"e mess").unwrap();
        parser.parse(&mut processor, b"age\r\n").unwrap();

        assert_eq!(1, processor.events.len());
        match processor.events[0] {
            ParseEvent::Message(ref a, ref s) => {
                assert_eq!(b"topic1", a.subject.as_slice());
                assert_eq!(b"reply_topic", a.reply.as_ref().unwrap().as_slice());
                assert_eq!(47, a.sid);
                assert_eq!(16, a.size);
                assert_eq!("a sample message", s);
            },
            _ => assert!(false),
        };
    }
}
