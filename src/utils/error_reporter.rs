use crate::core::InfraError;
use std::io::{self, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub struct ErrorReporter {
    had_error: bool,
    colored: bool,
}

impl ErrorReporter {
    pub fn new() -> Self {
        Self {
            had_error: false,
            colored: true,
        }
    }

    pub fn new_no_color() -> Self {
        Self {
            had_error: false,
            colored: false,
        }
    }

    pub fn report_error(&mut self, error: &InfraError) {
        self.had_error = true;

        let mut stderr = StandardStream::stderr(if self.colored {
            ColorChoice::Auto
        } else {
            ColorChoice::Never
        });

        match error {
            InfraError::LexError {
                message,
                line,
                column,
                source_code: _,
            } => {
                self.report_at(
                    &mut stderr,
                    *line,
                    *column,
                    "Lexical Error",
                    message,
                    Color::Red,
                );
            }
            InfraError::ParseError {
                message,
                line,
                column,
                source_code: _,
                hint,
            } => {
                self.report_at(
                    &mut stderr,
                    *line,
                    *column,
                    "Parse Error",
                    message,
                    Color::Red,
                );
                if let Some(hint_msg) = hint {
                    self.print_hint(&mut stderr, hint_msg);
                }
            }
            InfraError::RuntimeError {
                message,
                line,
                column,
                stack_trace,
                source_code: _,
            } => {
                if let (Some(l), Some(c)) = (line, column) {
                    self.report_at(&mut stderr, *l, *c, "Runtime Error", message, Color::Red);
                } else {
                    self.print_error(&mut stderr, "Runtime Error", message, Color::Red);
                }

                if !stack_trace.is_empty() {
                    self.print_stack_trace(&mut stderr, stack_trace);
                }
            }
            InfraError::TypeError {
                expected,
                found,
                context,
                line,
                column,
                hint,
            } => {
                let message = if let Some(ctx) = context {
                    format!("expected {}, found {} (in {})", expected, found, ctx)
                } else {
                    format!("expected {}, found {}", expected, found)
                };

                if let (Some(l), Some(c)) = (line, column) {
                    self.report_at(&mut stderr, *l, *c, "Type Error", &message, Color::Magenta);
                } else {
                    self.print_error(&mut stderr, "Type Error", &message, Color::Magenta);
                }

                if let Some(hint_msg) = hint {
                    self.print_hint(&mut stderr, hint_msg);
                }
            }
            InfraError::DivisionByZero { line, column } => {
                if let (Some(l), Some(c)) = (line, column) {
                    self.report_at(
                        &mut stderr,
                        *l,
                        *c,
                        "Runtime Error",
                        "Division by zero",
                        Color::Red,
                    );
                } else {
                    self.print_error(&mut stderr, "Runtime Error", "Division by zero", Color::Red);
                }
            }
            InfraError::UndefinedVariable {
                name,
                line,
                column,
                suggestion,
            } => {
                if let (Some(l), Some(c)) = (line, column) {
                    self.report_at(
                        &mut stderr,
                        *l,
                        *c,
                        "Runtime Error",
                        &format!("Undefined variable '{}'", name),
                        Color::Red,
                    );
                } else {
                    self.print_error(
                        &mut stderr,
                        "Runtime Error",
                        &format!("Undefined variable '{}'", name),
                        Color::Red,
                    );
                }

                if let Some(sugg) = suggestion {
                    self.print_suggestion(&mut stderr, sugg);
                }
            }
            InfraError::UndefinedFunction {
                name,
                line,
                column,
                suggestion,
            } => {
                if let (Some(l), Some(c)) = (line, column) {
                    self.report_at(
                        &mut stderr,
                        *l,
                        *c,
                        "Runtime Error",
                        &format!("Undefined function '{}'", name),
                        Color::Red,
                    );
                } else {
                    self.print_error(
                        &mut stderr,
                        "Runtime Error",
                        &format!("Undefined function '{}'", name),
                        Color::Red,
                    );
                }

                if let Some(sugg) = suggestion {
                    self.print_suggestion(&mut stderr, sugg);
                }
            }
            InfraError::ArgumentCountMismatch {
                expected,
                found,
                function_name,
                line,
            } => {
                let message = if let Some(func_name) = function_name {
                    format!(
                        "Function '{}' expected {} arguments, found {}",
                        func_name, expected, found
                    )
                } else {
                    format!("Expected {} arguments, found {}", expected, found)
                };

                if let Some(l) = line {
                    self.report_at(&mut stderr, *l, 0, "Runtime Error", &message, Color::Red);
                } else {
                    self.print_error(&mut stderr, "Runtime Error", &message, Color::Red);
                }
            }
            InfraError::IndexOutOfBounds {
                index,
                length,
                array_name,
                line,
            } => {
                let message = if let Some(name) = array_name {
                    format!(
                        "Array index {} out of bounds for '{}' (length: {})",
                        index, name, length
                    )
                } else {
                    format!(
                        "Array index {} out of bounds for array of length {}",
                        index, length
                    )
                };

                if let Some(l) = line {
                    self.report_at(&mut stderr, *l, 0, "Runtime Error", &message, Color::Red);
                } else {
                    self.print_error(&mut stderr, "Runtime Error", &message, Color::Red);
                }
            }
            InfraError::PropertyNotFound {
                property,
                object_type,
                line,
                available_properties,
            } => {
                let message = if let Some(obj_type) = object_type {
                    format!("Property '{}' not found on {}", property, obj_type)
                } else {
                    format!("Property '{}' not found on object", property)
                };

                if let Some(l) = line {
                    self.report_at(&mut stderr, *l, 0, "Runtime Error", &message, Color::Red);
                } else {
                    self.print_error(&mut stderr, "Runtime Error", &message, Color::Red);
                }

                if let Some(props) = available_properties {
                    self.print_available_properties(&mut stderr, props);
                }
            }
            InfraError::ReturnValue(value) => {
                // This should not be reported as an error in normal operation
                if let Some(val) = value {
                    self.print_error(
                        &mut stderr,
                        "Internal Error",
                        &format!("Unexpected return: {}", val),
                        Color::Yellow,
                    );
                } else {
                    self.print_error(
                        &mut stderr,
                        "Internal Error",
                        "Unexpected return",
                        Color::Yellow,
                    );
                }
            }
            InfraError::IoError {
                message,
                operation,
                path,
            } => {
                let mut error_msg = message.clone();
                if let Some(op) = operation {
                    error_msg = format!("{} during operation '{}'", error_msg, op);
                }
                if let Some(p) = path {
                    error_msg = format!("{} at path '{}'", error_msg, p);
                }

                self.print_error(&mut stderr, "I/O Error", &error_msg, Color::Red);
            }
            InfraError::Exception {
                message,
                exception_type,
                line,
                stack_trace,
            } => {
                let error_type = exception_type.as_deref().unwrap_or("Exception");

                if let Some(l) = line {
                    self.report_at(&mut stderr, *l, 0, error_type, message, Color::Red);
                } else {
                    self.print_error(&mut stderr, error_type, message, Color::Red);
                }

                if !stack_trace.is_empty() {
                    self.print_stack_trace(&mut stderr, stack_trace);
                }
            }
            InfraError::ModuleError {
                module_name,
                reason,
            } => {
                let message = format!("Could not load '{}': {}", module_name, reason);
                self.print_error(&mut stderr, "Module Error", &message, Color::Red);
            }
            InfraError::AsyncError { message, operation } => {
                let error_msg = if let Some(op) = operation {
                    format!("in {}: {}", op, message)
                } else {
                    message.clone()
                };

                self.print_error(&mut stderr, "Async Error", &error_msg, Color::Red);
            }
            InfraError::ClassError {
                message,
                class_name,
                method_name,
                line,
            } => {
                let error_msg = if let Some(method) = method_name {
                    if let Some(class) = class_name {
                        format!("in method '{}' (class: {}): {}", method, class, message)
                    } else {
                        format!("in method '{}': {}", method, message)
                    }
                } else if let Some(class) = class_name {
                    format!("in class '{}': {}", class, message)
                } else {
                    message.clone()
                };

                if let Some(l) = line {
                    self.report_at(&mut stderr, *l, 0, "Class Error", &error_msg, Color::Red);
                } else {
                    self.print_error(&mut stderr, "Class Error", &error_msg, Color::Red);
                }
            }
            InfraError::MemoryError { message, operation } => {
                let error_msg = if let Some(op) = operation {
                    format!("during {}: {}", op, message)
                } else {
                    message.clone()
                };

                self.print_error(&mut stderr, "Memory Error", &error_msg, Color::Red);
            }
            InfraError::Generic(message) => {
                self.print_error(&mut stderr, "Error", message, Color::Red);
            }
        }

        stderr.flush().unwrap();
    }

    pub fn had_error(&self) -> bool {
        self.had_error
    }

    pub fn reset(&mut self) {
        self.had_error = false;
    }

    fn report_at(
        &self,
        writer: &mut StandardStream,
        line: usize,
        column: usize,
        error_type: &str,
        message: &str,
        color: Color,
    ) {
        writer
            .set_color(ColorSpec::new().set_bold(true).set_fg(Some(color)))
            .unwrap();
        write!(writer, "{}", error_type).unwrap();
        writer.reset().unwrap();

        if column > 0 {
            writeln!(writer, " [line {}, column {}]: {}", line, column, message).unwrap();
        } else {
            writeln!(writer, " [line {}]: {}", line, message).unwrap();
        }
    }

    fn print_error(
        &self,
        writer: &mut StandardStream,
        error_type: &str,
        message: &str,
        color: Color,
    ) {
        writer
            .set_color(ColorSpec::new().set_bold(true).set_fg(Some(color)))
            .unwrap();
        write!(writer, "{}", error_type).unwrap();
        writer.reset().unwrap();
        writeln!(writer, ": {}", message).unwrap();
    }

    fn print_hint(&self, writer: &mut StandardStream, hint: &str) {
        writer
            .set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))
            .unwrap();
        writeln!(writer, "  💡 Hint: {}", hint).unwrap();
        writer.reset().unwrap();
    }

    fn print_suggestion(&self, writer: &mut StandardStream, suggestion: &str) {
        writer
            .set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))
            .unwrap();
        writeln!(writer, "  🤔 Did you mean '{}'?", suggestion).unwrap();
        writer.reset().unwrap();
    }

    fn print_available_properties(&self, writer: &mut StandardStream, properties: &[String]) {
        writer
            .set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))
            .unwrap();
        writeln!(
            writer,
            "  📋 Available properties: {}",
            properties.join(", ")
        )
        .unwrap();
        writer.reset().unwrap();
    }

    fn print_stack_trace(&self, writer: &mut StandardStream, stack_trace: &[String]) {
        writer
            .set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))
            .unwrap();
        writeln!(writer, "  📚 Stack trace:").unwrap();

        for (i, frame) in stack_trace.iter().enumerate() {
            write!(writer, "    {}. ", i + 1).unwrap();
            writer.set_color(ColorSpec::new().set_dimmed(true)).unwrap();
            writeln!(writer, "{}", frame).unwrap();
            writer.reset().unwrap();
        }
    }
}

impl Default for ErrorReporter {
    fn default() -> Self {
        Self::new()
    }
}
