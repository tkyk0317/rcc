use token::TokenInfo;
use token::Token;

// 文法.
//   <Expr> ::= <Term> [ ['+' | '-']  <Term>]*
//   <Term> ::= <Factor> ['*' <Factor>]*
//   <Factor> ::= [NUMBER]* | <Expr>

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Plus(Box<Expr>, Box<Expr>),
    Minus(Box<Expr>, Box<Expr>),
    Multiple(Box<Expr>, Box<Expr>),
    Factor(i64)
}

#[derive(Debug,Clone)]
pub struct Ast<'a> {
    tokens: &'a Vec<TokenInfo>,     // トークン配列.
    current_pos: usize,             // 現在読み取り位置.
}

// AST実装.
//
impl<'a> Ast<'a> {
    // コンストラクタ.
    pub fn new (tokens: &Vec<TokenInfo>) -> Ast {
        Ast { current_pos: 0, tokens: tokens }
    }

    // トークン列を受け取り、抽象構文木を返す.
    pub fn parse(&mut self) -> Expr {
        self.expr()
    }

    // expression.
    fn expr(&mut self) -> Expr {
        // 各非終端記号ごとに処理を行う.
        let cur = self.next_consume();
        let ope = self.next();
        match ope.get_token_type() {
            Token::Plus | Token::Minus | Token::Multi => {
                let factor = self.factor(cur);
                self.expr_add_sub(factor)
            }
            _ => panic!("Not Support Token Type: {:?}", ope)
        }
    }

    // term.
    fn term(&mut self, cur: Expr) -> Expr {
        let ope = self.next_consume();
        match ope.get_token_type() {
            Token::Multi => {
                let right_token = self.next_consume();
                let right_factor = self.factor(right_token);
                let left_factor = self.multiple(cur, right_factor);

                // 次の演算子を確認.
                let next_ope = self.next();
                match next_ope.get_token_type() {
                    Token::Multi => self.term(left_factor),
                    // 次の演算子は乗算演算子以外.
                    _ => self.expr_add_sub(left_factor)
                }
            }
            _ => {
                self.back(1);
                cur
            }
        }
    }

    // plus/minus expression.
    fn expr_add_sub(&mut self, cur: Expr) -> Expr {
        let ope = self.next_consume();
        match ope.get_token_type() {
            Token::Plus | Token::Minus => {
                // 加減算演算子AST作成.
                let right_token = self.next_consume();
                let _right_factor = self.factor(right_token);
                let right_factor = self.term(_right_factor);
                let left_factor = match ope.get_token_type() {
                    Token::Plus => self.plus(cur, right_factor),
                    _ => self.minus(cur, right_factor)
                };

                // 次の演算子を確認.
                let next_ope = self.next();
                match next_ope.get_token_type() {
                    Token::Plus | Token::Minus => self.expr_add_sub(left_factor),
                    _ => self.term(left_factor)
                }
            }
            _ => {
                self.back(1);
                self.term(cur)
            }
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

    // factor.
    fn factor(&mut self, cur: TokenInfo) -> Expr {
        if Token::Number == cur.get_token_type() { Expr::Factor(cur.get_token_value().parse::<i64>().unwrap()) }
        else { self.expr() }
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

    // 読み取り位置巻き戻し.
    fn back(&mut self, i: usize) {
        self.current_pos = self.current_pos - i;
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
    }
}

