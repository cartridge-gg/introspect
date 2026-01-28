use crate::syntax::common::Identifier;
use crate::syntax::expr::{Expr, ExprPath};
use crate::syntax::module::Item;
use crate::{
    AsCairo, Attribute, CollectionsAsCairo, Modifier, from_typed_syntax_node, syntax_enum,
    syntax_option, syntax_terminal_bool, syntax_type, vec_from_element_list,
};

syntax_enum! {
    Statement{
        Let(LetStatement),
        Expr(ExprStatement),
        Continue(ContinueStatement),
        Return(ReturnStatement),
        Break(BreakStatement),
        Item(Item),
        Missing,
    }
}

syntax_enum! {
    Pattern{
        Underscore,
        Literal(String),
        False,
        True,
        ShortString(String),
        String(String),
        Identifier(Identifier),
        Struct(PatternStruct),
        Tuple(Vec<Pattern>),
        Enum(PatternEnum),
        FixedSizeArray(Vec<Pattern>),
        Path(ExprPath),
    }
}

syntax_enum! {
    Condition{
        Let(LetCondition),
        Expr(Expr),
    }
}

syntax_enum! {
    PatternStructParam{
        Single(Identifier),
        WithExpr(ParamWithPatten),
        Tail,
    }
}

syntax_type! {
    ExprStatement[StatementExpr]{
        attributes: Vec<Attribute>,
        expr: Expr,
        semicolon: bool,
    }
}

syntax_type! {
    LetStatement[StatementLet]{
        attributes: Vec<Attribute>,
        pattern: Pattern,
        type_clause: Option<Expr>,
        rhs: Expr,
        let_else_clause: Option<Vec<Statement>>,
    }
}

syntax_type! {
    ContinueStatement[StatementContinue]{
        attributes: Vec<Attribute>,
    }
}

syntax_type! {
    ReturnStatement[StatementReturn]{
        attributes: Vec<Attribute>,
        expr[expr_clause]: Option<Expr>,
    }
}

syntax_type! {
    BreakStatement[StatementBreak]{
        attributes: Vec<Attribute>,
        expr[expr_clause]: Option<Expr>,
    }
}

syntax_type! {
    PatternEnum{
        path: ExprPath,
        pattern: Option<Box<Pattern>>,
    }
}

syntax_type! {
    ParamWithPatten[PatternStructParamWithExpr]{
        modifiers: Vec<Modifier>,
        name: String,
        pattern: Box<Pattern>,
    }
}

syntax_type! {
    PatternStruct{
        path: ExprPath,
        params: Vec<PatternStructParam>,
    }
}

syntax_type! {
    LetCondition[ConditionLet]{
    patterns: Vec<Pattern>,
    expr: Expr,
}}

vec_from_element_list! {PatternTuple.patterns, Pattern}
vec_from_element_list! {PatternFixedSizeArray.patterns, Pattern}
vec_from_element_list! {PatternStructParamList, PatternStructParam}
vec_from_element_list! {StatementList, Statement}
vec_from_element_list! {PatternListOr, Pattern}
vec_from_element_list! {ConditionListAnd, Condition}
vec_from_element_list! {LetElseClause.else_block.statements, Statement}

syntax_terminal_bool! {Semicolon}

syntax_option! {
    OptionPatternEnumInnerPattern{PatternEnumInnerPattern: Box<Pattern>}
}

syntax_option! {OptionLetElseClause{LetElseClause: Vec<Statement>}}

from_typed_syntax_node! {PatternEnumInnerPattern.pattern, Pattern}
from_typed_syntax_node! {ConditionExpr.expr, Expr}
from_typed_syntax_node! {StatementItem.item, Item}

// impl AsCairo for Statement {
//     fn as_cairo(&self) -> String {
//         match self {
//             Statement::Let(e) => e.as_cairo(),
//             Statement::Expr(e) => e.as_cairo(),
//             Statement::Continue(e) => e.as_cairo(),
//             Statement::Return(e) => e.as_cairo(),
//             Statement::Break(e) => e.as_cairo(),
//             Statement::Item(e) => e.as_cairo(),
//             Statement::Missing => "".to_string(),
//         }
//     }
// }

// impl AsCairo for Pattern {
//     fn as_cairo(&self) -> String {
//         match self {
//             Pattern::Underscore => "_".to_string(),
//             Pattern::Literal(e) => e.to_string(),
//             Pattern::False => "false".to_string(),
//             Pattern::True => "true".to_string(),
//             Pattern::ShortString(s) => s.clone(),
//             Pattern::String(s) => s.clone(),
//             Pattern::Identifier(ident) => ident.as_cairo(),
//             Pattern::Struct(s) => s.as_cairo(),
//             Pattern::Tuple(t) => t.as_cairo_tuple(),
//             Pattern::Enum(e) => e.as_cairo(),
//             Pattern::FixedSizeArray(arr) => arr.as_cairo_csv_bracketed(),
//             Pattern::Path(p) => p.as_cairo(),
//         }
//     }
// }

// impl AsCairo for Identifier {
//     fn as_cairo(&self) -> String {
//         format!("{}{}", self.modifiers.as_cairo(), self.name)
//     }
// }

// impl AsCairo for PatternStruct {
//     fn as_cairo(&self) -> String {
//         let path = self.path.as_cairo();
//         let params = self.params.as_cairo_csv();
//         format!("{path}{{{params}}}")
//     }
// }

// impl AsCairo for PatternEnum {
//     fn as_cairo(&self) -> String {
//         let path = self.path.as_cairo();
//         match &self.pattern {
//             Some(pattern) => format!("{path}({})", pattern.as_cairo()),
//             None => path,
//         }
//     }
// }

// impl AsCairo for PatternStructParam {
//     fn as_cairo(&self) -> String {
//         match self {
//             PatternStructParam::Single(ident) => ident.as_cairo(),
//             PatternStructParam::WithExpr(param) => param.as_cairo(),
//             PatternStructParam::Tail => "..".to_string(),
//         }
//     }
// }

// impl AsCairo for ParamWithPatten {
//     fn as_cairo(&self) -> String {
//         let modifiers = self.modifiers.as_cairo();
//         let name = &self.name;
//         let pattern = self.pattern.as_cairo();
//         format!("{modifiers}{name}: {pattern}")
//     }
// }

// impl AsCairo for LetStatement {
//     fn as_cairo(&self) -> String {
//         let type_clause = match &self.type_clause {
//             Some(ty) => format!(":{}", ty.as_cairo()),
//             None => "".to_string(),
//         };
//         let let_else_clause = match &self.let_else_clause {
//             Some(stmts) => format!(" else {{{}}}", stmts.as_cairo_block_section()),
//             None => "".to_string(),
//         };
//         let attributes = self.attributes.as_cairo_block_section();
//         let patten = self.pattern.as_cairo();
//         let rhs = self.rhs.as_cairo();
//         format!("{attributes}let {patten}{type_clause} = {rhs}{let_else_clause};",)
//     }
// }

// impl AsCairo for ExprStatement {
//     fn as_cairo(&self) -> String {
//         let attributes = self.attributes.as_cairo_block_section();
//         let expr = self.expr.as_cairo();
//         let semicolon = if self.semicolon { ";" } else { "" };
//         format!("{attributes}{expr}{semicolon}")
//     }
// }

// impl AsCairo for ContinueStatement {
//     fn as_cairo(&self) -> String {
//         let attributes = self.attributes.as_cairo_block_section();
//         format!("{attributes}continue;")
//     }
// }

// impl AsCairo for ReturnStatement {
//     fn as_cairo(&self) -> String {
//         let attributes = self.attributes.as_cairo_block_section();
//         let expr = match &self.expr {
//             Some(expr) => format!(" {}", expr.as_cairo()),
//             None => "".to_string(),
//         };
//         format!("{attributes}return{expr};")
//     }
// }

// impl AsCairo for BreakStatement {
//     fn as_cairo(&self) -> String {
//         let attributes = self.attributes.as_cairo_block_section();
//         let expr = match &self.expr {
//             Some(expr) => format!(" {}", expr.as_cairo()),
//             None => "".to_string(),
//         };
//         format!("{attributes}break{expr};")
//     }
// }

// impl AsCairo for Condition {
//     fn as_cairo(&self) -> String {
//         match self {
//             Condition::Let(cond) => cond.as_cairo(),
//             Condition::Expr(expr) => expr.as_cairo(),
//         }
//     }
// }

// impl AsCairo for Vec<Condition> {
//     fn as_cairo(&self) -> String {
//         self.as_cairo_delimited(" && ")
//     }
// }

// impl AsCairo for LetCondition {
//     fn as_cairo(&self) -> String {
//         let patterns = self.patterns.as_cairo();
//         let expr = self.expr.as_cairo();
//         format!("let {patterns} = {expr}")
//     }
// }

// impl AsCairo for Vec<Pattern> {
//     fn as_cairo(&self) -> String {
//         self.as_cairo_delimited(" | ")
//     }
// }
