use crate::macros::{
    MacroDeclarationItem, MacroElement, MacroParam, MacroParamKind, MacroRepetition,
    MacroRepetitionOperator, MacroRule, TokenNode, WrappedMacro,
};
use crate::{CairoWrite, CairoWriteSlice};
impl CairoWrite for MacroDeclarationItem {
    fn cwrite<W: std::fmt::Write>(&self, buf: &mut W) -> std::fmt::Result {
        self.attributes.cwrite(buf)?;
        self.visibility.cwrite(buf)?;
        buf.write_str("macro ")?;
        self.name.cwrite(buf)?;
        buf.write_str(" {\n")?;
        self.rules.cwrite(buf)?;
        buf.write_str("\n}")
    }
}

impl CairoWrite for MacroRule {
    fn cwrite<W: std::fmt::Write>(&self, buf: &mut W) -> std::fmt::Result {
        self.lhs.cwrite(buf)?;
        buf.write_str(" => ")?;
        self.rhs.cwrite(buf)?;
        buf.write_char(';')
    }
}

impl CairoWrite for MacroParam {
    fn cwrite<W: std::fmt::Write>(&self, buf: &mut W) -> std::fmt::Result {
        buf.write_char('$')?;
        self.name.cwrite(buf)?;
        if let Some(kind) = &self.kind {
            kind.write_prefixed(buf, ':')?;
        }
        Ok(())
    }
}

impl CairoWrite for MacroParamKind {
    fn cwrite<W: std::fmt::Write>(&self, buf: &mut W) -> std::fmt::Result {
        match self {
            MacroParamKind::Identifier(value) | MacroParamKind::Expr(value) => buf.write_str(value),
            MacroParamKind::Missing => Ok(()),
        }
    }
}

impl CairoWrite for MacroRepetition {
    fn cwrite<W: std::fmt::Write>(&self, buf: &mut W) -> std::fmt::Result {
        buf.write_str("$(")?;
        self.elements.cwrite(buf)?;
        buf.write_char(')')?;
        if self.comma {
            buf.write_char(',')?;
        }
        self.operator.cwrite(buf)
    }
}

impl CairoWrite for MacroRepetitionOperator {
    fn cwrite<W: std::fmt::Write>(&self, buf: &mut W) -> std::fmt::Result {
        match self {
            MacroRepetitionOperator::ZeroOrOne => buf.write_str("?"),
            MacroRepetitionOperator::ZeroOrMore => buf.write_str("*"),
            MacroRepetitionOperator::OneOrMore => buf.write_str("+"),
            MacroRepetitionOperator::Missing => Ok(()),
        }
    }
}

impl CairoWrite for WrappedMacro {
    fn cwrite<W: std::fmt::Write>(&self, buf: &mut W) -> std::fmt::Result {
        match self {
            WrappedMacro::Parenthesized(elements) => elements.cwrite(buf),
            WrappedMacro::Braced(elements) => elements.cwrite_braced(buf),
            WrappedMacro::Bracketed(elements) => elements.cwrite_bracketed(buf),
        }
    }
}

impl CairoWrite for MacroElement {
    fn cwrite<W: std::fmt::Write>(&self, buf: &mut W) -> std::fmt::Result {
        match self {
            MacroElement::Token(t) => t.cwrite(buf),
            MacroElement::Param(p) => p.cwrite(buf),
            MacroElement::Subtree(r) => r.cwrite(buf),
            MacroElement::Repetition(w) => w.cwrite(buf),
        }
    }
}
