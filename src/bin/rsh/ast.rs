
pub enum Node {

    Line {
        expression: Box<Node>,
    },

    Expression {
        env_vars: Vec<Node>,
        text: Vec<Node>,
    },

    Text {
        value: String,
    },

    VarDef {
        identifier: String,
        value: String,
    }

}
