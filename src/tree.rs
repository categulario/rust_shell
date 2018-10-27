use parser::TokenType;

#[derive(Debug,PartialEq)]
pub enum GrammarError {
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

#[derive(Debug,PartialEq)]
pub enum OrExprOptions {
    SingleExpr(CallExpr),
    Or(CallExpr, Box<OrExpr>),
}

#[derive(Debug,PartialEq)]
pub struct OrExpr {
    value: OrExprOptions,
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

#[derive(Debug,PartialEq)]
pub enum SemicolonExprOptions {
    SingleExpr(AndExpr),
    Semicolon(AndExpr, Box<SemicolonExpr>),
}

#[derive(Debug,PartialEq)]
pub struct SemicolonExpr {
    value: SemicolonExprOptions,
}

#[derive(Debug,PartialEq)]
pub struct Expr {
    value: SemicolonExpr,
}

impl Expr {
    fn from(tokens: Vec<TokenType>) -> Result<Expr, GrammarError> {
        return Ok(Expr{});
    }
}

#[test]
fn test_simple_tree() {
    let tokens = vec![
        TokenType::Word("echo".to_string()), TokenType::DoubleQuotedString("\"foo\"".to_string())
    ];

    assert_eq!(Expr::from(tokens).unwrap(), Expr{
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
