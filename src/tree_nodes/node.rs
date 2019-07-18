// use crate::tokenizer::tokenizer::Token;
// use logos::Lexer;
// use std::ops::Range;

// pub struct Node<'a> {
//     range: Range<usize>,
//     slice: &'a str,
//     expr: Expr,
// }

// impl<'a> Node<'a> {
//     fn new(lexer: &Lexer<Token, &'a str>, expr: Expr) -> Node<'a> {
//         Node {
//             range: lexer.range(),
//             slice: lexer.slice(),
//             expr,
//         }
//     }
// }
