use token::TokenInfo;
use token::Token;

// 文法.
//   <Expr> ::= <Term> <AddSubExpr>
//   <AddSubExpr> ::= ['+'|'-'] <Term> <AddSubExpr>
//   <Term> ::= <Factor> <SubTerm>
//   <MultiDivTerm> ::= ['*'|'.'] <Factor> <MultiDivTerm>
//   <Factor> ::= '(' NUMBER ')'

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Plus(Box<Expr>, Box<Expr>),
    Minus(Box<Expr>, Box<Expr>),
    Multiple(Box<Expr>, Box<Expr>),
    Division(Box<Expr>, Box<Expr>),
    Factor(i64),
}

#[derive(Debug,Clone)]
pub struct Ast<'a> {
    tokens: &'a Vec<TokenInfo>,     // トークン配列.
    current_pos: usize,             // 現在読み取り位置.
}

// 抽象構文木をトークン列から作成する
impl<'a> Ast<'a> {
    // コンストラクタ.
    pub fn new (tokens: &Vec<TokenInfo>) -> Ast {
        Ast { current_pos: 0, tokens: tokens }
    }

    // トークン列を受け取り、抽象構文木を返す.
    pub fn parse(&mut self) -> Expr {
        self.expr(None)
    }

    // expression
    fn expr(&mut self, acc: Option<Expr>) -> Expr {
        let factor = self.term(acc);
        self.expr_add_sub(factor)
    }

    // add or sub expression.
    fn expr_add_sub(&mut self, acc: Expr) -> Expr {
        let ope = self.next();
        match ope.get_token_type() {
            Token::Plus | Token::Minus => {
                self.consume();
                let right = self.term(None);
                let tree = match ope.get_token_type() {
                    Token::Plus => self.plus(acc, right),
                    _ => self.minus(acc, right)
                };
                self.expr_add_sub(tree)
            }
            _ => self.term(Some(acc))
        }
    }

    // term.
    fn term(&mut self, acc: Option<Expr>) -> Expr {
        let factor = self.factor(acc);
        self.term_multi_div(factor)
    }

    // multiple and division term.
    fn term_multi_div(&mut self, acc: Expr) -> Expr {
        let ope = self.next();
        match ope.get_token_type() {
            Token::Multi | Token::Division => {
                self.consume();
                let right = self.factor(None);
                let tree = match ope.get_token_type() {
                    Token::Multi => self.multiple(acc, right),
                    _ => self.division(acc, right)
                };
                self.term_multi_div(tree)
            }
            _ => self.factor(Some(acc))
        }
    }

    // factor.
    fn factor(&mut self, acc: Option<Expr>) -> Expr {
        let token = self.next();
        match token.get_token_type() {
            Token::Number => {
                self.consume();
                self.number(token)
            }
            Token::LeftBracket => {
                self.consume();
                let factor = self.factor(acc);
                self.expr(Some(factor))
            }
            Token::RightBracket => {
                self.consume();
                acc.unwrap()
            },
            _ => acc.unwrap()
        }
    }

    // plus.
    fn plus(&mut self, left: Expr, right: Expr) -> Expr {
        Expr::Plus(Box::new(left), Box::new(right))
    }

    // minus.
    fn minus(&mut self, left: Expr, right: Expr) -> Expr {
        Expr::Minus(Box::new(left), Box::new(right))
    }

    // multipler.
    fn multiple(&mut self, left: Expr, right: Expr) -> Expr {
       Expr::Multiple(Box::new(left), Box::new(right))
    }

    // division.
    fn division(&self, left: Expr, right: Expr) -> Expr {
        Expr::Division(Box::new(left), Box::new(right))
    }

    // number
    fn number(&mut self, token: TokenInfo ) -> Expr {
        Expr::Factor(token.get_token_value().parse::<i64>().unwrap())
    }

    // トークン読み取り.
    fn next(&mut self) -> TokenInfo {
        if self.current_pos >= self.tokens.len() {
            return TokenInfo::new(Token::Unknown, "".to_string());
        }
        self.tokens[self.current_pos].clone()
    }

    // 読み取り位置更新.
    fn next_consume(&mut self) -> TokenInfo {
        if self.current_pos >= self.tokens.len() {
            return TokenInfo::new(Token::Unknown, "".to_string());
        }
        let token = self.tokens[self.current_pos].clone();
        self.current_pos = self.current_pos + 1;
        token
    }

    // 読み取り位置更新.
    fn consume(&mut self) {
        self.current_pos = self.current_pos + 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_operator() {
        // 単純な加算テスト.
        {
            let data =
                vec![
                    TokenInfo::new(Token::Number, "1".to_string()),
                    TokenInfo::new(Token::Plus, '+'.to_string()),
                    TokenInfo::new(Token::Number, "2".to_string())
                ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::Plus(
                    Box::new(Expr::Factor(1)),
                    Box::new(Expr::Factor(2))
                )
            )
        }
        // 複数の加算テスト.
        {
            let data =
                vec![
                    TokenInfo::new(Token::Number, '1'.to_string()),
                    TokenInfo::new(Token::Plus, '+'.to_string()),
                    TokenInfo::new(Token::Number, '2'.to_string()),
                    TokenInfo::new(Token::Plus, '+'.to_string()),
                    TokenInfo::new(Token::Number, '3'.to_string())
                ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::Plus(
                    Box::new(Expr::Plus(
                        Box::new(Expr::Factor(1)),
                        Box::new(Expr::Factor(2))
                    )),
                    Box::new(Expr::Factor(3))
                )
            )
        }
        // 複数の加算テスト.
        {
            let data =
                vec![
                    TokenInfo::new(Token::Number, '1'.to_string()),
                    TokenInfo::new(Token::Plus, '+'.to_string()),
                    TokenInfo::new(Token::Number, '2'.to_string()),
                    TokenInfo::new(Token::Plus, '+'.to_string()),
                    TokenInfo::new(Token::Number, '3'.to_string()),
                    TokenInfo::new(Token::Plus, '+'.to_string()),
                    TokenInfo::new(Token::Number, '4'.to_string())
                ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::Plus(
                    Box::new(Expr::Plus(
                        Box::new(Expr::Plus(
                            Box::new(Expr::Factor(1)),
                            Box::new(Expr::Factor(2))
                        )),
                        Box::new(Expr::Factor(3)),
                    )),
                    Box::new(Expr::Factor(4)),
                )
            )
        }
    }

    #[test]
    fn test_sub_operator() {
        // 単純な減算テスト.
        {
            let data =
                vec![
                    TokenInfo::new(Token::Number, "1".to_string()),
                    TokenInfo::new(Token::Minus, '-'.to_string()),
                    TokenInfo::new(Token::Number, "2".to_string())
                ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::Minus(
                    Box::new(Expr::Factor(1)),
                    Box::new(Expr::Factor(2))
                )
            )
        }
        // 複数の減算テスト.
        {
            let data =
                vec![
                    TokenInfo::new(Token::Number, "100".to_string()),
                    TokenInfo::new(Token::Minus, '-'.to_string()),
                    TokenInfo::new(Token::Number, '2'.to_string()),
                    TokenInfo::new(Token::Minus, '-'.to_string()),
                    TokenInfo::new(Token::Number, '3'.to_string())
                ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::Minus(
                    Box::new(Expr::Minus(
                        Box::new(Expr::Factor(100)),
                        Box::new(Expr::Factor(2))
                    )),
                    Box::new(Expr::Factor(3))
                )
            )
        }
        // 複数の減算テスト.
        {
            let data =
                vec![
                    TokenInfo::new(Token::Number, '1'.to_string()),
                    TokenInfo::new(Token::Minus, '-'.to_string()),
                    TokenInfo::new(Token::Number, '2'.to_string()),
                    TokenInfo::new(Token::Minus, '-'.to_string()),
                    TokenInfo::new(Token::Number, '3'.to_string()),
                    TokenInfo::new(Token::Minus, '-'.to_string()),
                    TokenInfo::new(Token::Number, '4'.to_string())
                ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::Minus(
                    Box::new(Expr::Minus(
                        Box::new(Expr::Minus(
                            Box::new(Expr::Factor(1)),
                            Box::new(Expr::Factor(2))
                        )),
                        Box::new(Expr::Factor(3)),
                    )),
                    Box::new(Expr::Factor(4)),
                )
            )
        }
    }

    #[test]
    fn test_mul_operator() {
        // 単純な乗算テスト.
        {
            let data =
                vec![
                    TokenInfo::new(Token::Number, "1".to_string()),
                    TokenInfo::new(Token::Multi, '*'.to_string()),
                    TokenInfo::new(Token::Number, "2".to_string())
                ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::Multiple(
                    Box::new(Expr::Factor(1)),
                    Box::new(Expr::Factor(2))
                )
            )
        }
        // 複数の減算テスト.
        {
            let data =
                vec![
                    TokenInfo::new(Token::Number, '1'.to_string()),
                    TokenInfo::new(Token::Multi, '*'.to_string()),
                    TokenInfo::new(Token::Number, '2'.to_string()),
                    TokenInfo::new(Token::Multi, '*'.to_string()),
                    TokenInfo::new(Token::Number, '3'.to_string())
                ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::Multiple(
                    Box::new(Expr::Multiple(
                        Box::new(Expr::Factor(1)),
                        Box::new(Expr::Factor(2))
                    )),
                    Box::new(Expr::Factor(3))
                )
            )
        }
        // 複数の減算テスト.
        {
            let data =
                vec![
                    TokenInfo::new(Token::Number, '1'.to_string()),
                    TokenInfo::new(Token::Multi, '*'.to_string()),
                    TokenInfo::new(Token::Number, '2'.to_string()),
                    TokenInfo::new(Token::Multi, '*'.to_string()),
                    TokenInfo::new(Token::Number, '3'.to_string()),
                    TokenInfo::new(Token::Multi, '*'.to_string()),
                    TokenInfo::new(Token::Number, '4'.to_string())
                ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::Multiple(
                    Box::new(Expr::Multiple(
                        Box::new(Expr::Multiple(
                            Box::new(Expr::Factor(1)),
                            Box::new(Expr::Factor(2))
                        )),
                        Box::new(Expr::Factor(3))
                    )),
                    Box::new(Expr::Factor(4))
                )
            )
        }
    }

    #[test]
    fn test_div_operator() {
        // 単純な乗算テスト.
        {
            let data =
                vec![
                    TokenInfo::new(Token::Number, "1".to_string()),
                    TokenInfo::new(Token::Division, '/'.to_string()),
                    TokenInfo::new(Token::Number, "2".to_string())
                ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::Division(
                    Box::new(Expr::Factor(1)),
                    Box::new(Expr::Factor(2))
                )
            )
        }
        // 複数の減算テスト.
        {
            let data =
                vec![
                    TokenInfo::new(Token::Number, '1'.to_string()),
                    TokenInfo::new(Token::Division, '/'.to_string()),
                    TokenInfo::new(Token::Number, '2'.to_string()),
                    TokenInfo::new(Token::Division, '/'.to_string()),
                    TokenInfo::new(Token::Number, '3'.to_string())
                ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::Division(
                    Box::new(Expr::Division(
                        Box::new(Expr::Factor(1)),
                        Box::new(Expr::Factor(2))
                    )),
                    Box::new(Expr::Factor(3))
                )
            )
        }
        // 複数の減算テスト.
        {
            let data =
                vec![
                    TokenInfo::new(Token::Number, '1'.to_string()),
                    TokenInfo::new(Token::Division, '/'.to_string()),
                    TokenInfo::new(Token::Number, '2'.to_string()),
                    TokenInfo::new(Token::Division, '/'.to_string()),
                    TokenInfo::new(Token::Number, '3'.to_string()),
                    TokenInfo::new(Token::Division, '/'.to_string()),
                    TokenInfo::new(Token::Number, '4'.to_string())
                ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::Division(
                    Box::new(Expr::Division(
                        Box::new(Expr::Division(
                            Box::new(Expr::Factor(1)),
                            Box::new(Expr::Factor(2))
                        )),
                        Box::new(Expr::Factor(3))
                    )),
                    Box::new(Expr::Factor(4))
                )
            )
        }
    }

    #[test]
    fn test_mix_operator() {
        // 複数演算子のテスト.
        {
            let data =
                vec![
                    TokenInfo::new(Token::Number, '1'.to_string()),
                    TokenInfo::new(Token::Multi, '*'.to_string()),
                    TokenInfo::new(Token::Number, '2'.to_string()),
                    TokenInfo::new(Token::Plus, '+'.to_string()),
                    TokenInfo::new(Token::Number, '3'.to_string())
                ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::Plus(
                    Box::new(Expr::Multiple(
                        Box::new(Expr::Factor(1)),
                        Box::new(Expr::Factor(2))
                    )),
                    Box::new(Expr::Factor(3)),
                )
            )
        }
        // 複数演算子のテスト.
        {
            let data =
                vec![
                    TokenInfo::new(Token::Number, '1'.to_string()),
                    TokenInfo::new(Token::Plus, '+'.to_string()),
                    TokenInfo::new(Token::Number, '2'.to_string()),
                    TokenInfo::new(Token::Multi, '*'.to_string()),
                    TokenInfo::new(Token::Number, '3'.to_string())
                ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::Plus(
                    Box::new(Expr::Factor(1)),
                    Box::new(Expr::Multiple(
                        Box::new(Expr::Factor(2)),
                        Box::new(Expr::Factor(3))
                    ))
                )
            )
        }
        // 複数演算子のテスト.
        {
            let data =
                vec![
                    TokenInfo::new(Token::Number, '1'.to_string()),
                    TokenInfo::new(Token::Division, '/'.to_string()),
                    TokenInfo::new(Token::Number, '2'.to_string()),
                    TokenInfo::new(Token::Plus, '+'.to_string()),
                    TokenInfo::new(Token::Number, '3'.to_string())
                ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::Plus(
                    Box::new(Expr::Division(
                        Box::new(Expr::Factor(1)),
                        Box::new(Expr::Factor(2))
                    )),
                    Box::new(Expr::Factor(3)),
                )
            )
        }
        // 複数演算子のテスト.
        {
            let data =
                vec![
                    TokenInfo::new(Token::Number, '1'.to_string()),
                    TokenInfo::new(Token::Plus, '+'.to_string()),
                    TokenInfo::new(Token::Number, '2'.to_string()),
                    TokenInfo::new(Token::Division, '/'.to_string()),
                    TokenInfo::new(Token::Number, '3'.to_string())
                ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::Plus(
                    Box::new(Expr::Factor(1)),
                    Box::new(Expr::Division(
                        Box::new(Expr::Factor(2)),
                        Box::new(Expr::Factor(3))
                    ))
                )
            )
        }
    }

    #[test]
    fn test_bracket() {
        // カッコのテスト.
        {
            let data =
                vec![
                    TokenInfo::new(Token::LeftBracket, "(".to_string()),
                    TokenInfo::new(Token::Number, "1".to_string()),
                    TokenInfo::new(Token::Plus, '+'.to_string()),
                    TokenInfo::new(Token::Number, "2".to_string()),
                    TokenInfo::new(Token::RightBracket, ")".to_string()),
                ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::Plus(
                    Box::new(Expr::Factor(1)),
                    Box::new(Expr::Factor(2))
                )
            )
        }
        {
            let data =
                vec![
                    TokenInfo::new(Token::Number, "1".to_string()),
                    TokenInfo::new(Token::Plus, '+'.to_string()),
                    TokenInfo::new(Token::LeftBracket, "(".to_string()),
                    TokenInfo::new(Token::Number, "2".to_string()),
                    TokenInfo::new(Token::Plus, '+'.to_string()),
                    TokenInfo::new(Token::Number, "3".to_string()),
                    TokenInfo::new(Token::RightBracket, ")".to_string()),
                ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::Plus(
                    Box::new(Expr::Factor(1)),
                    Box::new(Expr::Plus(
                        Box::new(Expr::Factor(2)),
                        Box::new(Expr::Factor(3))
                    ))
                )
            )
        }
    }
}

