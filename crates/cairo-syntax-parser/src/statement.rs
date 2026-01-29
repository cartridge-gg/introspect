use crate::{
    Attribute, Expr, ExprPath, Identifier, Item, Modifier, from_typed_syntax_node, syntax_enum,
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
