use crate::configuration::Configuration;
use std::{error::Error, fmt, iter::Peekable, str::Chars};

use std::fmt::Write as FmtWrite;

type Result<T = ()> = std::result::Result<T, ParseError>;

pub fn parse(config: &Configuration, chars: &mut Chars, stage: &mut String) -> Result {
    Parser::new(config, chars).parse_to_stage(stage)
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ParseError {
    InvalidSyntaxClose(String),
    NotFindSyntaxClose(String),
    FmtError(fmt::Error),
}
impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidSyntaxClose(s) => write!(f, "invalid the close of syntax: {}", s)?,
            Self::NotFindSyntaxClose(s) => write!(f, "not find the close of syntax: {}", s)?,
            Self::FmtError(e) => write!(f, "{}", e)?,
        }
        Ok(())
    }
}

impl From<fmt::Error> for ParseError {
    fn from(e: fmt::Error) -> Self {
        Self::FmtError(e)
    }
}

struct Parser<'a, 'b> {
    config: &'a Configuration,
    chars: Peekable<&'a mut Chars<'b>>,
}
impl<'a, 'b> Parser<'a, 'b> {
    fn new(config: &'a Configuration, chars: &'a mut Chars<'b>) -> Self {
        Self {
            config,
            chars: chars.peekable(),
        }
    }

    /// parse chars to stage
    fn parse_to_stage(&mut self, stage: &mut String) -> Result {
        let mut buff = Buffer::new();

        loop {
            match self.chars.next() {
                None => {
                    buff.merge_span_to_line();
                    stage.push_str(&buff.line);
                    return Ok(());
                }
                Some('#') => {
                    buff.merge_span_to_line();
                    self.parse_comment(&mut buff.span)?;
                    buff.move_line_to_stage(stage);
                }
                Some('\n') => {
                    buff.move_line_to_stage(stage);
                    stage.push('\n');
                }
                Some(' ') => match buff.span.as_str() {
                    "if" => {
                        self.parse_if_statement(&mut buff.span, 0)?;
                        buff.move_line_to_stage(stage);
                    }
                    "foreach" => {
                        self.parse_for_statement(&mut buff.span, 0)?;
                        buff.move_line_to_stage(stage);
                    }
                    "elif" | "else" | "endif" | "endforeach" => return Err(ParseError::InvalidSyntaxClose(buff.span)),
                    _ => buff.merge_span_to_line(),
                },
                Some(c) => self.match_char_parse(&mut buff, c, 0)?,
            }
        }
    }

    fn match_char_parse(&mut self, buff: &mut Buffer, c: char, indent: u8) -> Result {
        match c {
            '\'' => {
                buff.merge_span_to_line();
                self.parse_string(&mut buff.span)?;
                buff.merge_span_to_line();
                buff.last_identifier = true;
            }
            '(' => {
                buff.merge_span_to_line();
                buff.move_line_to_span();
                self.parse_argument(&mut buff.span, indent)?;
                buff.last_identifier = true;
            }
            '[' => {
                buff.merge_span_to_line();
                // "[]" may be index of array
                if buff.last_identifier && !buff.line.ends_with(" in") {
                    buff.move_line_to_span();
                }
                self.parse_array(&mut buff.span, indent)?;
                buff.last_identifier = true;
            }
            '{' => {
                buff.merge_span_to_line();
                self.parse_dictionary(&mut buff.span, indent)?;
                buff.last_identifier = true;
            }
            ')' | ']' | '}' => return Err(ParseError::InvalidSyntaxClose(c.into())),
            '+' | '-' => {
                buff.last_identifier = (buff.span.is_empty() || !buff.last_identifier) && buff.begin_statement;
                buff.begin_statement = false;
                buff.merge_span_to_line();
                buff.span.push(c);
            }
            '*' | '/' | '%' => {
                buff.last_identifier = false;
                buff.begin_statement = false;
                buff.merge_span_to_line();
                buff.span.push(c);
            }
            c if buff.last_identifier != crate::grammar::is_identifier(c) => {
                buff.last_identifier = crate::grammar::is_identifier(c);
                if !buff.last_identifier {
                    buff.begin_statement = true;
                }
                buff.merge_span_to_line();
                buff.span.push(c);
            }
            c => {
                buff.begin_statement = false;
                buff.span.push(c);
            }
        }
        Ok(())
    }
}

impl<'a, 'b> Parser<'a, 'b> {
    /// parse comment line
    fn parse_comment(&mut self, stage: &mut String) -> Result {
        let mut buff = String::from("# ");
        loop {
            match self.chars.next() {
                None => return Ok(write!(stage, "#")?),
                Some('\n') => return Ok(writeln!(stage, "#")?),
                Some(' ') => {}
                Some(c) => {
                    buff.push(c);
                    break;
                }
            }
        }
        loop {
            match self.chars.next() {
                None => return Ok(write!(stage, "{}", buff.trim_end())?),
                Some('\n') => return Ok(writeln!(stage, "{}", buff.trim_end())?),
                Some(c) => buff.push(c),
            }
        }
    }

    fn parse_string(&mut self, stage: &mut String) -> Result {
        // single line strings
        let parse_alone_string = |chars: &mut Peekable<&mut Chars>, stage: &mut String, mut last: char| loop {
            write!(stage, "'{}", last)?;
            loop {
                match chars.next().ok_or_else(|| ParseError::NotFindSyntaxClose("'".into()))? {
                    '\'' if last != '\\' => {
                        stage.push('\'');
                        return Ok(());
                    }
                    c => last = c,
                }
                stage.push(last);
            }
        };

        // multi line strings
        let parse_multi_string = |chars: &mut Peekable<&mut Chars>, stage: &mut String| {
            stage.push_str("'''");
            let mut last = '\'';
            let mut quote_count = 0;
            loop {
                match chars
                    .next()
                    .ok_or_else(|| ParseError::NotFindSyntaxClose("'''".into()))?
                {
                    '\'' if last != '\\' => {
                        if quote_count == 2 {
                            return Ok(write!(stage, "'''")?);
                        }
                        last = '\'';
                        quote_count += 1;
                    }
                    c => {
                        stage.push(c);
                        last = c;
                        quote_count = 0;
                    }
                }
            }
        };

        match self.chars.next() {
            None | Some('\n') => Err(ParseError::NotFindSyntaxClose("'".into())),
            Some('\'') => match self.chars.peek() {
                Some('\'') => {
                    self.chars.next();
                    // begin with three `'`
                    parse_multi_string(&mut self.chars, stage)
                }
                _ => Ok(write!(stage, "''")?),
            },
            Some(c) => parse_alone_string(&mut self.chars, stage, c),
        }
    }
}

impl<'a, 'b> Parser<'a, 'b> {
    fn parse_array(&mut self, stage: &mut String, indent: u8) -> Result {
        self.parse_list(stage, indent, '[', ']')
    }

    fn parse_dictionary(&mut self, stage: &mut String, indent: u8) -> Result {
        self.parse_list(stage, indent, '{', '}')
    }

    fn parse_argument(&mut self, stage: &mut String, indent: u8) -> Result {
        self.parse_list(stage, indent, '(', ')')
    }

    fn parse_list(&mut self, stage: &mut String, indent_outer: u8, char_begin: char, char_close: char) -> Result {
        let mut items = vec![];

        let mut buff = Buffer::new();

        let mut multiline = false;
        let mut newline_comment = false;

        let mut indent_inner = indent_outer;

        loop {
            match self
                .chars
                .next()
                .ok_or_else(|| ParseError::NotFindSyntaxClose(char_begin.into()))?
            {
                c if c == char_close => {
                    buff.merge_span_to_line();
                    if !buff.line.is_empty() {
                        buff.line.push(',');
                        items.push(buff.line);
                    }
                    if items.is_empty() {
                        return Ok(write!(stage, "{}{}", char_begin, char_close)?);
                    }
                    for item in items.iter_mut() {
                        if item.ends_with('\n') {
                            item.pop();
                        }
                    }
                    let (head, body, foot) = if multiline {
                        let indent_outer_str = std::iter::repeat(' ').take(indent_outer.into()).collect::<String>();
                        let indent_inner_str = std::iter::repeat(' ').take(indent_inner.into()).collect::<String>();
                        (
                            format!("\n{}", indent_inner_str),
                            items.join(&(String::from('\n') + &indent_inner_str)),
                            format!("\n{}", indent_outer_str),
                        )
                    } else {
                        if let Some(last) = items.last_mut() {
                            if last.ends_with(',') {
                                last.pop();
                            }
                        }
                        let extra = if self.config.space_inner_bracket { " " } else { "" };
                        (extra.into(), items.join(" "), extra.into())
                    };
                    return Ok(write!(stage, "{}{}{}{}{}", char_begin, head, body, foot, char_close)?);
                }
                '#' => {
                    buff.merge_span_to_line();
                    if !buff.line.is_empty() {
                        buff.line.push(',');
                        items.push(buff.line);
                        buff.line = String::new();
                    } else if newline_comment {
                        items.push(buff.line);
                        buff.line = String::new();
                    }
                    if let Some(buff) = items.last_mut() {
                        if !buff.is_empty() {
                            buff.push(' ');
                        }
                        self.parse_comment(buff)?;
                    }
                    newline_comment = true;
                }
                ':' => {
                    buff.merge_span_to_line();
                    if self.config.space_before_colon {
                        buff.line.push_str(" :");
                    } else {
                        buff.line.push(':');
                    }
                    buff.last_identifier = false;
                }
                ',' => {
                    buff.merge_span_to_line();
                    buff.move_line_to_lists(&mut items);
                    newline_comment = false;
                    buff.last_identifier = false;
                }
                '\n' => {
                    buff.merge_span_to_line();
                    multiline = true;
                    newline_comment = true;
                    indent_inner = indent_outer + self.config.indent_width;
                }
                ' ' => buff.merge_span_to_line(),
                c => self.match_char_parse(&mut buff, c, indent_inner)?,
            }
        }
    }
}

impl<'a, 'b> Parser<'a, 'b> {
    fn parse_if_statement(&mut self, stage: &mut String, indent_outer: u8) -> Result {
        let indent_inner = indent_outer + self.config.indent_width;

        let indent_outer_str = std::iter::repeat(' ').take(indent_outer.into()).collect::<String>();
        let indent_inner_str = std::iter::repeat(' ').take(indent_inner.into()).collect::<String>();

        let mut buff = Buffer::new();

        let mut first_line = true;

        stage.push(' ');

        let push_indent_from_line = |stage: &mut String, line: &str| {
            let line = line.trim();
            if line == "else" || line.starts_with("elif ") {
                stage.push_str(&indent_outer_str);
            } else {
                stage.push_str(&indent_inner_str);
            }
        };

        loop {
            match self.chars.next() {
                None if buff.span.as_str() == "endif" => return Ok(write!(stage, "{}endif", indent_outer_str)?),
                None => return Err(ParseError::NotFindSyntaxClose("if".into())),
                Some('#') => {
                    buff.merge_span_to_line();
                    self.parse_comment(&mut buff.span)?;
                    buff.move_line_to_stage_with_indent(stage, &mut first_line, push_indent_from_line);
                }
                Some('\n') => {
                    match buff.span.as_ref() {
                        "foreach" => self.parse_for_statement(&mut buff.span, indent_inner)?,
                        "if" => self.parse_if_statement(&mut buff.span, indent_inner)?,
                        "endforeach" => return Err(ParseError::InvalidSyntaxClose(buff.span)),
                        "endif" => return Ok(writeln!(stage, "{}endif", indent_outer_str)?),
                        _ => {}
                    }
                    buff.move_line_to_stage_with_indent(stage, &mut first_line, push_indent_from_line);
                    stage.push('\n');
                }
                Some(' ') => match buff.span.as_ref() {
                    "foreach" => {
                        self.parse_for_statement(&mut buff.span, indent_inner)?;
                        buff.move_line_to_stage_with_indent(stage, &mut first_line, push_indent_from_line);
                    }
                    "if" => {
                        self.parse_if_statement(&mut buff.span, indent_inner)?;
                        buff.move_line_to_stage_with_indent(stage, &mut first_line, push_indent_from_line);
                    }
                    "endforeach" => return Err(ParseError::InvalidSyntaxClose(buff.span)),
                    "endif" => return Ok(write!(stage, "{}endif", indent_outer_str)?),
                    _ => buff.merge_span_to_line(),
                },
                Some(c) => self.match_char_parse(&mut buff, c, indent_inner)?,
            }
        }
    }
    fn parse_for_statement(&mut self, stage: &mut String, indent_outer: u8) -> Result {
        let indent_inner = indent_outer + self.config.indent_width;

        let indent_outer_str = std::iter::repeat(' ').take(indent_outer.into()).collect::<String>();
        let indent_inner_str = std::iter::repeat(' ').take(indent_inner.into()).collect::<String>();

        let mut buff = Buffer::new();

        let mut first_line = true;

        stage.push(' ');

        let push_indent_from_line = |stage: &mut String, _: &str| stage.push_str(&indent_inner_str);

        loop {
            match self.chars.next() {
                None if buff.span.as_str() == "endforeach" => {
                    return Ok(write!(stage, "{}endforeach", indent_outer_str)?)
                }
                None => return Err(ParseError::NotFindSyntaxClose("foreach".into())),
                Some('#') => {
                    buff.merge_span_to_line();
                    self.parse_comment(&mut buff.span)?;
                    buff.move_line_to_stage_with_indent(stage, &mut first_line, push_indent_from_line);
                }
                Some('\n') => {
                    match buff.span.as_ref() {
                        "foreach" => self.parse_for_statement(&mut buff.span, indent_inner)?,
                        "if" => self.parse_if_statement(&mut buff.span, indent_inner)?,
                        "endforeach" => return Ok(writeln!(stage, "{}endforeach", indent_outer_str)?),
                        "endif" => return Err(ParseError::InvalidSyntaxClose(buff.span)),
                        _ => {}
                    }
                    buff.move_line_to_stage_with_indent(stage, &mut first_line, push_indent_from_line);
                    stage.push('\n');
                }
                Some(' ') => match buff.span.as_ref() {
                    "foreach" => {
                        self.parse_for_statement(&mut buff.span, indent_inner)?;
                        buff.move_line_to_stage_with_indent(stage, &mut first_line, push_indent_from_line);
                    }
                    "if" => {
                        self.parse_if_statement(&mut buff.span, indent_inner)?;
                        buff.move_line_to_stage_with_indent(stage, &mut first_line, push_indent_from_line);
                    }
                    "endforeach" => return Ok(writeln!(stage, "{}endforeach", indent_outer_str)?),
                    "endif" => return Err(ParseError::InvalidSyntaxClose(buff.span)),
                    _ => buff.merge_span_to_line(),
                },
                Some(c) => self.match_char_parse(&mut buff, c, indent_inner)?,
            }
        }
    }
}

struct Buffer {
    /// a sentence
    line: String,
    /// a word or symbol
    span: String,
    /// last char is identifier, literal
    last_identifier: bool,
    /// start new statement, ("+" | "-") unary_operator identity
    begin_statement: bool,
}
impl Buffer {
    fn new() -> Self {
        Self {
            line: String::new(),
            span: String::new(),
            last_identifier: false,
            begin_statement: true,
        }
    }
    fn merge_span_to_line(&mut self) {
        if !self.span.is_empty() {
            if !self.line.is_empty() {
                self.line.push(' ');
            }
            self.line.push_str(&self.span);
            self.span.clear();
        }
    }
    fn move_line_to_span(&mut self) {
        std::mem::swap(&mut self.line, &mut self.span);
        self.line.clear();
    }
    fn move_line_to_lists(&mut self, lists: &mut Vec<String>) {
        if !self.line.is_empty() {
            self.line.push(',');
            let mut line = String::new();
            std::mem::swap(&mut line, &mut self.line);
            lists.push(line);
        }
    }
    fn move_line_to_stage(&mut self, stage: &mut String) {
        self.merge_span_to_line();
        stage.push_str(&self.line.trim_end_matches(' '));
        self.line.clear();
    }
    fn move_line_to_stage_with_indent<F>(&mut self, stage: &mut String, first_line: &mut bool, push_indent_from_line: F)
    where
        F: Fn(&mut String, &str),
    {
        self.merge_span_to_line();
        if *first_line {
            *first_line = false;
        } else {
            push_indent_from_line(stage, &self.line);
        }
        stage.push_str(&self.line.trim_end_matches(' '));
        self.line.clear();
    }
}
