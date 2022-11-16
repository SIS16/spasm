use core::panic;
use std::{collections::VecDeque, num::IntErrorKind, path::PathBuf};

use crate::{
    report_error,
    token::{Token, TokenType},
};

#[derive(Debug)]
pub struct Program {
    pub text: Option<TextSection>,
    pub data: Option<DataSection>,
}

impl Program {
    fn new() -> Program {
        Program {
            text: None,
            data: None,
        }
    }

    fn find_constant_label(&self, name: &str) -> Option<&ConstantLabel> {
        let Some(data) = &self.data else {
            return None;
        };

        for label in &data.labels {
            if label.name == name {
                return Some(&label);
            }
        }

        None
    }

    fn find_subroutine_label(&self, name: &str) -> Option<&SubroutineLabel> {
        let Some(text) = &self.text else {
            return None;
        };

        for label in &text.labels {
            if label.name == name {
                return Some(&label);
            }
        }

        None
    }
}

trait Parsable {
    fn parse(path: &PathBuf, lines: &Vec<String>, tokens: &mut VecDeque<Token>) -> Self
    where
        Self: Sized;
}

#[derive(Debug)]
pub struct DataSection {
    labels: Vec<ConstantLabel>,
}

#[derive(Debug)]
pub struct ConstantLabel {
    name: String,
    constants: Vec<ConstantLabelType>,
}

#[derive(Debug)]
pub enum ConstantLabelType {
    StringLiteral(String),
    Word(u16),
}

impl Parsable for DataSection {
    fn parse(path: &PathBuf, lines: &Vec<String>, tokens: &mut VecDeque<Token>) -> DataSection {
        let mut data = DataSection { labels: Vec::new() };

        // Loop through every label in the section
        while !tokens.is_empty() {
            let first_token = tokens.pop_front().unwrap();

            // Check for end of section or illegal directives
            if let TokenType::Directive(name) = &first_token.token_type {
                if name == "data" || name == "text" {
                    tokens.push_front(first_token);
                    return data;
                } else {
                    report_error(
                        format!("Illegal directive token `.{}`", name).as_str(),
                        path,
                        lines,
                        first_token.line_number,
                        first_token.column_start,
                        first_token.column_end,
                    )
                }
            };

            // Start parsing this section as a label
            let TokenType::Label(label_name) = first_token.token_type else {
                report_error(
                    format!("Unexpected token '{:?}' in data section.", first_token.token_type).as_str(),
                    path,
                    lines,
                    first_token.line_number,
                    first_token.column_start,
                    first_token.column_end,
                )
            };

            let mut constant_label = ConstantLabel {
                name: label_name,
                constants: Vec::new(),
            };

            let mut constant_tokens = read_tokens_to_label_or_eos(tokens);

            if constant_tokens.len() == 0 {
                report_error(
                    format!("Label `{}` cannot be empty!", constant_label.name).as_str(),
                    path,
                    lines,
                    first_token.line_number,
                    first_token.column_start,
                    first_token.column_end,
                )
            }

            while !constant_tokens.is_empty() {
                if constant_tokens.len() == 1 {
                    let token = constant_tokens.front().unwrap();

                    report_error(
                        "Expected at least 2 tokens in constant.",
                        path,
                        lines,
                        token.line_number,
                        token.column_start,
                        token.column_end,
                    )
                }

                let directive_token = constant_tokens.pop_front().unwrap();
                let constant_token = constant_tokens.pop_front().unwrap();

                let TokenType::Directive(directive) = &directive_token.token_type else {
                    report_error(
                        "First token in a constant must be a directive!",
                        path,
                        lines,
                        directive_token.line_number,
                        directive_token.column_start,
                        directive_token.column_end,
                    )
                };

                match directive.as_str() {
                    "ascii" => {
                        // Assume the next constant is a string
                        let TokenType::AsciiString(string) = &constant_token.token_type else {
                            report_error(
                                format!("Expected string literal after .ascii directive!").as_str(),
                                path,
                                lines,
                                constant_token.line_number,
                                constant_token.column_start,
                                constant_token.column_end,
                            )
                        };

                        constant_label
                            .constants
                            .push(ConstantLabelType::StringLiteral(string.clone()))
                    }
                    "word" => {
                        match &constant_token.token_type {
                            TokenType::Binary(value) => {
                                // Parse from string value
                                let bin_value = match u16::from_str_radix(value, 2) {
                                    Ok(v) => v,
                                    Err(err) => match err.kind() {
                                        // Greater than a 16 bit word
                                        IntErrorKind::PosOverflow =>  report_error(
                                            "Binary literal is larger than expected 16-bit word! (Max is %1111111111111111)",
                                            path,
                                            lines,
                                            constant_token.line_number,
                                            constant_token.column_start,
                                            constant_token.column_end,
                                        ),
                                        kind => panic!("Unexpected IntErrorKind {kind:?}")
                                    }
                                };

                                // Add constant to current label
                                constant_label
                                    .constants
                                    .push(ConstantLabelType::Word(bin_value))
                            }
                            TokenType::Decimal(value) => {
                                // Parse from string value
                                let dec_value = match u16::from_str_radix(value, 10) {
                                    Ok(v) => v,
                                    Err(err) => match err.kind() {
                                        // Greater than a 16 bit word
                                        IntErrorKind::PosOverflow =>  report_error(
                                            "Decimal literal is larger than expected 16-bit word! (Max is 65535)",
                                            path,
                                            lines,
                                            constant_token.line_number,
                                            constant_token.column_start,
                                            constant_token.column_end,
                                        ),
                                        kind => panic!("Unexpected IntErrorKind {kind:?}")
                                    }
                                };

                                // Add constant to current label
                                constant_label
                                    .constants
                                    .push(ConstantLabelType::Word(dec_value))
                            }
                            TokenType::Hex(value) => {
                                // Parse from string value
                                let hex_value = match u16::from_str_radix(value, 16) {
                                    Ok(v) => v,
                                    Err(err) => match err.kind() {
                                        // Greater than a 16 bit word
                                        IntErrorKind::PosOverflow =>  report_error(
                                            "Hexadecimal literal is larger than expected 16-bit word! (Max is $FFFF)",
                                            path,
                                            lines,
                                            constant_token.line_number,
                                            constant_token.column_start,
                                            constant_token.column_end,
                                        ),
                                        kind => panic!("Unexpected IntErrorKind {kind:?}")
                                    }
                                };

                                // Add constant to current label
                                constant_label
                                    .constants
                                    .push(ConstantLabelType::Word(hex_value))
                            }
                            TokenType::Immediate => report_error(
                                "The .word directive does not require an immediate `#` marker!",
                                path,
                                lines,
                                constant_token.line_number,
                                constant_token.column_start,
                                constant_token.column_end,
                            ),
                            _ => report_error(
                                "Expected a number literal after .word directive!",
                                path,
                                lines,
                                constant_token.line_number,
                                constant_token.column_start,
                                constant_token.column_end,
                            ),
                        }
                    }
                    _ => report_error(
                        format!("Unknown constant directive `.{directive}`!").as_str(),
                        path,
                        lines,
                        directive_token.line_number,
                        directive_token.column_start,
                        directive_token.column_end,
                    ),
                }
            }

            data.labels.push(constant_label);

            // println!("{data:#?}");
        }

        data
    }
}

#[derive(Debug)]
pub struct TextSection {
    labels: Vec<SubroutineLabel>,
}

#[derive(Debug)]
pub struct SubroutineLabel {
    name: String,
    instructions: Vec<Instruction>,
}

#[derive(Debug)]
pub enum InstructionArgumentType {
    NumberImmediate(u16),       // Immediate Value - #$FFFF
    MemoryAddress(u16),         // Memory Address - $FFFF
    MemoryAddressIndirect(u16), // Memory Address - ($FFFF)
    LabelAddress(String),       // Label Name - boot_loader
    LabelValue(String),         // Label Name - [boot_loader]
    Register(Register),         // Register - %eax
}

impl Parsable for InstructionArgumentType {
    fn parse(
        path: &PathBuf,
        lines: &Vec<String>,
        tokens: &mut VecDeque<Token>,
    ) -> InstructionArgumentType {
        let first_token = tokens.pop_front().unwrap();

        todo!()
    }
}

type InstructionArguments = VecDeque<InstructionArgumentType>;

impl Parsable for InstructionArguments {
    fn parse(
        path: &PathBuf,
        lines: &Vec<String>,
        argument_tokens: &mut VecDeque<Token>,
    ) -> InstructionArguments {
        let mut arguments = InstructionArguments::new();

        let mut args = split_tokens_by_commas(path, lines, argument_tokens);

        println!("args = {args:#?}");

        while !args.is_empty() {
            let mut arg = args.pop_front().unwrap();

            arguments.push_back(InstructionArgumentType::parse(path, lines, &mut arg))
        }

        arguments
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Register {
    /* 8-bit */
    AX,
    BX,
    CX,
    DX,
    EX,
    /* 16-bit */
    EAX,
    EBX,
    ECX,
    EDX,
    EEX,
}

impl Register {
    fn from_name(name: String) -> Option<Register> {
        let reg = match name.to_lowercase().as_str() {
            "ax" => Register::AX,
            "bx" => Register::BX,
            "cx" => Register::CX,
            "dx" => Register::DX,
            "ex" => Register::EX,
            "eax" => Register::EAX,
            "ebx" => Register::EBX,
            "ecx" => Register::ECX,
            "edx" => Register::EDX,
            "eex" => Register::EEX,
            _ => return None,
        };

        Some(reg)
    }
}

impl Parsable for TextSection {
    fn parse(path: &PathBuf, lines: &Vec<String>, tokens: &mut VecDeque<Token>) -> TextSection {
        let mut text = TextSection { labels: Vec::new() };

        // Loop through every label in the section
        while !tokens.is_empty() {
            let first_token = tokens.pop_front().unwrap();

            // Check for end of section or illegal directives
            if let TokenType::Directive(name) = &first_token.token_type {
                if name == "data" || name == "text" {
                    tokens.push_front(first_token);
                    return text;
                } else {
                    report_error(
                        format!("Illegal directive token `.{}`", name).as_str(),
                        path,
                        lines,
                        first_token.line_number,
                        first_token.column_start,
                        first_token.column_end,
                    )
                }
            };

            // Start parsing this section as a label
            let TokenType::Label(label_name) = first_token.token_type else {
                report_error(
                    format!("Unexpected token '{:?}' in text section.", first_token.token_type).as_str(),
                    path,
                    lines,
                    first_token.line_number,
                    first_token.column_start,
                    first_token.column_end,
                )
            };

            let mut subroutine_label = SubroutineLabel {
                name: label_name,
                instructions: Vec::new(),
            };

            // Read all the tokens in this label
            let mut subroutine_tokens = read_tokens_to_label_or_eos(tokens);

            // Subroutine labels need to have instructions in them
            if subroutine_tokens.len() == 0 {
                report_error(
                    format!("Label `{}` cannot be empty!", subroutine_label.name).as_str(),
                    path,
                    lines,
                    first_token.line_number,
                    first_token.column_start,
                    first_token.column_end,
                )
            }

            // Read tokens one line at a time until we reach the end of the subroutine
            while !subroutine_tokens.is_empty() {
                let mut line = read_tokens_to_eol(&mut subroutine_tokens);

                let first_line_token = line.pop_front().unwrap();

                // Make sure first token is an instruction
                let TokenType::Instruction(instruction_mnemonic) = &first_line_token.token_type else {
                    report_error(
                       "Lines inside a subroutine must start with an instruction",
                        path,
                        lines,
                        first_line_token.line_number,
                        first_line_token.column_start,
                        first_line_token.column_end,
                    )
                };

                println!("Found instruction: {instruction_mnemonic:#?}");
                println!("Tokens in instruction: {line:#?}");

                let instruction_arguments = InstructionArguments::parse(path, lines, &mut line);

                println!("Parsed arguments: {instruction_arguments:#?}");

                let instruction = Instruction::parse(instruction_mnemonic, instruction_arguments);
            }

            text.labels.push(subroutine_label);
        }

        text
    }
}

#[rustfmt::skip]
#[derive(Debug)]
#[allow(non_camel_case_types, dead_code)]
pub enum Instruction {
    /* nop :O */
    nop,                                            // nop                  ; No Operation
    /* mov */
    mov_RegisterToMemory(u16, Register),            // mov $F354, %eax      ; Copy value in %eax to mem address $F354
    mov_MemoryToRegister(Register, u16),            // mov %eax, $F354      ; Copy value in mem address $F354 to %eax
    mov_ImmediateToRegister(Register, u16),         // mov %eax, #$F354     ; Copy immediate value #$F354 to %eax
    mov_RegisterToRegister(Register, Register),     // mov %eax, %ebx       ; Copy value in %ebx to %eax
    mov_ImmediateToMemory8(u16, u8),                // mov $F354, #69       ; Copy 8 bit immediate #69 to mem address $F354
    mov_ImmediateToMemory16(u16, u16),              // mov $F354, #420      ; Copy 16 bit immediate #420 to mem addresses $F354-F355
    mov_ImmediateToMemory32(u16, u32),              // mov $F354, #69420    ; Copy 32 bit immediate #69420 to mem address $F354-F357
    /* add - accumulator */
    add_RegisterToAccumulator(Register),            // add %ebx             ; Add the value of %ebx to the accumulator register
    add_ImmediateToAccumulator(u16),                // add #2               ; Add 2 to the accumulator register
    /* add - to register */
    add_RegisterToRegister(Register, Register),     // add %ebx, ecx        ; Add the value of %ecx to the value in %ebx
    add_ImmediateToRegister(Register, u16),         // add %ebx, #2         ; Add 2 to the value in %ebx
    /* inc/dec - accumulator */
    inc_Accumulator,                                // inc                  ; Increment the accumulator
    dec_Accumulator,                                // dec                  ; Decrement the accumulator
    /* inc/dec - register */
    inc_Register(Register),                         // inc %ebx             ; Increment the %ebx register
    dec_Register(Register),                         // dec %ebx             ; Decrement the %ebx register
    /* jumps */
    jmp_Immediate(u16),                             // jmp #$F354           ; Jump to memory address #$F354
    jmp_Register(Register),                         // jmp %ebx             ; Jump to memory address stored in %ebx
    jmp_Memory(u16),                                // jmp $F354            ; Jump to memory address stored in address $F354
    jsr(SubroutineLabel),                           // jsr boot_loader      ; Push current pc onto stack and jump to subroutine boot_loader
    ret,                                            // ret                  ; Pop return address off stack and jump back
    /* syscalls */
    syscall,                                        // syscall              ; Jump to the syscall handler
    ssc(u16),                                       // ssc #$00FF           ; Sets the syscall handler register to the value #$00FF
    /* stack */
    push_Immediate(u16),                            // push #$420           ; Pushes the value #$420 onto the stack
    push_Memory(u16),                               // push $420            ; Pushes the value at mem address $420 onto the stack
    push_Register(Register),                        // push %ebx            ; Pushes the value in %ebx onto the stack
    pop_Memory(u16),                                // pop $420             ; Pops the top value on the stack into mem address $420
    pop_Register(Register),                         // pop %ebx             ; Pops the top value on the stack into %ebx
}

impl Instruction {
    fn parse(
        instruction_mnemonic: &String,
        instruction_arguments: InstructionArguments,
    ) -> Instruction {
        todo!()
    }
}

pub fn build_program(path: &PathBuf, lines: &Vec<String>, tokens: &mut VecDeque<Token>) -> Program {
    let mut ast = Program::new();

    while !tokens.is_empty() {
        let token = tokens.pop_front().unwrap();

        let TokenType::Directive(name) = token.token_type else {
            report_error(
                format!("Unexpected token '{:?}'. Program should start with either .data or .text section directive!", token.token_type).as_str(),
                path,
                lines,
                token.line_number,
                token.column_start,
                token.column_end,
            )
        };

        match name.as_str() {
            "data" => {
                if ast.data.is_none() {
                    ast.data = Some(DataSection::parse(path, lines, tokens));
                } else {
                    report_error(
                        "Duplicate section '.data'",
                        path,
                        lines,
                        token.line_number,
                        token.column_start,
                        token.column_end,
                    )
                }
            }
            "text" => {
                if ast.text.is_none() {
                    ast.text = Some(TextSection::parse(path, lines, tokens));
                } else {
                    report_error(
                        "Duplicate section '.text'",
                        path,
                        lines,
                        token.line_number,
                        token.column_start,
                        token.column_end,
                    )
                }
            }
            _ => report_error(
                "Expected program to start with either .data or .text section!",
                path,
                lines,
                token.line_number,
                token.column_start,
                token.column_end,
            ),
        }
    }

    ast
}

/**
 * Read tokens to the end of the line for parsing
 */
fn read_tokens_to_eol(tokens: &mut VecDeque<Token>) -> VecDeque<Token> {
    let mut line = VecDeque::new();

    if tokens.is_empty() {
        return line;
    }

    let first_token = tokens.pop_front().unwrap();
    line.push_back(first_token);

    while !tokens.is_empty()
        && tokens.front().unwrap().line_number == line.front().unwrap().line_number
    {
        line.push_back(tokens.pop_front().unwrap());
    }

    line
}

/**
 * Read tokens until the next label or end of section
 */
fn read_tokens_to_label_or_eos(tokens: &mut VecDeque<Token>) -> VecDeque<Token> {
    let mut result = VecDeque::new();

    if tokens.is_empty() {
        return result;
    }

    while !tokens.is_empty()
        && !match &tokens.front().unwrap().token_type {
            TokenType::Directive(name) => match name.as_str() {
                "text" => true,
                "data" => true,
                _ => false,
            },
            TokenType::Label(_) => true,
            _ => false,
        }
    {
        result.push_back(tokens.pop_front().unwrap());
    }

    result
}

/**
 * Split a token vector by the commas
 */
fn split_tokens_by_commas(
    path: &PathBuf,
    lines: &Vec<String>,
    tokens: &mut VecDeque<Token>,
) -> VecDeque<VecDeque<Token>> {
    let mut result = VecDeque::new();

    if tokens.is_empty() {
        return result;
    }

    let mut current_argument = VecDeque::new();

    // Loop through the tokens, and if we reach a comma,
    // push the current argument into result list,
    // else push it into the current argument
    while !tokens.is_empty() {
        let token = tokens.pop_front().unwrap();

        match &token.token_type {
            TokenType::Comma => {
                // Make sure there are not 2 commas in a row,
                // a comma before the first argument, or a comma at the end of a line
                if current_argument.is_empty() || tokens.is_empty() {
                    report_error(
                        "Unexpected argument separator `,`!",
                        path,
                        lines,
                        token.line_number,
                        token.column_start,
                        token.column_end,
                    )
                }

                result.push_back(current_argument);
                current_argument = VecDeque::new();
            }
            _ => {
                current_argument.push_back(token);
            }
        }
    }

    // Catch tokens that didn't have a comma after them
    if !current_argument.is_empty() {
        result.push_back(current_argument);
    }

    result
}
