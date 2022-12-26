use std::{collections::VecDeque, num::IntErrorKind, path::PathBuf};

use regex::Regex;

use crate::report_error;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Token {
    pub line_number: u32,
    pub column_start: u32,
    pub column_end: u32,
    pub value: String,
    pub token_type: TokenType,
}

#[rustfmt::skip]
#[derive(Debug, PartialEq)]
pub enum TokenType {
    Label(String),       // Any valid identifier followed by ':' and whitespace to end of line
    Directive(String),   // '.' followed by a valid identifier
    Instruction(String), // Any valid identifier at the beginning of a line
    Comma,               // ','
    Register(String),    // '%' followed by any identifier
    Immediate,           // '#'
    Decimal(String),     // any decimal value without a prefix
    Binary(String),      // '%' followed by a binary value
    Hex(String),         // '$' followed by a hex value
    AsciiString(String), // Any valid ascii string enclosed by '"' including valid escape characters
    Identifier(String),  // Any alphanumeric value on its own
    OpenBracket,         // '['
    CloseBracket,        // ']'
    OpenParenthesis,     // '('
    CloseParenthesis,    // ')'
}

impl Token {
    pub fn parse_u16(&self, path: &PathBuf, lines: &Vec<String>) -> u16 {
        match &self.token_type {
            TokenType::Binary(value) => {
                // Parse from string value
                match u16::from_str_radix(value, 2) {
                    Ok(v) => v,
                    Err(err) => match err.kind() {
                        // Greater than a 16 bit word
                        IntErrorKind::PosOverflow => report_error(
                            "Binary literal is larger than expected 16-bit word! (Max is %1111111111111111)",
                            path,
                            lines,
                            self.line_number,
                            self.column_start,
                            self.column_end,
                        ),
                        kind => panic!("Unexpected IntErrorKind: {kind:?}"),
                    },
                }
            }
            TokenType::Decimal(value) => {
                // Parse from string value
                match u16::from_str_radix(value, 10) {
                    Ok(v) => v,
                    Err(err) => match err.kind() {
                        // Greater than a 16 bit word
                        IntErrorKind::PosOverflow => report_error(
                            "Decimal literal is larger than expected 16-bit word! (Max is 65535)",
                            path,
                            lines,
                            self.line_number,
                            self.column_start,
                            self.column_end,
                        ),
                        kind => panic!("Unexpected IntErrorKind: {kind:?}"),
                    },
                }
            }
            TokenType::Hex(value) => {
                // Parse from string value
                match u16::from_str_radix(value, 16) {
                    Ok(v) => v,
                    Err(err) => match err.kind() {
                        // Greater than a 16 bit word
                        IntErrorKind::PosOverflow => report_error(
                            "Hexadecimal literal is larger than expected 16-bit word! (Max is $FFFF)",
                            path,
                            lines,
                            self.line_number,
                            self.column_start,
                            self.column_end,
                        ),
                        kind => panic!("Unexpected IntErrorKind: {kind:?}"),
                    },
                }
            }
            _ => panic!("Cannot parse u16 from non number type!"),
        }
    }
}

pub fn tokenize_lines(path: &PathBuf, lines: &Vec<String>) -> VecDeque<Token> {
    let mut tokens: VecDeque<Token> = VecDeque::new();

    for (line_number, line) in lines.iter().enumerate() {
        let line_number = line_number as u32;

        let mut chars: VecDeque<_> = line.chars().collect();

        let mut col_number: u32 = 0;
        let mut found_instruction = false;
        let mut found_directive = false;

        // Check if line is empty
        if chars.is_empty() {
            continue;
        }

        // Loop through characters in the line building tokens
        while !chars.is_empty() {
            let token_col_start = col_number;

            let first_char = chars.pop_front().unwrap();
            col_number += 1;

            match (
                first_char,
                first_char.is_alphabetic() || first_char == '_',
                first_char.is_numeric(),
            ) {
                // Keep going until we find something more interesting
                (' ', _, _) => continue,
                // If we found a comment, there are no more tokens so just jump to the next line
                (';', _, _) => break,
                // Directive
                ('.', _, _) => {
                    let identifier =  read_to_chars(vec![' ', ']', ')', '[', '(', ','], &mut col_number, &mut chars);

                    let Some(value) = identifier else {
                        report_error(
                            "Unexpected end of directive token",
                            path,
                            lines,
                            line_number,
                            token_col_start,
                            col_number,
                        );
                    };

                    if !value.is_alphanumeric() {
                        report_error(
                            "Directive names must be alphanumeric!",
                            path,
                            lines,
                            line_number,
                            token_col_start,
                            col_number,
                        );
                    }

                    let full_value = format!("{first_char}{value}");

                    tokens.push_back(Token {
                        line_number: line_number as u32,
                        column_start: token_col_start,
                        column_end: col_number,
                        value: full_value,
                        token_type: TokenType::Directive(value),
                    });

                    found_directive = true;
                }
                // First character is alphanumeric
                // Could be a label, an instruction, or an identifier
                (_, true, _) => {
                    let proceeding =
                        read_to_chars(vec![' ', ']', ')', '[', '(', ','], &mut col_number, &mut chars);

                    let value = match proceeding {
                        Some(val) => val,
                        None => "".to_owned(),
                    };

                    let full_value = format!("{first_char}{value}");

                    // Found a label
                    if full_value.ends_with(":") {
                        // Check if name without the ':' is valid
                        if !(&full_value[..full_value.len() - 1]).is_alphanumeric() {
                            report_error(
                                "Label name must be alphanumeric!",
                                path,
                                lines,
                                line_number,
                                token_col_start,
                                col_number,
                            );
                        }

                        let label_name = (&full_value[..full_value.len() - 1]).to_owned();

                        tokens.push_back(Token {
                            line_number: line_number as u32,
                            column_start: token_col_start,
                            column_end: col_number,
                            value: full_value,
                            token_type: TokenType::Label(label_name),
                        });

                        continue;
                    }

                    // If we found a naked identifier on a line where we have not yet
                    // found an instruction or directive, this must be an instruction
                    if !found_instruction && !found_directive {
                        // Found an instruction
                        found_instruction = true;

                        if !full_value.is_alphanumeric() {
                            report_error(
                                "Instruction name must be alphanumeric!",
                                path,
                                lines,
                                line_number,
                                token_col_start,
                                col_number,
                            );
                        }

                        tokens.push_back(Token {
                            line_number: line_number as u32,
                            column_start: token_col_start,
                            column_end: col_number,
                            value: full_value.clone(),
                            token_type: TokenType::Instruction(full_value),
                        });
                    }
                    // If we already found an instruction on this line,
                    // it must be another identifier
                    else {
                        if !full_value.is_alphanumeric() {
                            report_error(
                                "Identifier name must be alphanumeric!",
                                path,
                                lines,
                                line_number,
                                token_col_start,
                                col_number,
                            );
                        }

                        tokens.push_back(Token {
                            line_number: line_number as u32,
                            column_start: token_col_start,
                            column_end: col_number,
                            value: full_value.clone(),
                            token_type: TokenType::Identifier(full_value),
                        });
                    }
                }
                // Ascii String Literal
                ('"', _, _) => {
                    let proceeding = read_to_char_inclusive('"', &mut col_number, &mut chars);

                    let Some(value) = proceeding else {
                        report_error(
                            "Expected closing '\"' for string literal",
                            path,
                            lines,
                            line_number,
                            token_col_start,
                            col_number,
                        );
                    };

                    let full_value = format!("{first_char}{value}");

                    let string_contents = (&full_value[1..full_value.len() - 1]).to_owned();

                    tokens.push_back(Token {
                        line_number: line_number as u32,
                        column_start: token_col_start,
                        column_end: col_number,
                        value: full_value,
                        token_type: TokenType::AsciiString(string_contents),
                    });
                }
                // Register name or binary value
                ('%', _, _) => {
                    let value = read_to_chars(vec![' ', ',', ';', '(', ')', '[', ']'], &mut col_number, &mut chars);

                    let Some(value) = value else {
                        report_error(
                            "Unexpected end of token",
                            path,
                            lines,
                            line_number,
                            token_col_start,
                            col_number,
                        );
                    };

                    let full_value = format!("{first_char}{value}");

                    // Value is binary literal
                    if value.is_numeric() {
                        if !value.is_binary() {
                            report_error(
                                "'%' Can only be used for binary literals!",
                                path,
                                lines,
                                line_number,
                                token_col_start,
                                col_number,
                            );
                        }

                        // Push binary token
                        tokens.push_back(Token {
                            line_number: line_number as u32,
                            column_start: token_col_start,
                            column_end: col_number,
                            value: full_value,
                            token_type: TokenType::Binary(value),
                        });

                        continue;
                    }

                    /* Otherwise must be a register name */

                    // Make sure register name is valie
                    if !value.is_alphanumeric() {
                        report_error(
                            "Register names must be alphanumeric!",
                            path,
                            lines,
                            line_number,
                            token_col_start,
                            col_number,
                        );
                    }

                    // Push register token
                    tokens.push_back(Token {
                        line_number: line_number as u32,
                        column_start: token_col_start,
                        column_end: col_number,
                        value: full_value,
                        token_type: TokenType::Register(value),
                    });
                }
                // Comma
                (',', _, _) => {
                    tokens.push_back(Token {
                        line_number: line_number as u32,
                        column_start: token_col_start,
                        column_end: col_number,
                        value: first_char.to_string(),
                        token_type: TokenType::Comma,
                    });
                }
                // Immediate Value
                ('#', _, _) => {
                    tokens.push_back(Token {
                        line_number: line_number as u32,
                        column_start: token_col_start,
                        column_end: col_number,
                        value: first_char.to_string(),
                        token_type: TokenType::Immediate,
                    });
                }
                // Hex Value
                ('$', _, _) => {
                    let value = read_to_chars(vec![' ', ',', ';', '(', ')', '[', ']'], &mut col_number, &mut chars);

                    let Some(value) = value else {
                        report_error(
                            "Unexpected end of hex literal token",
                            path,
                            lines,
                            line_number,
                            token_col_start,
                            col_number,
                        );
                    };

                    let full_value = format!("{first_char}{value}");

                    if !value.is_alphanumeric() {
                        report_error(
                            "Unexpected non-alphanumeric characters in hex literal!",
                            path,
                            lines,
                            line_number,
                            token_col_start,
                            col_number,
                        );
                    }

                    // Make sure the value is value hex
                    if !value.is_hex() {
                        report_error(
                            "'$' Can only be used for hex literals!",
                            path,
                            lines,
                            line_number,
                            token_col_start,
                            col_number,
                        );
                    }

                    // Push hex token
                    tokens.push_back(Token {
                        line_number: line_number as u32,
                        column_start: token_col_start,
                        column_end: col_number,
                        value: full_value,
                        token_type: TokenType::Hex(value),
                    });
                }
                (_, _, true) => {
                    let literal = read_to_chars(vec![' ', ',', ';', '(', ')', '[', ']'], &mut col_number, &mut chars);

                    let value = match literal {
                        Some(val) => val,
                        None => "".to_owned(),
                    };

                    let full_value = format!("{first_char}{value}");

                    if !value.is_numeric() {
                        report_error(
                            "Unexpected non-numeric characters in decimal literal!",
                            path,
                            lines,
                            line_number,
                            token_col_start,
                            col_number,
                        );
                    }

                    // Push decimal token
                    tokens.push_back(Token {
                        line_number: line_number as u32,
                        column_start: token_col_start,
                        column_end: col_number,
                        value: full_value.clone(),
                        token_type: TokenType::Decimal(full_value),
                    });
                }
                // Open Bracket
                ('[', _, _) => {
                    tokens.push_back(Token {
                        line_number: line_number as u32,
                        column_start: token_col_start,
                        column_end: col_number,
                        value: first_char.to_string(),
                        token_type: TokenType::OpenBracket,
                    });
                } // Close Bracket
                (']', _, _) => {
                    tokens.push_back(Token {
                        line_number: line_number as u32,
                        column_start: token_col_start,
                        column_end: col_number,
                        value: first_char.to_string(),
                        token_type: TokenType::CloseBracket,
                    });
                } // Open Parenthesis
                ('(', _, _) => {
                    tokens.push_back(Token {
                        line_number: line_number as u32,
                        column_start: token_col_start,
                        column_end: col_number,
                        value: first_char.to_string(),
                        token_type: TokenType::OpenParenthesis,
                    });
                } // Close Parenthesis
                (')', _, _) => {
                    tokens.push_back(Token {
                        line_number: line_number as u32,
                        column_start: token_col_start,
                        column_end: col_number,
                        value: first_char.to_string(),
                        token_type: TokenType::CloseParenthesis,
                    });
                }
                _ => {
                    report_error(
                        format!("Unexpected value '{first_char}' at start of token").as_str(),
                        path,
                        lines,
                        line_number,
                        token_col_start,
                        col_number,
                    );
                }
            }
        }
    }

    tokens
}

fn read_to_char_inclusive(
    character: char,
    col_number: &mut u32,
    chars: &mut VecDeque<char>,
) -> Option<String> {
    if chars.is_empty() {
        return None;
    }

    let mut string = String::new();

    while !chars.is_empty() {
        if *chars.front().unwrap() == character {
            string.push_str(chars.pop_front().unwrap().to_string().as_str());

            return if string.len() > 0 { Some(string) } else { None };
        }

        let character = chars.pop_front().unwrap();
        *col_number += 1;

        // TODO - probably not efficient
        string.push_str(character.to_string().as_str());
    }

    Some(string)
}

fn read_to_chars(
    characters: Vec<char>,
    col_number: &mut u32,
    chars: &mut VecDeque<char>,
) -> Option<String> {
    if chars.is_empty() {
        return None;
    }

    let mut string = String::new();

    while !chars.is_empty() {
        if characters.contains(chars.front().unwrap()) {
            return if string.len() > 0 { Some(string) } else { None };
        }

        let character = chars.pop_front().unwrap();
        *col_number += 1;

        // TODO - probably not efficient
        string.push_str(character.to_string().as_str());
    }

    Some(string)
}

trait Extractable {
    fn extract_range(&self, start: u32, end: u32) -> Self;
}

impl Extractable for String {
    fn extract_range(&self, start: u32, end: u32) -> String {
        let string = &*self;
        let slice = &string[start as usize..end as usize];
        slice.to_owned()
    }
}

trait Alphabetic {
    fn is_alphanumeric(&self) -> bool;
    fn is_numeric(&self) -> bool;
    fn is_binary(&self) -> bool;
    fn is_hex(&self) -> bool;
}

impl Alphabetic for String {
    fn is_alphanumeric(&self) -> bool {
        let re = Regex::new(r"^[a-zA-Z0-9_]*$").unwrap();
        re.is_match(self.as_str())
    }

    fn is_numeric(&self) -> bool {
        let re = Regex::new(r"^[0-9]*$").unwrap();
        re.is_match(self.as_str())
    }

    fn is_binary(&self) -> bool {
        let re = Regex::new(r"^[0-1]*$").unwrap();
        re.is_match(self.as_str())
    }

    fn is_hex(&self) -> bool {
        let re = Regex::new(r"^[0-9a-fA-F]*$").unwrap();
        re.is_match(self.as_str())
    }
}

impl Alphabetic for &str {
    fn is_alphanumeric(&self) -> bool {
        let re = Regex::new(r"^[a-zA-Z0-9_]*$").unwrap();
        re.is_match(self)
    }

    fn is_numeric(&self) -> bool {
        let re = Regex::new(r"^[0-9]*$").unwrap();
        re.is_match(self)
    }

    fn is_binary(&self) -> bool {
        let re = Regex::new(r"^[0-1]*$").unwrap();
        re.is_match(self)
    }

    fn is_hex(&self) -> bool {
        let re = Regex::new(r"^[0-9a-fA-F]*$").unwrap();
        re.is_match(self)
    }
}
