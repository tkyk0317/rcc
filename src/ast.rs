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
        // 文法に従いながら解析を行う.
        let token = self.next_consume();
        match token.get_token_type() {
            Token::Number => self.expr(token),
            _ => panic!("not support token type {:?}", token)
        }
    }

    // expression.
    fn expr(&mut self, cur: TokenInfo) -> Expr {
        // 各非終端記号ごとに処理を行う.
        let ope = self.next();
        match ope.get_token_type() {
            Token::Plus | Token::Minus | Token::Multi => self.expr_add_sub(cur),
            _ => panic!("Not Support Token Type: {:?}", ope)
        }
    }

    // term.
    fn term(&mut self, cur: TokenInfo) -> Expr {
        let left_factor = self.factor(cur);
        let ope = self.next_consume();

        match ope.get_token_type() {
            Token::Multi => {
                let right_token = self.next_consume();
                let recur_factor = self.term(right_token);
                self.multiple(left_factor, recur_factor)
            }
            _ => {
                self.back(1);
                left_factor
            }
        }

    }

    // plus/minus expression.
    fn expr_add_sub(&mut self, cur: TokenInfo) -> Expr {
        let left_factor = self.term(cur);
        let ope = self.next_consume();

        match ope.get_token_type() {
            Token::Plus => {
                // 加減算演算子AST作成.
                let right_token = self.next_consume();
                let next_ope = self.next();
                if next_ope.get_token_type() == Token::Plus ||
                   next_ope.get_token_type() == Token::Minus {
                    let factor2 = self.expr_add_sub(right_token);
                    self.plus(left_factor, factor2)
                }
                else {
                    let right_factor = self.term(right_token);
                    self.plus(left_factor, right_factor)
                }
            }
            Token::Minus => {
                let right_token = self.next_consume();
                let next_ope = self.next();
                if next_ope.get_token_type() == Token::Plus ||
                   next_ope.get_token_type() == Token::Minus {
                    let factor2 = self.expr_add_sub(right_token);
                    self.minus(left_factor, factor2)
                }
                else {
                    let right_factor = self.term(right_token);
                    self.minus(left_factor, right_factor)
                }
             }
            _ => {
                self.back(1);
                left_factor
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
        else { self.expr(cur) }
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
                    Box::new(Expr::Factor(1)),
                    Box::new(Expr::Plus(
                        Box::new(Expr::Factor(2)),
                        Box::new(Expr::Factor(3))
                    ))
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
                    Box::new(Expr::Factor(1)),
                    Box::new(Expr::Plus(
                        Box::new(Expr::Factor(2)),
                        Box::new(Expr::Plus(
                            Box::new(Expr::Factor(3)),
                            Box::new(Expr::Factor(4))
                        ))
                    ))
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
                    Box::new(Expr::Factor(100)),
                    Box::new(Expr::Minus(
                        Box::new(Expr::Factor(2)),
                        Box::new(Expr::Factor(3))
                    ))
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
                    Box::new(Expr::Factor(1)),
                    Box::new(Expr::Minus(
                        Box::new(Expr::Factor(2)),
                        Box::new(Expr::Minus(
                            Box::new(Expr::Factor(3)),
                            Box::new(Expr::Factor(4))
                        ))
                    ))
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
                    Box::new(Expr::Factor(1)),
                    Box::new(Expr::Multiple(
                        Box::new(Expr::Factor(2)),
                        Box::new(Expr::Factor(3))
                    ))
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
                    Box::new(Expr::Factor(1)),
                    Box::new(Expr::Multiple(
                        Box::new(Expr::Factor(2)),
                        Box::new(Expr::Multiple(
                            Box::new(Expr::Factor(3)),
                            Box::new(Expr::Factor(4))
                        )),
                    )),
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

