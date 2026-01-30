use crate::{
    Attribute, Visibility, from_typed_syntax_node, syntax_enum, syntax_option,
    syntax_terminal_bool, syntax_terminal_enum, syntax_type,
    typed_syntax_node_to_string_without_trivia, vec_from_element_list,
};

syntax_type! { MacroDeclarationItem[ItemMacroDeclaration]{
        attributes: Vec<Attribute>,
        visibility: Visibility,
        name: String,
        rules: Vec<MacroRule>,
}}

syntax_type! {
    MacroRule{
        lhs: WrappedMacro,
        rhs: Vec<MacroElement>
    }
}

syntax_type! {
    MacroParam{
        name: String,
        kind: Option<MacroParamKind>,
    }
}

syntax_type! {
    MacroRepetition{
        elements: Vec<MacroElement>,
        comma[separator]: bool,
        operator: MacroRepetitionOperator,
    }
}

vec_from_element_list! {ParenthesizedMacro.elements, MacroElement}
vec_from_element_list! {BracedMacro.elements, MacroElement}
vec_from_element_list! {BracketedMacro.elements, MacroElement}
vec_from_element_list! {MacroRulesList, MacroRule}
vec_from_element_list! {MacroElements, MacroElement}

syntax_option! { OptionParamKind {ParamKind: MacroParamKind} }

syntax_terminal_bool! {Comma}

from_typed_syntax_node! {ParamKind.kind, MacroParamKind}
from_typed_syntax_node! {TokenTreeLeaf.leaf, TokenNode}
from_typed_syntax_node! {MacroWrapper.subtree, WrappedMacro}

typed_syntax_node_to_string_without_trivia! {ParamIdent.ident}
typed_syntax_node_to_string_without_trivia! {ParamExpr.expr}

syntax_enum! {
    WrappedMacro{
        Parenthesized(Vec<MacroElement>),
        Braced(Vec<MacroElement>),
        Bracketed(Vec<MacroElement>),
    }
}

syntax_enum! {
    MacroElement{
        Token(TokenNode),
        Param(MacroParam),
        Subtree(WrappedMacro),
        Repetition(MacroRepetition),
    }
}

syntax_enum! {
    MacroParamKind{
        Identifier(String),
        Expr(String),
        Missing,
    }
}

syntax_terminal_enum! {
    MacroRepetitionOperator{
        ZeroOrOne[TerminalQuestionMark],
        ZeroOrMore[TerminalMul],
        OneOrMore[TerminalPlus],
        Missing[MacroRepetitionOperatorMissing],
    }
}

syntax_enum! {
    TokenNode{
        Identifier[TerminalIdentifier](String),
        LiteralNumber[TerminalLiteralNumber](String),
        ShortString[TerminalShortString](String),
        String[TerminalString](String),
        As[TerminalAs],
        Const[TerminalConst],
        Else[TerminalElse],
        Enum[TerminalEnum],
        Extern[TerminalExtern],
        False[TerminalFalse],
        Function[TerminalFunction],
        If[TerminalIf],
        While[TerminalWhile],
        For[TerminalFor],
        Loop[TerminalLoop],
        Impl[TerminalImpl],
        Implicits[TerminalImplicits],
        Let[TerminalLet],
        Macro[TerminalMacro],
        Match[TerminalMatch],
        Module[TerminalModule],
        Mut[TerminalMut],
        NoPanic[TerminalNoPanic],
        Of[TerminalOf],
        Ref[TerminalRef],
        Continue[TerminalContinue],
        Return[TerminalReturn],
        Break[TerminalBreak],
        Struct[TerminalStruct],
        Trait[TerminalTrait],
        True[TerminalTrue],
        Type[TerminalType],
        Use[TerminalUse],
        Pub[TerminalPub],
        And[TerminalAnd],
        AndAnd[TerminalAndAnd],
        Arrow[TerminalArrow],
        At[TerminalAt],
        BadCharacters[TerminalBadCharacters],
        Colon[TerminalColon],
        ColonColon[TerminalColonColon],
        Comma[TerminalComma],
        Div[TerminalDiv],
        DivEq[TerminalDivEq],
        Dollar[TerminalDollar],
        Dot[TerminalDot],
        DotDot[TerminalDotDot],
        DotDotEq[TerminalDotDotEq],
        EndOfFile[TerminalEndOfFile],
        Eq[TerminalEq],
        EqEq[TerminalEqEq],
        GE[TerminalGE],
        GT[TerminalGT],
        Hash[TerminalHash],
        LBrace[TerminalLBrace],
        LBrack[TerminalLBrack],
        LE[TerminalLE],
        LParen[TerminalLParen],
        LT[TerminalLT],
        MatchArrow[TerminalMatchArrow],
        Minus[TerminalMinus],
        MinusEq[TerminalMinusEq],
        Mod[TerminalMod],
        ModEq[TerminalModEq],
        Mul[TerminalMul],
        MulEq[TerminalMulEq],
        Neq[TerminalNeq],
        Not[TerminalNot],
        BitNot[TerminalBitNot],
        Or[TerminalOr],
        OrOr[TerminalOrOr],
        Plus[TerminalPlus],
        PlusEq[TerminalPlusEq],
        QuestionMark[TerminalQuestionMark],
        RBrace[TerminalRBrace],
        RBrack[TerminalRBrack],
        RParen[TerminalRParen],
        Semicolon[TerminalSemicolon],
        Underscore[TerminalUnderscore],
        Xor[TerminalXor],
        Empty[TerminalEmpty],
    }
}
