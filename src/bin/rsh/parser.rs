
use pest::{self,Parser};
use crate::ast;
use crate::ast::Node;

#[derive(pest_derive::Parser)]
#[grammar = "bin/rsh/grammar/grammar.pest"]
struct RshParser;

pub enum ParserError {
    PestError(pest::error::Error<Rule>),
    AstError(Node),
    MiscError(String),
}

pub fn parse_command(source: &str) -> std::result::Result<Node, ParserError> {
    match RshParser::parse(Rule::Line, source) {
        Ok(mut pairs) => {
            Ok(ast_from_pair(pairs
                .find(|x| x.as_rule() == Rule::Line)
                .unwrap()
                .into_inner()
                .find(|x| x.as_rule() == Rule::Expression)
                .unwrap()
            ))
        },
        Err(e) => {
            Err(ParserError::PestError(e))
        }
    }
}

fn ast_from_pair(pair: pest::iterators::Pair<Rule>) -> Node {
    match pair.as_rule() {



        Rule::Line => {
            match pair.into_inner().find(|x| x.as_rule() == Rule::Expression) {
                Some(expr) => {
                    return ast_from_pair(expr);
                },
                None => return Node::Error,
            }
        },



        Rule::Expression => {
            let mut var_defs = Vec::<ast::VarDef>::new();
            let mut texts = Vec::<ast::Text>::new();
            let mut operator: Option<ast::Operator> = None;
            let mut expression: Option<ast::Expression> = None;
            for pair in pair.into_inner() {
                match ast_from_pair(pair) {
                    Node::VarDef(var_def) => var_defs.push(var_def),
                    Node::Text(text) => texts.push(text),
                    Node::Operator(op) => operator = Some(op),
                    Node::Expression(expr) => expression = Some(expr),
                    Node::Error => return Node::Error,
                    _ => continue,
                };
            }
            if texts.is_empty() {
                return Node::Error;
            }
            match operator {
                Some(operator) => {
                    return Node::Expression(ast::Expression::Composite {
                        var_defs: Box::new(var_defs),
                        text: Box::new(texts),
                        op: operator,
                        next: Box::new(expression)
                    });
                },
                None => {
                    return Node::Expression(ast::Expression::Single {
                        var_defs: Box::new(var_defs),
                        text: Box::new(texts)
                    })
                }
            }
        },



        Rule::Operator => {
            return match pair.as_str() {
                ";"   => Node::Operator(ast::Operator::Semicolon),
                "&&"  => Node::Operator(ast::Operator::And),
                "&"   => Node::Operator(ast::Operator::Fork),
                "||"  => Node::Operator(ast::Operator::Or),
                "|"   => Node::Operator(ast::Operator::Pipe),
                ">>"  => Node::Operator(ast::Operator::RedirectAppend),
                ">&"  => Node::Operator(ast::Operator::RedirectFd),
                ">"   => Node::Operator(ast::Operator::RedirectOut),
                "<<<" => Node::Operator(ast::Operator::HereStr),
                "<<"  => Node::Operator(ast::Operator::HereDoc),
                "<"   => Node::Operator(ast::Operator::RedirectIn),
                _     => Node::Error
            };
        },



        Rule::VarDef => {



        },



        Rule::Text => {



        },



        Rule::Expansion => {



        }



        _ => return Node::Error



    };
    Node::Error
}
