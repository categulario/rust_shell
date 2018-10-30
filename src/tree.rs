use parser::TokenType;
use std::iter::Peekable;

#[derive(Debug,PartialEq)]
pub enum GrammarError {
    InvalidCmdStart,
}

trait FromTokens<T> {
    fn from_tokens<'a, U: Iterator<Item = &'a TokenType>>(tokens: &mut Peekable<U>) -> Result<T, GrammarError>;
}

#[derive(Debug,PartialEq)]
pub enum CallExprOptions {
    ProgCall(TokenType, Vec<TokenType>),
    Parenthesis(Box<Expr>),
    Empty,
}

#[derive(Debug,PartialEq)]
pub struct CallExpr {
    value: CallExprOptions,
}

impl FromTokens<CallExpr> for CallExpr {
    fn from_tokens<'a, U: Iterator<Item = &'a TokenType>>(tokens: &mut Peekable<U>) -> Result<CallExpr, GrammarError> {
        match tokens.peek() {
            Some(&TokenType::Word(s)) => {
                tokens.next();

                let mut args = Vec::new();

                loop {
                    match tokens.peek() {
                        Some(&TokenType::Word(s)) => {
                            tokens.next();
                            args.push(TokenType::Word(s.clone()));
                        }
                        Some(&TokenType::DoubleQuotedString(s)) => {
                            tokens.next();
                            args.push(TokenType::DoubleQuotedString(s.clone()));
                        }
                        Some(&TokenType::SingleQuotedString(s)) => {
                            tokens.next();
                            args.push(TokenType::SingleQuotedString(s.clone()));
                        }
                        _ => {
                            break;
                        }
                    }
                }

                Ok(CallExpr{
                    value: CallExprOptions::ProgCall(TokenType::Word(s.clone()), args)
                })
            }
            None => {
                Ok(CallExpr{
                    value: CallExprOptions::Empty
                })
            }
            _ => {
                Err(GrammarError::InvalidCmdStart)
            }
        }
    }
}

#[derive(Debug,PartialEq)]
pub enum OrExprOptions {
    SingleExpr(CallExpr),
    Or(CallExpr, Box<OrExpr>),
}

#[derive(Debug,PartialEq)]
pub struct OrExpr {
    value: OrExprOptions,
}

impl FromTokens<OrExpr> for OrExpr {
    fn from_tokens<'a, U: Iterator<Item = &'a TokenType>>(tokens: &mut Peekable<U>) -> Result<OrExpr, GrammarError> {
        return Ok(OrExpr{
            value: OrExprOptions::SingleExpr(CallExpr::from_tokens(tokens)?),
        });
    }
}

#[derive(Debug,PartialEq)]
pub enum AndExprOptions {
    SingleExpr(OrExpr),
    And(OrExpr, Box<AndExpr>),
}

#[derive(Debug,PartialEq)]
pub struct AndExpr {
    value: AndExprOptions,
}

impl FromTokens<AndExpr> for AndExpr {
    fn from_tokens<'a, U: Iterator<Item = &'a TokenType>>(tokens: &mut Peekable<U>) -> Result<AndExpr, GrammarError> {
        let or_expr = OrExpr::from_tokens(tokens)?;

        match tokens.peek() {
            Some(&TokenType::And) => {
                tokens.next();

                Ok(AndExpr{
                    value: AndExprOptions::And(or_expr, Box::new(AndExpr::from_tokens(tokens)?)),
                })
            }
            _ => {
                Ok(AndExpr{
                    value: AndExprOptions::SingleExpr(or_expr),
                })
            }
        }
    }
}

#[derive(Debug,PartialEq)]
pub enum SemicolonExprOptions {
    SingleExpr(AndExpr),
    Semicolon(AndExpr, Box<SemicolonExpr>),
}

#[derive(Debug,PartialEq)]
pub struct SemicolonExpr {
    value: SemicolonExprOptions,
}

impl FromTokens<SemicolonExpr> for SemicolonExpr {
    fn from_tokens<'a, U: Iterator<Item = &'a TokenType>>(tokens: &mut Peekable<U>) -> Result<SemicolonExpr, GrammarError> {
        return Ok(SemicolonExpr{
            value: SemicolonExprOptions::SingleExpr(AndExpr::from_tokens(tokens)?),
        });
    }
}

#[derive(Debug,PartialEq)]
pub struct Expr {
    value: SemicolonExpr,
}

impl FromTokens<Expr> for Expr {
    fn from_tokens<'a, U: Iterator<Item = &'a TokenType>>(tokens: &mut Peekable<U>) -> Result<Expr, GrammarError> {
        return Ok(Expr{
            value: SemicolonExpr::from_tokens(tokens)?,
        });
    }
}

#[test]
fn test_simple_tree() {
    let tokens = [
        TokenType::Word("echo".to_string()), TokenType::DoubleQuotedString("\"foo\"".to_string())
    ];

    let mut it = tokens.iter().peekable();

    assert_eq!(Expr::from_tokens(&mut it).unwrap(), Expr{
        value: SemicolonExpr{
            value: SemicolonExprOptions::SingleExpr(AndExpr{
                value: AndExprOptions::SingleExpr(OrExpr{
                    value: OrExprOptions::SingleExpr(CallExpr{
                        value: CallExprOptions::ProgCall(
                            TokenType::Word("echo".to_string()),
                            vec![TokenType::DoubleQuotedString("\"foo\"".to_string())]
                        ),
                    }),
                }),
            }),
        },
    });
}

#[test]
fn test_and() {
    let tokens = [
        TokenType::Word("ls".to_string()), TokenType::And, TokenType::Word("ls".to_string())
    ];

    let mut it = tokens.iter().peekable();

    assert_eq!(Expr::from_tokens(&mut it).unwrap(), Expr{
        value: SemicolonExpr{
            value: SemicolonExprOptions::SingleExpr(AndExpr{
                value: AndExprOptions::And(OrExpr{
                    value: OrExprOptions::SingleExpr(CallExpr{
                        value: CallExprOptions::ProgCall(
                            TokenType::Word("ls".to_string()), vec![]
                        ),
                    }),
                }, Box::new(AndExpr{
                    value: AndExprOptions::SingleExpr(OrExpr{
                        value: OrExprOptions::SingleExpr(CallExpr{
                            value: CallExprOptions::ProgCall(
                                TokenType::Word("ls".to_string()), vec![]
                            ),
                        })
                    }),
                })),
            }),
        },
    });
}
