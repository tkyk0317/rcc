use std::fmt;
use token::TokenInfo;
use token::Token;

// 文法.
//   <Condition> ::= <Logical> <SubCondition>
//   <SubCondition> ::= '?' <Logical> ':' <Logical> <SubCondition>
//   <Logical> ::= <Relation> <SubLogical>
//   <SubLogical> ::= ['&&' | '||'] <Relation> <SubLogical>
//   <Relation> ::= <Expr> <SubRelation>
//   <SubRelation> ::= <Op> <Expr> <SubRelation>
//   <Op> ::= ['==' | '!=' | '<' | '>' | '>=' | '<=']
//   <Expr> ::= <Term> <AddSubExpr>
//   <AddSubExpr> ::= ['+'|'-'] <Term> <AddSubExpr>
//   <Term> ::= <Factor> <SubTerm>
//   <MultiDivTerm> ::= ['*'|'.'|'%'] <Factor> <MultiDivTerm>
//   <Factor> ::= '(' NUMBER ')'

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Condition(Box<Expr>, Box<Expr>, Box<Expr>),
    LogicalAnd(Box<Expr>, Box<Expr>),
    LogicalOr(Box<Expr>, Box<Expr>),
    Equal(Box<Expr>, Box<Expr>),
    NotEqual(Box<Expr>, Box<Expr>),
    LessThan(Box<Expr>, Box<Expr>),
    GreaterThan(Box<Expr>, Box<Expr>),
    LessThanEqual(Box<Expr>, Box<Expr>),
    GreaterThanEqual(Box<Expr>, Box<Expr>),
    Plus(Box<Expr>, Box<Expr>),
    Minus(Box<Expr>, Box<Expr>),
    Multiple(Box<Expr>, Box<Expr>),
    Division(Box<Expr>, Box<Expr>),
    Remainder(Box<Expr>, Box<Expr>),
    Factor(i64),
}

// 出力フォーマット定義.
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expr::Condition(ref a, ref b, ref c) =>  write!(f, "{} ? {} : {}", *a, *b, *c),
            Expr::LogicalAnd(ref a, ref b) => write!(f, "{} && {}", *a, *b),
            Expr::LogicalOr(ref a, ref b) => write!(f, "{} || {}", *a, *b),
            Expr::Equal(ref a, ref b) => write!(f, "{} == {}", *a, *b),
            Expr::NotEqual(ref a, ref b) => write!(f, "{} != {}", *a, *b),
            Expr::LessThan(ref a, ref b) => write!(f, "{} < {}", *a, *b),
            Expr::LessThanEqual(ref a, ref b) => write!(f, "{} <= {}", *a, *b),
            Expr::GreaterThan(ref a, ref b) => write!(f, "{} > {}", *a, *b),
            Expr::GreaterThanEqual(ref a, ref b) => write!(f, "{} >= {}", *a, *b),
            Expr::Plus(ref a, ref b) => write!(f, "{} + {}", *a, *b),
            Expr::Minus(ref a, ref b) => write!(f, "{} - {}", *a, *b),
            Expr::Multiple(ref a, ref b) => write!(f, "{} * {}", *a, *b),
            Expr::Division(ref a, ref b) => write!(f, "{} / {}", *a, *b),
            Expr::Remainder(ref a, ref b) =>  write!(f, "{} % {}", *a, *b),
            Expr::Factor(v) =>  write!(f, "{}", v),
        }
    }
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
        self.condition(None)
    }

    // condition.
    fn condition(&mut self, acc: Option<Expr>) -> Expr {
        let left = self.logical(acc);
        self.sub_condition(left)
    }

    // sub condition.
    fn sub_condition(&mut self, acc: Expr) -> Expr {
        let ope_type = self.next().get_token_type();
        match ope_type {
            Token::Question => {
                self.consume();
                let middle = self.logical(None);

                // コロンがない場合、終了.
                if self.next_consume().get_token_type() != Token::Colon { panic!("Not Exists Colon") }
                else {
                    let right = self.logical(None);
                    let tree = Expr::Condition(Box::new(acc), Box::new(middle), Box::new(right));
                    self.sub_condition(tree)
                }
            }
            _ => acc
        }
    }

    // logical.
    fn logical(&mut self, acc: Option<Expr>) -> Expr {
        let left = self.relation(acc);
        self.sub_logical(left)
    }

    // sub logical.
    fn sub_logical(&mut self, acc: Expr) -> Expr {
        let create = |ope: Token, left, right| {
            match ope {
                Token::LogicalAnd => Expr::LogicalAnd(Box::new(left), Box::new(right)),
                _ => Expr::LogicalOr(Box::new(left), Box::new(right))
            }
        };

        let ope_type = self.next().get_token_type();
        match ope_type {
            Token::LogicalAnd | Token::LogicalOr => {
                self.consume();
                let right = self.relation(None);
                self.sub_logical(create(ope_type, acc, right))
            }
            _ => acc
        }
    }

    // relation.
    fn relation(&mut self, acc: Option<Expr>) -> Expr {
        let left = self.expr(acc);
        self.sub_relation(left)
    }

    // sub relation.
    fn sub_relation(&mut self, acc: Expr) -> Expr {
        let create = |ope: Token, left, right| {
            match ope {
                Token::Equal => Expr::Equal(Box::new(left), Box::new(right)),
                Token::NotEqual => Expr::NotEqual(Box::new(left), Box::new(right)),
                Token::LessThan => Expr::LessThan(Box::new(left), Box::new(right)),
                Token::GreaterThan => Expr::GreaterThan(Box::new(left), Box::new(right)),
                Token::LessThanEqual => Expr::LessThanEqual(Box::new(left), Box::new(right)),
                Token::GreaterThanEqual => Expr::GreaterThanEqual(Box::new(left), Box::new(right)),
                _ => panic!("Not Support Token Type {:?}", ope)
            }
        };

        let ope_type = self.next().get_token_type();
        match ope_type {
            Token::Equal | Token::NotEqual |
            Token::LessThan | Token::LessThanEqual |
            Token::GreaterThan | Token::GreaterThanEqual => {
                self.consume();
                let right = self.expr(None);
                self.sub_relation(create(ope_type, acc, right))
            }
            _ => acc
        }
    }

    // expression
    fn expr(&mut self, acc: Option<Expr>) -> Expr {
        let factor = self.term(acc);
        self.expr_add_sub(factor)
    }

    // add or sub expression.
    fn expr_add_sub(&mut self, acc: Expr) -> Expr {
        let create = |ope, left, right| {
            match ope {
                Token::Plus => Expr::Plus(Box::new(left), Box::new(right)),
                _ => Expr::Minus(Box::new(left), Box::new(right))
            }
        };

        let ope = self.next();
        match ope.get_token_type() {
            Token::Plus | Token::Minus => {
                self.consume();
                let right = self.term(None);
                self.expr_add_sub(create(ope.get_token_type(), acc, right))
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
        let create = |ope, left, right| {
            match ope {
                Token::Multi => Expr::Multiple(Box::new(left), Box::new(right)),
                Token::Division => Expr::Division(Box::new(left), Box::new(right)),
                _ => Expr::Remainder(Box::new(left), Box::new(right))
            }
        };

        let ope = self.next();
        match ope.get_token_type() {
            Token::Multi | Token::Division | Token::Remainder => {
                self.consume();
                let right = self.factor(None);
                self.term_multi_div(create(ope.get_token_type(), acc, right))
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
                let tree = self.condition(Some(factor));

                // 閉じカッコがあるかどうかチェック.
                if Token::RightBracket != self.next_consume().get_token_type() {
                    panic!("Not Exists Right Bracket")
                }
                tree
            }
            _ => acc.unwrap()
        }
    }

    // number
    fn number(&self, token: TokenInfo ) -> Expr {
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
    #[allow(dead_code)]
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
        {
            let data =
                vec![
                    TokenInfo::new(Token::Number, "2".to_string()),
                    TokenInfo::new(Token::LessThan, "<".to_string()),
                    TokenInfo::new(Token::Number, "3".to_string()),
                    TokenInfo::new(Token::Equal, "==".to_string()),
                    TokenInfo::new(Token::Number, "4".to_string()),
                    TokenInfo::new(Token::GreaterThanEqual, ">=".to_string()),
                    TokenInfo::new(Token::Number, "5".to_string()),
                ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::GreaterThanEqual(
                    Box::new(Expr::Equal(
                       Box::new(Expr::LessThan(
                           Box::new(Expr::Factor(2)), Box::new(Expr::Factor(3))
                       )),
                       Box::new(Expr::Factor(4))
                    )),
                    Box::new(Expr::Factor(5))
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

    #[test]
    fn test_equal_operator() {
        // 等価演算子テスト.
        {
            let data =
                vec![
                    TokenInfo::new(Token::Number, "1".to_string()),
                    TokenInfo::new(Token::Plus, '+'.to_string()),
                    TokenInfo::new(Token::Number, "2".to_string()),
                    TokenInfo::new(Token::Equal, "==".to_string()),
                    TokenInfo::new(Token::Number, "3".to_string()),
                    TokenInfo::new(Token::Plus, '+'.to_string()),
                    TokenInfo::new(Token::Number, "4".to_string()),
                ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::Equal(
                    Box::new(Expr::Plus(
                        Box::new(Expr::Factor(1)),
                        Box::new(Expr::Factor(2))
                    )),
                    Box::new(Expr::Plus(
                        Box::new(Expr::Factor(3)),
                        Box::new(Expr::Factor(4))
                    ))
                 )
            )
        }
        {
            let data =
                vec![
                    TokenInfo::new(Token::Number, "1".to_string()),
                    TokenInfo::new(Token::Multi, '*'.to_string()),
                    TokenInfo::new(Token::Number, "2".to_string()),
                    TokenInfo::new(Token::Equal, "==".to_string()),
                    TokenInfo::new(Token::Number, "3".to_string()),
                    TokenInfo::new(Token::Multi, '*'.to_string()),
                    TokenInfo::new(Token::Number, "4".to_string()),
                ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::Equal(
                    Box::new(Expr::Multiple(
                        Box::new(Expr::Factor(1)),
                        Box::new(Expr::Factor(2))
                    )),
                    Box::new(Expr::Multiple(
                        Box::new(Expr::Factor(3)),
                        Box::new(Expr::Factor(4))
                    ))
                 )
            )
        }
        {
            let data =
                vec![
                    TokenInfo::new(Token::Number, "1".to_string()),
                    TokenInfo::new(Token::Multi, '*'.to_string()),
                    TokenInfo::new(Token::Number, "2".to_string()),
                    TokenInfo::new(Token::Plus, '+'.to_string()),
                    TokenInfo::new(Token::Number, "1".to_string()),
                    TokenInfo::new(Token::Equal, "==".to_string()),
                    TokenInfo::new(Token::Number, "3".to_string()),
                    TokenInfo::new(Token::Minus, '-'.to_string()),
                    TokenInfo::new(Token::Number, "4".to_string()),
                ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::Equal(
                    Box::new(Expr::Plus(
                        Box::new(Expr::Multiple(
                            Box::new(Expr::Factor(1)),
                            Box::new(Expr::Factor(2))
                        )),
                        Box::new(Expr::Factor(1))
                    )),
                    Box::new(Expr::Minus(
                        Box::new(Expr::Factor(3)),
                        Box::new(Expr::Factor(4))
                    ))
                 )
            )
        }
    }

    #[test]
    fn test_not_equal_operator() {
        {
            let data =
                vec![
                    TokenInfo::new(Token::Number, "1".to_string()),
                    TokenInfo::new(Token::Multi, '*'.to_string()),
                    TokenInfo::new(Token::Number, "2".to_string()),
                    TokenInfo::new(Token::Plus, '+'.to_string()),
                    TokenInfo::new(Token::Number, "1".to_string()),
                    TokenInfo::new(Token::NotEqual, "!=".to_string()),
                    TokenInfo::new(Token::Number, "3".to_string()),
                    TokenInfo::new(Token::Minus, '-'.to_string()),
                    TokenInfo::new(Token::Number, "4".to_string()),
                ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::NotEqual(
                    Box::new(Expr::Plus(
                        Box::new(Expr::Multiple(
                            Box::new(Expr::Factor(1)),
                            Box::new(Expr::Factor(2))
                        )),
                        Box::new(Expr::Factor(1))
                    )),
                    Box::new(Expr::Minus(
                        Box::new(Expr::Factor(3)),
                        Box::new(Expr::Factor(4))
                    ))
                 )
            )
        }
    }

    #[test]
    fn test_less_than_operator() {
        {
            let data =
                vec![
                    TokenInfo::new(Token::Number, "1".to_string()),
                    TokenInfo::new(Token::Multi, '*'.to_string()),
                    TokenInfo::new(Token::Number, "2".to_string()),
                    TokenInfo::new(Token::Plus, '+'.to_string()),
                    TokenInfo::new(Token::Number, "1".to_string()),
                    TokenInfo::new(Token::LessThan, "<".to_string()),
                    TokenInfo::new(Token::Number, "3".to_string()),
                    TokenInfo::new(Token::Minus, '-'.to_string()),
                    TokenInfo::new(Token::Number, "4".to_string()),
                ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::LessThan(
                    Box::new(Expr::Plus(
                        Box::new(Expr::Multiple(
                            Box::new(Expr::Factor(1)),
                            Box::new(Expr::Factor(2))
                        )),
                        Box::new(Expr::Factor(1))
                    )),
                    Box::new(Expr::Minus(
                        Box::new(Expr::Factor(3)),
                        Box::new(Expr::Factor(4))
                    ))
                 )
            )
        }
        {
            let data =
                vec![
                    TokenInfo::new(Token::Number, "1".to_string()),
                    TokenInfo::new(Token::Multi, '*'.to_string()),
                    TokenInfo::new(Token::Number, "2".to_string()),
                    TokenInfo::new(Token::Plus, '+'.to_string()),
                    TokenInfo::new(Token::Number, "1".to_string()),
                    TokenInfo::new(Token::LessThanEqual, "<=".to_string()),
                    TokenInfo::new(Token::Number, "3".to_string()),
                    TokenInfo::new(Token::Minus, '-'.to_string()),
                    TokenInfo::new(Token::Number, "4".to_string()),
                ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::LessThanEqual(
                    Box::new(Expr::Plus(
                        Box::new(Expr::Multiple(
                            Box::new(Expr::Factor(1)),
                            Box::new(Expr::Factor(2))
                        )),
                        Box::new(Expr::Factor(1))
                    )),
                    Box::new(Expr::Minus(
                        Box::new(Expr::Factor(3)),
                        Box::new(Expr::Factor(4))
                    ))
                 )
            )
        }
    }

    #[test]
    fn test_greater_than_operator() {
        {
            let data =
                vec![
                    TokenInfo::new(Token::Number, "1".to_string()),
                    TokenInfo::new(Token::Multi, '*'.to_string()),
                    TokenInfo::new(Token::Number, "2".to_string()),
                    TokenInfo::new(Token::Plus, '+'.to_string()),
                    TokenInfo::new(Token::Number, "1".to_string()),
                    TokenInfo::new(Token::GreaterThan, ">".to_string()),
                    TokenInfo::new(Token::Number, "3".to_string()),
                    TokenInfo::new(Token::Minus, '-'.to_string()),
                    TokenInfo::new(Token::Number, "4".to_string()),
                ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::GreaterThan(
                    Box::new(Expr::Plus(
                        Box::new(Expr::Multiple(
                            Box::new(Expr::Factor(1)),
                            Box::new(Expr::Factor(2))
                        )),
                        Box::new(Expr::Factor(1))
                    )),
                    Box::new(Expr::Minus(
                        Box::new(Expr::Factor(3)),
                        Box::new(Expr::Factor(4))
                    ))
                 )
            )
        }
        {
            let data =
                vec![
                    TokenInfo::new(Token::Number, "1".to_string()),
                    TokenInfo::new(Token::Multi, '*'.to_string()),
                    TokenInfo::new(Token::Number, "2".to_string()),
                    TokenInfo::new(Token::Plus, '+'.to_string()),
                    TokenInfo::new(Token::Number, "1".to_string()),
                    TokenInfo::new(Token::GreaterThanEqual, ">=".to_string()),
                    TokenInfo::new(Token::Number, "3".to_string()),
                    TokenInfo::new(Token::Minus, '-'.to_string()),
                    TokenInfo::new(Token::Number, "4".to_string()),
                ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::GreaterThanEqual(
                    Box::new(Expr::Plus(
                        Box::new(Expr::Multiple(
                            Box::new(Expr::Factor(1)),
                            Box::new(Expr::Factor(2))
                        )),
                        Box::new(Expr::Factor(1))
                    )),
                    Box::new(Expr::Minus(
                        Box::new(Expr::Factor(3)),
                        Box::new(Expr::Factor(4))
                    ))
                 )
            )
        }
    }

    #[test]
    fn test_logical_operator() {
        // &&演算子のテスト.
        {
            let data =
                vec![
                    TokenInfo::new(Token::Number, "2".to_string()),
                    TokenInfo::new(Token::LogicalAnd, "&&".to_string()),
                    TokenInfo::new(Token::Number, "3".to_string()),
                ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::LogicalAnd(Box::new(Expr::Factor(2)), Box::new(Expr::Factor(3)))
            )
        }
        {
            let data =
                vec![
                    TokenInfo::new(Token::Number, "2".to_string()),
                    TokenInfo::new(Token::Plus, "+".to_string()),
                    TokenInfo::new(Token::Number, "3".to_string()),
                    TokenInfo::new(Token::LogicalAnd, "&&".to_string()),
                    TokenInfo::new(Token::Number, "4".to_string()),
                    TokenInfo::new(Token::Plus, "+".to_string()),
                    TokenInfo::new(Token::Number, "5".to_string()),
                ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::LogicalAnd(
                    Box::new(Expr::Plus(
                        Box::new(Expr::Factor(2)), Box::new(Expr::Factor(3))
                    )),
                    Box::new(Expr::Plus(
                        Box::new(Expr::Factor(4)), Box::new(Expr::Factor(5))
                    ))
                )
            )
        }
        {
            let data =
                vec![
                    TokenInfo::new(Token::Number, "2".to_string()),
                    TokenInfo::new(Token::Plus, "+".to_string()),
                    TokenInfo::new(Token::Number, "3".to_string()),
                    TokenInfo::new(Token::Equal, "==".to_string()),
                    TokenInfo::new(Token::Number, "4".to_string()),
                    TokenInfo::new(Token::Plus, "+".to_string()),
                    TokenInfo::new(Token::Number, "5".to_string()),
                    TokenInfo::new(Token::LogicalAnd, "&&".to_string()),
                    TokenInfo::new(Token::Number, "6".to_string()),
                    TokenInfo::new(Token::Plus, "+".to_string()),
                    TokenInfo::new(Token::Number, "7".to_string()),
                    TokenInfo::new(Token::NotEqual, "!=".to_string()),
                    TokenInfo::new(Token::Number, "8".to_string()),
                    TokenInfo::new(Token::Plus, "+".to_string()),
                    TokenInfo::new(Token::Number, "9".to_string()),

                ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::LogicalAnd(
                    Box::new(Expr::Equal(
                        Box::new(Expr::Plus(
                            Box::new(Expr::Factor(2)), Box::new(Expr::Factor(3))
                        )),
                        Box::new(Expr::Plus(
                            Box::new(Expr::Factor(4)), Box::new(Expr::Factor(5))
                        ))
                    )),
                    Box::new(Expr::NotEqual(
                        Box::new(Expr::Plus(
                            Box::new(Expr::Factor(6)), Box::new(Expr::Factor(7))
                        )),
                        Box::new(Expr::Plus(
                            Box::new(Expr::Factor(8)), Box::new(Expr::Factor(9))
                        ))
                    ))
                )
            )
        }
        // ||演算子のテスト.
        {
            let data =
                vec![
                    TokenInfo::new(Token::Number, "2".to_string()),
                    TokenInfo::new(Token::LogicalOr, "||".to_string()),
                    TokenInfo::new(Token::Number, "3".to_string()),
                ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::LogicalOr(Box::new(Expr::Factor(2)), Box::new(Expr::Factor(3)))
            )
        }
        {
            let data =
                vec![
                    TokenInfo::new(Token::Number, "2".to_string()),
                    TokenInfo::new(Token::Plus, "+".to_string()),
                    TokenInfo::new(Token::Number, "3".to_string()),
                    TokenInfo::new(Token::LogicalOr, "||".to_string()),
                    TokenInfo::new(Token::Number, "4".to_string()),
                    TokenInfo::new(Token::Plus, "+".to_string()),
                    TokenInfo::new(Token::Number, "5".to_string()),
                ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::LogicalOr(
                    Box::new(Expr::Plus(
                        Box::new(Expr::Factor(2)), Box::new(Expr::Factor(3))
                    )),
                    Box::new(Expr::Plus(
                        Box::new(Expr::Factor(4)), Box::new(Expr::Factor(5))
                    ))
                )
            )
        }
        {
            let data =
                vec![
                    TokenInfo::new(Token::Number, "2".to_string()),
                    TokenInfo::new(Token::Plus, "+".to_string()),
                    TokenInfo::new(Token::Number, "3".to_string()),
                    TokenInfo::new(Token::Equal, "==".to_string()),
                    TokenInfo::new(Token::Number, "4".to_string()),
                    TokenInfo::new(Token::Plus, "+".to_string()),
                    TokenInfo::new(Token::Number, "5".to_string()),
                    TokenInfo::new(Token::LogicalOr, "||".to_string()),
                    TokenInfo::new(Token::Number, "6".to_string()),
                    TokenInfo::new(Token::Plus, "+".to_string()),
                    TokenInfo::new(Token::Number, "7".to_string()),
                    TokenInfo::new(Token::NotEqual, "!=".to_string()),
                    TokenInfo::new(Token::Number, "8".to_string()),
                    TokenInfo::new(Token::Plus, "+".to_string()),
                    TokenInfo::new(Token::Number, "9".to_string()),

                ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::LogicalOr(
                    Box::new(Expr::Equal(
                        Box::new(Expr::Plus(
                            Box::new(Expr::Factor(2)), Box::new(Expr::Factor(3))
                        )),
                        Box::new(Expr::Plus(
                            Box::new(Expr::Factor(4)), Box::new(Expr::Factor(5))
                        ))
                    )),
                    Box::new(Expr::NotEqual(
                        Box::new(Expr::Plus(
                            Box::new(Expr::Factor(6)), Box::new(Expr::Factor(7))
                        )),
                        Box::new(Expr::Plus(
                            Box::new(Expr::Factor(8)), Box::new(Expr::Factor(9))
                        ))
                    ))
                )
            )
        }
    }

    #[test]
    fn test_mix_logical_operator() {
        {
            let data =
                vec![
                    TokenInfo::new(Token::Number, "2".to_string()),
                    TokenInfo::new(Token::LogicalOr, "||".to_string()),
                    TokenInfo::new(Token::Number, "3".to_string()),
                    TokenInfo::new(Token::LogicalAnd, "&&".to_string()),
                    TokenInfo::new(Token::Number, "4".to_string()),
                    TokenInfo::new(Token::LogicalOr, "||".to_string()),
                    TokenInfo::new(Token::Number, "5".to_string()),
                ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::LogicalOr(
                    Box::new(Expr::LogicalAnd(
                        Box::new(Expr::LogicalOr(
                            Box::new(Expr::Factor(2)), Box::new(Expr::Factor(3))
                        )),
                        Box::new(Expr::Factor(4))
                    )),
                    Box::new(Expr::Factor(5))
                )
            )
        }
    }

    #[test]
    fn test_condition_expression() {
        {
            let data =
                vec![
                    TokenInfo::new(Token::Number, "2".to_string()),
                    TokenInfo::new(Token::Equal, "==".to_string()),
                    TokenInfo::new(Token::Number, "3".to_string()),
                    TokenInfo::new(Token::Question, "?".to_string()),
                    TokenInfo::new(Token::Number, "1".to_string()),
                    TokenInfo::new(Token::Colon, ":".to_string()),
                    TokenInfo::new(Token::Number, "5".to_string()),
                ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::Condition(
                    Box::new(Expr::Equal(
                        Box::new(Expr::Factor(2)), Box::new(Expr::Factor(3))
                    )),
                    Box::new(Expr::Factor(1)),
                    Box::new(Expr::Factor(5))
                )
            )
        }
        {
            let data =
                vec![
                    TokenInfo::new(Token::Number, "2".to_string()),
                    TokenInfo::new(Token::Equal, "==".to_string()),
                    TokenInfo::new(Token::Number, "3".to_string()),
                    TokenInfo::new(Token::Question, "?".to_string()),
                    TokenInfo::new(Token::LeftBracket, "(".to_string()),
                    TokenInfo::new(Token::Number, "10".to_string()),
                    TokenInfo::new(Token::Equal, "==".to_string()),
                    TokenInfo::new(Token::Number, "11".to_string()),
                    TokenInfo::new(Token::Question, "?".to_string()),
                    TokenInfo::new(Token::Number, "12".to_string()),
                    TokenInfo::new(Token::Colon, ":".to_string()),
                    TokenInfo::new(Token::Number, "13".to_string()),
                    TokenInfo::new(Token::RightBracket, ")".to_string()),
                    TokenInfo::new(Token::Colon, ":".to_string()),
                    TokenInfo::new(Token::Number, "5".to_string()),
                ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::Condition(
                    Box::new(Expr::Equal(
                        Box::new(Expr::Factor(2)), Box::new(Expr::Factor(3))
                    )),
                    Box::new(Expr::Condition(
                        Box::new(Expr::Equal(
                            Box::new(Expr::Factor(10)), Box::new(Expr::Factor(11))
                        )),
                        Box::new(Expr::Factor(12)),
                        Box::new(Expr::Factor(13))
                    )),
                    Box::new(Expr::Factor(5))
                )
            )
        }
    }
}

