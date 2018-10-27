use parser::TokenType;
use std::iter::Peekable;

#[derive(Debug,PartialEq)]
pub enum GrammarError {
    InvalidCmdStart,
}

trait FromTokens<T> {
    fn from_tokens<U: Iterator<Item = TokenType>>(tokens: &mut Peekable<U>) -> Result<T, GrammarError>;
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
    fn from_tokens<U: Iterator<Item = TokenType>>(tokens: &mut Peekable<U>) -> Result<CallExpr, GrammarError> {
        match tokens.peek() {
            Some(&prog_name@TokenType::Word(_)) => {
                Ok(CallExpr{
                    value: CallExprOptions::ProgCall(prog_name, vec![])
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
    fn from_tokens<U: Iterator<Item = TokenType>>(tokens: &mut Peekable<U>) -> Result<OrExpr, GrammarError> {
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
    fn from_tokens<U: Iterator<Item = TokenType>>(tokens: &mut Peekable<U>) -> Result<AndExpr, GrammarError> {
        return Ok(AndExpr{
            value: AndExprOptions::SingleExpr(OrExpr::from_tokens(tokens)?),
        });
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
    fn from_tokens<U: Iterator<Item = TokenType>>(tokens: &mut Peekable<U>) -> Result<SemicolonExpr, GrammarError> {
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
    fn from_tokens<U: Iterator<Item = TokenType>>(tokens: &mut Peekable<U>) -> Result<Expr, GrammarError> {
        return Ok(Expr{
            value: SemicolonExpr::from_tokens(&mut tokens)?,
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
