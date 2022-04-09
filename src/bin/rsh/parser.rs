
use pest::{self,Parser};
use crate::ast::{Node};

#[derive(pest_derive::Parser)]
#[grammar = "bin/rsh/grammar/grammar.pest"]
struct PestParser;

// pub fn parse_line(source: &str) -> std::result::Result<Vec<Node>, pest::error::Error<Rule>> {
//     let pairs = PestParser::parse(Rule::Line, source);
//     for pair in pairs
// }

pub fn parse_expression(source: &str) -> std::result::Result<Vec<Node>, pest::error::Error<Rule>> {
    let mut ast = vec![];
    let pairs = PestParser::parse(Rule::Expression, source)?;
    for pair in pairs {
        if let Rule::Expression = pair.as_rule() {
            ast.push(ast_from_pair(pair));
        }
    }
    Ok(ast)
}

fn ast_from_pair(pair: pest::iterators::Pair<Rule>) -> Node {

    match pair.as_rule() {
        Rule::Line => {},
        Rule::Expression => {},
        Rule::Text => {},
        Rule::VarDef => {},
    };

}
