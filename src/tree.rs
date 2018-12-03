use parser::TokenType;
use std::iter::Peekable;

#[derive(Debug,PartialEq)]
pub enum GrammarError {
    InvalidCmdStart,
    MismatchedParenthesis,
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
            Some(&TokenType::Parenthesis('(')) => {
                tokens.next();

                let inner_expr = Expr::from_tokens(tokens)?;

                match tokens.peek() {
                    Some(&TokenType::Parenthesis(')')) => {
                        Ok(CallExpr{
                            value: CallExprOptions::Parenthesis(Box::new(inner_expr)),
                        })
                    }
                    _ => {
                        Err(GrammarError::MismatchedParenthesis)
                    }
                }
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
        let call_expr = CallExpr::from_tokens(tokens)?;

        match tokens.peek() {
            Some(&TokenType::Or) => {
                tokens.next();

                Ok(OrExpr{
                    value: OrExprOptions::Or(call_expr, Box::new(OrExpr::from_tokens(tokens)?)),
                })
            }
            _ => {
                Ok(OrExpr{
                    value: OrExprOptions::SingleExpr(call_expr),
                })
            }
        }
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
        let and_expr = AndExpr::from_tokens(tokens)?;

        match tokens.peek() {
            Some(&TokenType::Semicolon) => {
                tokens.next();

                Ok(SemicolonExpr{
                    value: SemicolonExprOptions::Semicolon(and_expr, Box::new(SemicolonExpr::from_tokens(tokens)?)),
                })
            }
            _ => {
                Ok(SemicolonExpr{
                    value: SemicolonExprOptions::SingleExpr(and_expr),
                })
            }
        }
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

#[test]
fn test_or() {
    let tokens = [
        TokenType::Word("ls".to_string()), TokenType::Or, TokenType::Word("ls".to_string())
    ];

    let mut it = tokens.iter().peekable();

    assert_eq!(Expr::from_tokens(&mut it).unwrap(), Expr{
        value: SemicolonExpr{
            value: SemicolonExprOptions::SingleExpr(AndExpr{
                value: AndExprOptions::SingleExpr(OrExpr{
                    value: OrExprOptions::Or(CallExpr{
                        value: CallExprOptions::ProgCall(
                            TokenType::Word("ls".to_string()), vec![]
                        ),
                    }, Box::new(OrExpr{
                        value: OrExprOptions::SingleExpr(CallExpr{
                            value: CallExprOptions::ProgCall(
                                TokenType::Word("ls".to_string()), vec![]
                            ),
                        }),
                    })),
                }),
            }),
        },
    });
}

#[test]
fn test_semicolon() {
    let tokens = [
        TokenType::Word("ls".to_string()), TokenType::Semicolon, TokenType::Word("ls".to_string())
    ];

    let mut it = tokens.iter().peekable();

    assert_eq!(Expr::from_tokens(&mut it).unwrap(), Expr{
        value: SemicolonExpr{
            value: SemicolonExprOptions::Semicolon(AndExpr{
                value: AndExprOptions::SingleExpr(OrExpr{
                    value: OrExprOptions::SingleExpr(CallExpr{
                        value: CallExprOptions::ProgCall(
                            TokenType::Word("ls".to_string()), vec![]
                        ),
                    }),
                }),
            }, Box::new(SemicolonExpr{
                value: SemicolonExprOptions::SingleExpr(AndExpr{
                    value: AndExprOptions::SingleExpr(OrExpr{
                        value: OrExprOptions::SingleExpr(CallExpr{
                            value: CallExprOptions::ProgCall(
                                TokenType::Word("ls".to_string()), vec![]
                            ),
                        }),
                    }),
                }),
            })),
        },
    });
}

#[test]
fn test_parenthesis() {
    let tokens = [
        TokenType::Word("ls".to_string()),
        TokenType::And,
        TokenType::Parenthesis('('),
        TokenType::Word("ls".to_string()),
        TokenType::Parenthesis(')'),
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
                            value: CallExprOptions::Parenthesis(Box::new(Expr{
                                value: SemicolonExpr{
                                    value: SemicolonExprOptions::SingleExpr(AndExpr{
                                        value: AndExprOptions::SingleExpr(OrExpr{
                                            value: OrExprOptions::SingleExpr(CallExpr{
                                                value: CallExprOptions::ProgCall(
                                                    TokenType::Word("ls".to_string()),
                                                    vec![]
                                                ),
                                            }),
                                        }),
                                    }),
                                },
                            })),
                        })
                    }),
                })),
            }),
        },
    });
}

#[test]
fn test_parenthesis_mismatched() {
    let tokens = [
        TokenType::Word("ls".to_string()),
        TokenType::And,
        TokenType::Parenthesis('('),
        TokenType::Word("ls".to_string()),
    ];

    let mut it = tokens.iter().peekable();

    assert_eq!(Expr::from_tokens(&mut it).unwrap_err(), GrammarError::MismatchedParenthesis);
}
