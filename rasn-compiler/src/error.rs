pub trait CompilerError: std::fmt::Debug {
    fn as_report(&self) -> String;
    fn as_styled_report(&self) -> String;
}