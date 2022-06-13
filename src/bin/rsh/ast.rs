
pub enum Node {
    Expression(Expression),
    Expansion(Expansion),
    Operator(Operator),
    VarDef(VarDef),
    Text(Text),
    Error,
}

pub enum Expression {

    Composite {
        var_defs: Box<Vec<VarDef>>,
        text: Box<Vec<Text>>,
        op: Operator,
        next: Box<Option<Expression>>,
    },

    Single {
        var_defs: Box<Vec<VarDef>>,
        text: Box<Vec<Text>>,
    },

}

pub enum Expansion {
    TextVar,
    PlainVar,
    CmdVar,
    CmdQuote,
}

pub enum Operator {
    Semicolon,
    And,
    Fork,
    Or,
    Pipe,
    RedirectOut,
    RedirectAppend,
    RedirectIn,
    RedirectFd,
    HereDoc,
    HereStr,
}

pub struct VarDef {
    identifier: String,
    value: String,
}

pub enum Text {

    Expansion {
        kind: Expansion,
        value: String,
    },

    RawText {
        value: String,
    },

    SingleString {
        value: String,
    },

    DoubleString {
        value: String,
    },
}
