use crate::{Parser, SyntaxKind::*, SyntaxKindSet, T};
use expressions::*;
use statements::*;

mod expressions;
mod statements;

pub(crate) fn module(p: &mut Parser) {
    let m = p.start();
    while !p.at(EOF) {
        statement(p);
    }
    m.complete(p, MODULE);
}