use crate::parser::Token;

use super::AtomNode;

pub enum FlatNode {
    Symbol(String),
    Atom(AtomNode),
}

pub fn parse(tokens: Vec<Token>) -> Vec<FlatNode> {
    let mut nodes = Vec::with_capacity(tokens.len());
    for token in tokens {
        let node = match token {
            Token::Bool(b) => FlatNode::Atom(AtomNode::Bool(b)),
            Token::Int(i) => FlatNode::Atom(AtomNode::Int(i)),
            Token::Float(f) => FlatNode::Atom(AtomNode::Float(f)),
            Token::Symbol(symbol) => FlatNode::Symbol(symbol),
            Token::Letter(l) => FlatNode::Atom(AtomNode::Letter(l)),
            Token::String(s) => FlatNode::Atom(AtomNode::String(s)),
            Token::Bytes(b) => FlatNode::Atom(AtomNode::Bytes(b)),
            Token::Delimeter => continue,
        };
        nodes.push(node);
    }
    nodes
}
