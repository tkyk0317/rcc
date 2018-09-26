use std::fmt;
use token::TokenInfo;
use token::Token;

// 文法.
//   <Expression> ::= <Block>
//   <Block> ::=  <Assign> ';'
//   <Assign> ::= VARIABLE '=' <Condition>
//   <Condition> ::= <Logical> <SubCondition>
//   <SubCondition> ::= '?' <Logical> ':' <Logical> <SubCondition>
//   <Logical> ::= <Relation> <SubLogical>
//   <SubLogical> ::= ['&&' | '||'] <BitOp> <SubLogical>
//   <BitOp> ::=  <Relation> <SubBitOp>
//   <SubBitOp> ::= ['&'|'|'|'^'] <Relation> <SubBitOp>
//   <Relation> ::= <Shift> <SubRelation>
//   <SubRelation> ::= <Op> <Shift> <SubRelation>
//   <Op> ::= ['==' | '!=' | '<' | '>' | '>=' | '<=']
//   <Shift> ::= <Expr> <SubShift>
//   <SubShift> ::= ['<<'|'>>'] <Expr> <SubShift>
//   <Expr> ::= <Term> <AddSubExpr>
//   <AddSubExpr> ::= ['+'|'-'] <Term> <AddSubExpr>
//   <Term> ::= <Factor> <SubTerm>
//   <MultiDivTerm> ::= ['*'|'/'|'%'] <Factor> <MultiDivTerm>
//   <Factor> ::= '(' NUMBER ')' | <UnAry>
//   <UnAry> ::= ['!'|'+'|'-'|'~'] NUMBER

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Block(Box<Expr>, Box<Expr>),
    Condition(Box<Expr>, Box<Expr>, Box<Expr>),
    LogicalAnd(Box<Expr>, Box<Expr>),
    LogicalOr(Box<Expr>, Box<Expr>),
    BitAnd(Box<Expr>, Box<Expr>),
    BitOr(Box<Expr>, Box<Expr>),
    BitXor(Box<Expr>, Box<Expr>),
    Equal(Box<Expr>, Box<Expr>),
    NotEqual(Box<Expr>, Box<Expr>),
    LessThan(Box<Expr>, Box<Expr>),
    GreaterThan(Box<Expr>, Box<Expr>),
    LessThanEqual(Box<Expr>, Box<Expr>),
    GreaterThanEqual(Box<Expr>, Box<Expr>),
    Plus(Box<Expr>, Box<Expr>),
    Minus(Box<Expr>, Box<Expr>),
    LeftShift(Box<Expr>, Box<Expr>),
    RightShift(Box<Expr>, Box<Expr>),
    Multiple(Box<Expr>, Box<Expr>),
    Division(Box<Expr>, Box<Expr>),
    Remainder(Box<Expr>, Box<Expr>),
    UnPlus(Box<Expr>),
    UnMinus(Box<Expr>),
    Not(Box<Expr>),
    BitReverse(Box<Expr>),
    Assign(Box<Expr>, Box<Expr>),
    Factor(i64),
    Variable(String),
}

// 出力フォーマット定義.
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expr::Block(ref a, ref b) => write!(f, "{}{}", *a, *b),
            Expr::Condition(ref a, ref b, ref c) => write!(f, "{} ? {} : {}", *a, *b, *c),
            Expr::LogicalAnd(ref a, ref b) => write!(f, "{} && {}", *a, *b),
            Expr::LogicalOr(ref a, ref b) => write!(f, "{} || {}", *a, *b),
            Expr::Equal(ref a, ref b) => write!(f, "{} == {}", *a, *b),
            Expr::NotEqual(ref a, ref b) => write!(f, "{} != {}", *a, *b),
            Expr::LessThan(ref a, ref b) => write!(f, "{} < {}", *a, *b),
            Expr::LessThanEqual(ref a, ref b) => write!(f, "{} <= {}", *a, *b),
            Expr::GreaterThan(ref a, ref b) => write!(f, "{} > {}", *a, *b),
            Expr::GreaterThanEqual(ref a, ref b) => write!(f, "{} >= {}", *a, *b),
            Expr::LeftShift(ref a, ref b) => write!(f, "{} << {}", *a, *b),
            Expr::RightShift(ref a, ref b) => write!(f, "{} >> {}", *a, *b),
            Expr::Plus(ref a, ref b) => write!(f, "{} + {}", *a, *b),
            Expr::Minus(ref a, ref b) => write!(f, "{} - {}", *a, *b),
            Expr::Multiple(ref a, ref b) => write!(f, "{} * {}", *a, *b),
            Expr::Division(ref a, ref b) => write!(f, "{} / {}", *a, *b),
            Expr::Remainder(ref a, ref b) => write!(f, "{} % {}", *a, *b),
            Expr::BitAnd(ref a, ref b) => write!(f, "{} & {}", *a, *b),
            Expr::BitOr(ref a, ref b) => write!(f, "{} | {}", *a, *b),
            Expr::BitXor(ref a, ref b) => write!(f, "{} ^ {}", *a, *b),
            Expr::UnPlus(ref a) => write!(f, "+{}", *a),
            Expr::UnMinus(ref a) => write!(f, "-{}", *a),
            Expr::Not(ref a) => write!(f, "!{}", *a),
            Expr::BitReverse(ref a) => write!(f, "~{}", *a),
            Expr::Assign(ref a, ref b) => write!(f, "{} = {}", *a, *b),
            Expr::Variable(ref v) => write!(f, "{}", v.clone()),
            Expr::Factor(v) => write!(f, "{}", v),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Ast<'a> {
    tokens: &'a Vec<TokenInfo>, // トークン配列.
    current_pos: usize, // 現在読み取り位置.
}

// 抽象構文木をトークン列から作成する
impl<'a> Ast<'a> {
    // コンストラクタ.
    pub fn new(tokens: &Vec<TokenInfo>) -> Ast {
        Ast {
            current_pos: 0,
            tokens: tokens,
        }
    }

    // トークン列を受け取り、抽象構文木を返す.
    pub fn parse(&mut self) -> Expr {
        self.block()
    }

    // block.
    fn block(&mut self) -> Expr {
        let left = self.assign();
        self.sub_block(left)
    }

    //sub block.
    fn sub_block(&mut self, acc: Expr) -> Expr {
        let token = self.next();
        match token.get_token_type() {
            Token::SemiColon => {
                self.consume();

                // 次のトークンがあれば処理を行う.
                if self.next().get_token_type() != Token::Unknown {
                    let right = self.condition(None);
                    Expr::Block(Box::new(acc), Box::new(self.sub_block(right)))
                } else {
                    acc
                }
            }
            _ => acc,
        }
    }

    // assign.
    fn assign(&mut self) -> Expr {
        let left = self.next();
        match left.get_token_type() {
            Token::Variable => {
                // ひとまず、assign operator決め打ち.
                self.consume();
                let var = Expr::Variable(left.get_token_value());
                self.consume();
                Expr::Assign(Box::new(var), Box::new(self.condition(None)))
            }
            _ => self.condition(None)
        }
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
                if self.next_consume().get_token_type() != Token::Colon {
                    panic!("Not Exists Colon")
                } else {
                    let right = self.logical(None);
                    let tree = Expr::Condition(Box::new(acc), Box::new(middle), Box::new(right));
                    self.sub_condition(tree)
                }
            }
            _ => acc,
        }
    }

    // logical.
    fn logical(&mut self, acc: Option<Expr>) -> Expr {
        let left = self.bit_operator(acc);
        self.sub_logical(left)
    }

    // sub logical.
    fn sub_logical(&mut self, acc: Expr) -> Expr {
        let create = |ope: Token, left, right| match ope {
            Token::LogicalAnd => Expr::LogicalAnd(Box::new(left), Box::new(right)),
            Token::Assign => Expr::Assign(Box::new(left), Box::new(right)),
            _ => Expr::LogicalOr(Box::new(left), Box::new(right)),
        };

        let ope_type = self.next().get_token_type();
        match ope_type {
            Token::LogicalAnd | Token::LogicalOr | Token::Assign => {
                self.consume();
                let right = self.bit_operator(None);
                self.sub_logical(create(ope_type, acc, right))
            }
            _ => acc,
        }
    }

    // bit operator.
    fn bit_operator(&mut self, acc: Option<Expr>) -> Expr {
        let left = self.relation(acc);
        self.sub_bit_operator(left)
    }

    // sub bit operator.
    fn sub_bit_operator(&mut self, acc: Expr) -> Expr {
        let create = |ope, left, right| match ope {
            Token::BitOr => Expr::BitOr(Box::new(left), Box::new(right)),
            Token::BitAnd => Expr::BitAnd(Box::new(left), Box::new(right)),
            Token::BitXor => Expr::BitXor(Box::new(left), Box::new(right)),
            _ => panic!("sub_bit_operator: Not Support Token {:?}", ope),
        };

        let token = self.next();
        match token.get_token_type() {
            Token::BitOr | Token::BitAnd | Token::BitXor => {
                self.consume();
                let right = self.relation(None);
                self.sub_bit_operator(create(token.get_token_type(), acc, right))
            }
            _ => acc,
        }
    }

    // relation.
    fn relation(&mut self, acc: Option<Expr>) -> Expr {
        let left = self.shift(acc);
        self.sub_relation(left)
    }

    // sub relation.
    fn sub_relation(&mut self, acc: Expr) -> Expr {
        let create = |ope: Token, left, right| match ope {
            Token::Equal => Expr::Equal(Box::new(left), Box::new(right)),
            Token::NotEqual => Expr::NotEqual(Box::new(left), Box::new(right)),
            Token::LessThan => Expr::LessThan(Box::new(left), Box::new(right)),
            Token::GreaterThan => Expr::GreaterThan(Box::new(left), Box::new(right)),
            Token::LessThanEqual => Expr::LessThanEqual(Box::new(left), Box::new(right)),
            Token::GreaterThanEqual => Expr::GreaterThanEqual(Box::new(left), Box::new(right)),
            _ => panic!("Not Support Token Type {:?}", ope),
        };

        let ope_type = self.next().get_token_type();
        match ope_type {
            Token::Equal | Token::NotEqual | Token::LessThan | Token::LessThanEqual |
            Token::GreaterThan | Token::GreaterThanEqual => {
                self.consume();
                let right = self.shift(None);
                self.sub_relation(create(ope_type, acc, right))
            }
            _ => acc,
        }
    }

    // shift operation.
    fn shift(&mut self, acc: Option<Expr>) -> Expr {
        let left = self.expr(acc);
        self.sub_shift(left)
    }

    fn sub_shift(&mut self, acc: Expr) -> Expr {
        let create = |ope: Token, left, right| match ope {
            Token::LeftShift => Expr::LeftShift(Box::new(left), Box::new(right)),
            Token::RightShift => Expr::RightShift(Box::new(left), Box::new(right)),
            _ => panic!("Not Support Token {:?}", ope),
        };

        let token = self.next();
        match token.get_token_type() {
            Token::LeftShift | Token::RightShift => {
                self.consume();
                let right = self.expr(None);
                self.sub_shift(create(token.get_token_type(), acc, right))
            }
            _ => acc,
        }
    }

    // expression
    fn expr(&mut self, acc: Option<Expr>) -> Expr {
        let left = self.term(acc);
        self.expr_add_sub(left)
    }

    // add or sub expression.
    fn expr_add_sub(&mut self, acc: Expr) -> Expr {
        let create = |ope, left, right| match ope {
            Token::Plus => Expr::Plus(Box::new(left), Box::new(right)),
            _ => Expr::Minus(Box::new(left), Box::new(right)),
        };

        let ope = self.next();
        match ope.get_token_type() {
            Token::Plus | Token::Minus => {
                self.consume();
                let right = self.term(None);
                self.expr_add_sub(create(ope.get_token_type(), acc, right))
            }
            _ => acc,
        }
    }

    // term.
    fn term(&mut self, acc: Option<Expr>) -> Expr {
        let left = self.factor(acc);
        self.term_multi_div(left)
    }

    // multiple and division term.
    fn term_multi_div(&mut self, acc: Expr) -> Expr {
        let create = |ope, left, right| match ope {
            Token::Multi => Expr::Multiple(Box::new(left), Box::new(right)),
            Token::Division => Expr::Division(Box::new(left), Box::new(right)),
            _ => Expr::Remainder(Box::new(left), Box::new(right)),
        };

        let ope = self.next();
        match ope.get_token_type() {
            Token::Multi | Token::Division | Token::Remainder => {
                self.consume();
                let right = self.factor(None);
                self.term_multi_div(create(ope.get_token_type(), acc, right))
            }
            _ => acc,
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
                let tree = self.condition(None);

                // 閉じカッコがあるかどうかチェック.
                if Token::RightBracket != self.next_consume().get_token_type() {
                    panic!("Not Exists Right Bracket")
                }
                tree
            }
            Token::Plus => {
                self.consume();
                Expr::UnPlus(Box::new(self.factor(None)))
            }
            Token::Minus => {
                self.consume();
                Expr::UnMinus(Box::new(self.factor(None)))
            }
            Token::Not => {
                self.consume();
                Expr::Not(Box::new(self.factor(None)))
            }
            Token::BitReverse => {
                self.consume();
                Expr::BitReverse(Box::new(self.factor(None)))
            }
            Token::Variable => {
                self.consume();
                Expr::Variable(token.get_token_value())
            }
            _ => acc.unwrap(),
        }
    }

    // number
    fn number(&self, token: TokenInfo) -> Expr {
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
            let data = vec![
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::Plus, '+'.to_string()),
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
            ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::Plus(Box::new(Expr::Factor(1)), Box::new(Expr::Factor(2)))
            )
        }
        // 複数の加算テスト.
        {
            let data = vec![
                TokenInfo::new(Token::Number, '1'.to_string()),
                TokenInfo::new(Token::Plus, '+'.to_string()),
                TokenInfo::new(Token::Number, '2'.to_string()),
                TokenInfo::new(Token::Plus, '+'.to_string()),
                TokenInfo::new(Token::Number, '3'.to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
            ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::Plus(
                    Box::new(Expr::Plus(
                        Box::new(Expr::Factor(1)),
                        Box::new(Expr::Factor(2)),
                    )),
                    Box::new(Expr::Factor(3)),
                )
            )
        }
        // 複数の加算テスト.
        {
            let data = vec![
                TokenInfo::new(Token::Number, '1'.to_string()),
                TokenInfo::new(Token::Plus, '+'.to_string()),
                TokenInfo::new(Token::Number, '2'.to_string()),
                TokenInfo::new(Token::Plus, '+'.to_string()),
                TokenInfo::new(Token::Number, '3'.to_string()),
                TokenInfo::new(Token::Plus, '+'.to_string()),
                TokenInfo::new(Token::Number, '4'.to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
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
                            Box::new(Expr::Factor(2)),
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
            let data = vec![
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::Minus, '-'.to_string()),
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
            ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::Minus(Box::new(Expr::Factor(1)), Box::new(Expr::Factor(2)))
            )
        }
        // 複数の減算テスト.
        {
            let data = vec![
                TokenInfo::new(Token::Number, "100".to_string()),
                TokenInfo::new(Token::Minus, '-'.to_string()),
                TokenInfo::new(Token::Number, '2'.to_string()),
                TokenInfo::new(Token::Minus, '-'.to_string()),
                TokenInfo::new(Token::Number, '3'.to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
            ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::Minus(
                    Box::new(Expr::Minus(
                        Box::new(Expr::Factor(100)),
                        Box::new(Expr::Factor(2)),
                    )),
                    Box::new(Expr::Factor(3)),
                )
            )
        }
        // 複数の減算テスト.
        {
            let data = vec![
                TokenInfo::new(Token::Number, '1'.to_string()),
                TokenInfo::new(Token::Minus, '-'.to_string()),
                TokenInfo::new(Token::Number, '2'.to_string()),
                TokenInfo::new(Token::Minus, '-'.to_string()),
                TokenInfo::new(Token::Number, '3'.to_string()),
                TokenInfo::new(Token::Minus, '-'.to_string()),
                TokenInfo::new(Token::Number, '4'.to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
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
                            Box::new(Expr::Factor(2)),
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
            let data = vec![
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::Multi, '*'.to_string()),
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
            ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::Multiple(Box::new(Expr::Factor(1)), Box::new(Expr::Factor(2)))
            )
        }
        // 複数の減算テスト.
        {
            let data = vec![
                TokenInfo::new(Token::Number, '1'.to_string()),
                TokenInfo::new(Token::Multi, '*'.to_string()),
                TokenInfo::new(Token::Number, '2'.to_string()),
                TokenInfo::new(Token::Multi, '*'.to_string()),
                TokenInfo::new(Token::Number, '3'.to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
            ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::Multiple(
                    Box::new(Expr::Multiple(
                        Box::new(Expr::Factor(1)),
                        Box::new(Expr::Factor(2)),
                    )),
                    Box::new(Expr::Factor(3)),
                )
            )
        }
        // 複数の減算テスト.
        {
            let data = vec![
                TokenInfo::new(Token::Number, '1'.to_string()),
                TokenInfo::new(Token::Multi, '*'.to_string()),
                TokenInfo::new(Token::Number, '2'.to_string()),
                TokenInfo::new(Token::Multi, '*'.to_string()),
                TokenInfo::new(Token::Number, '3'.to_string()),
                TokenInfo::new(Token::Multi, '*'.to_string()),
                TokenInfo::new(Token::Number, '4'.to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
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
                            Box::new(Expr::Factor(2)),
                        )),
                        Box::new(Expr::Factor(3)),
                    )),
                    Box::new(Expr::Factor(4)),
                )
            )
        }
    }

    #[test]
    fn test_div_operator() {
        // 単純な乗算テスト.
        {
            let data = vec![
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::Division, '/'.to_string()),
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
            ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::Division(Box::new(Expr::Factor(1)), Box::new(Expr::Factor(2)))
            )
        }
        // 複数の減算テスト.
        {
            let data = vec![
                TokenInfo::new(Token::Number, '1'.to_string()),
                TokenInfo::new(Token::Division, '/'.to_string()),
                TokenInfo::new(Token::Number, '2'.to_string()),
                TokenInfo::new(Token::Division, '/'.to_string()),
                TokenInfo::new(Token::Number, '3'.to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
            ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::Division(
                    Box::new(Expr::Division(
                        Box::new(Expr::Factor(1)),
                        Box::new(Expr::Factor(2)),
                    )),
                    Box::new(Expr::Factor(3)),
                )
            )
        }
        // 複数の減算テスト.
        {
            let data = vec![
                TokenInfo::new(Token::Number, '1'.to_string()),
                TokenInfo::new(Token::Division, '/'.to_string()),
                TokenInfo::new(Token::Number, '2'.to_string()),
                TokenInfo::new(Token::Division, '/'.to_string()),
                TokenInfo::new(Token::Number, '3'.to_string()),
                TokenInfo::new(Token::Division, '/'.to_string()),
                TokenInfo::new(Token::Number, '4'.to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
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
                            Box::new(Expr::Factor(2)),
                        )),
                        Box::new(Expr::Factor(3)),
                    )),
                    Box::new(Expr::Factor(4)),
                )
            )
        }
    }

    #[test]
    fn test_mix_operator() {
        // 複数演算子のテスト.
        {
            let data = vec![
                TokenInfo::new(Token::Number, '1'.to_string()),
                TokenInfo::new(Token::Multi, '*'.to_string()),
                TokenInfo::new(Token::Number, '2'.to_string()),
                TokenInfo::new(Token::Plus, '+'.to_string()),
                TokenInfo::new(Token::Number, '3'.to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
            ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::Plus(
                    Box::new(Expr::Multiple(
                        Box::new(Expr::Factor(1)),
                        Box::new(Expr::Factor(2)),
                    )),
                    Box::new(Expr::Factor(3)),
                )
            )
        }
        // 複数演算子のテスト.
        {
            let data = vec![
                TokenInfo::new(Token::Number, '1'.to_string()),
                TokenInfo::new(Token::Plus, '+'.to_string()),
                TokenInfo::new(Token::Number, '2'.to_string()),
                TokenInfo::new(Token::Multi, '*'.to_string()),
                TokenInfo::new(Token::Number, '3'.to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
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
                        Box::new(Expr::Factor(3)),
                    )),
                )
            )
        }
        // 複数演算子のテスト.
        {
            let data = vec![
                TokenInfo::new(Token::Number, '1'.to_string()),
                TokenInfo::new(Token::Division, '/'.to_string()),
                TokenInfo::new(Token::Number, '2'.to_string()),
                TokenInfo::new(Token::Plus, '+'.to_string()),
                TokenInfo::new(Token::Number, '3'.to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
            ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::Plus(
                    Box::new(Expr::Division(
                        Box::new(Expr::Factor(1)),
                        Box::new(Expr::Factor(2)),
                    )),
                    Box::new(Expr::Factor(3)),
                )
            )
        }
        // 複数演算子のテスト.
        {
            let data = vec![
                TokenInfo::new(Token::Number, '1'.to_string()),
                TokenInfo::new(Token::Plus, '+'.to_string()),
                TokenInfo::new(Token::Number, '2'.to_string()),
                TokenInfo::new(Token::Division, '/'.to_string()),
                TokenInfo::new(Token::Number, '3'.to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
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
                        Box::new(Expr::Factor(3)),
                    )),
                )
            )
        }
        {
            let data = vec![
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::LessThan, "<".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::Equal, "==".to_string()),
                TokenInfo::new(Token::Number, "4".to_string()),
                TokenInfo::new(Token::GreaterThanEqual, ">=".to_string()),
                TokenInfo::new(Token::Number, "5".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
            ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::GreaterThanEqual(
                    Box::new(Expr::Equal(
                        Box::new(Expr::LessThan(
                            Box::new(Expr::Factor(2)),
                            Box::new(Expr::Factor(3)),
                        )),
                        Box::new(Expr::Factor(4)),
                    )),
                    Box::new(Expr::Factor(5)),
                )
            )
        }
    }

    #[test]
    fn test_bracket() {
        // カッコのテスト.
        {
            let data = vec![
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::Plus, '+'.to_string()),
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
            ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::Plus(Box::new(Expr::Factor(1)), Box::new(Expr::Factor(2)))
            )
        }
        {
            let data = vec![
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::Plus, '+'.to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::Plus, '+'.to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
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
                        Box::new(Expr::Factor(3)),
                    )),
                )
            )
        }
    }

    #[test]
    fn test_equal_operator() {
        // 等価演算子テスト.
        {
            let data = vec![
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::Plus, '+'.to_string()),
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::Equal, "==".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::Plus, '+'.to_string()),
                TokenInfo::new(Token::Number, "4".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
            ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::Equal(
                    Box::new(Expr::Plus(
                        Box::new(Expr::Factor(1)),
                        Box::new(Expr::Factor(2)),
                    )),
                    Box::new(Expr::Plus(
                        Box::new(Expr::Factor(3)),
                        Box::new(Expr::Factor(4)),
                    )),
                )
            )
        }
        {
            let data = vec![
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::Multi, '*'.to_string()),
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::Equal, "==".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::Multi, '*'.to_string()),
                TokenInfo::new(Token::Number, "4".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
            ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::Equal(
                    Box::new(Expr::Multiple(
                        Box::new(Expr::Factor(1)),
                        Box::new(Expr::Factor(2)),
                    )),
                    Box::new(Expr::Multiple(
                        Box::new(Expr::Factor(3)),
                        Box::new(Expr::Factor(4)),
                    )),
                )
            )
        }
        {
            let data = vec![
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::Multi, '*'.to_string()),
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::Plus, '+'.to_string()),
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::Equal, "==".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::Minus, '-'.to_string()),
                TokenInfo::new(Token::Number, "4".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
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
                            Box::new(Expr::Factor(2)),
                        )),
                        Box::new(Expr::Factor(1)),
                    )),
                    Box::new(Expr::Minus(
                        Box::new(Expr::Factor(3)),
                        Box::new(Expr::Factor(4)),
                    )),
                )
            )
        }
    }

    #[test]
    fn test_not_equal_operator() {
        {
            let data = vec![
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::Multi, '*'.to_string()),
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::Plus, '+'.to_string()),
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::NotEqual, "!=".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::Minus, '-'.to_string()),
                TokenInfo::new(Token::Number, "4".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
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
                            Box::new(Expr::Factor(2)),
                        )),
                        Box::new(Expr::Factor(1)),
                    )),
                    Box::new(Expr::Minus(
                        Box::new(Expr::Factor(3)),
                        Box::new(Expr::Factor(4)),
                    )),
                )
            )
        }
    }

    #[test]
    fn test_less_than_operator() {
        {
            let data = vec![
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::Multi, '*'.to_string()),
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::Plus, '+'.to_string()),
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::LessThan, "<".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::Minus, '-'.to_string()),
                TokenInfo::new(Token::Number, "4".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
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
                            Box::new(Expr::Factor(2)),
                        )),
                        Box::new(Expr::Factor(1)),
                    )),
                    Box::new(Expr::Minus(
                        Box::new(Expr::Factor(3)),
                        Box::new(Expr::Factor(4)),
                    )),
                )
            )
        }
        {
            let data = vec![
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::Multi, '*'.to_string()),
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::Plus, '+'.to_string()),
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::LessThanEqual, "<=".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::Minus, '-'.to_string()),
                TokenInfo::new(Token::Number, "4".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
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
                            Box::new(Expr::Factor(2)),
                        )),
                        Box::new(Expr::Factor(1)),
                    )),
                    Box::new(Expr::Minus(
                        Box::new(Expr::Factor(3)),
                        Box::new(Expr::Factor(4)),
                    )),
                )
            )
        }
    }

    #[test]
    fn test_greater_than_operator() {
        {
            let data = vec![
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::Multi, '*'.to_string()),
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::Plus, '+'.to_string()),
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::GreaterThan, ">".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::Minus, '-'.to_string()),
                TokenInfo::new(Token::Number, "4".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
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
                            Box::new(Expr::Factor(2)),
                        )),
                        Box::new(Expr::Factor(1)),
                    )),
                    Box::new(Expr::Minus(
                        Box::new(Expr::Factor(3)),
                        Box::new(Expr::Factor(4)),
                    )),
                )
            )
        }
        {
            let data = vec![
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::Multi, '*'.to_string()),
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::Plus, '+'.to_string()),
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::GreaterThanEqual, ">=".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::Minus, '-'.to_string()),
                TokenInfo::new(Token::Number, "4".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
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
                            Box::new(Expr::Factor(2)),
                        )),
                        Box::new(Expr::Factor(1)),
                    )),
                    Box::new(Expr::Minus(
                        Box::new(Expr::Factor(3)),
                        Box::new(Expr::Factor(4)),
                    )),
                )
            )
        }
    }

    #[test]
    fn test_logical_operator() {
        // &&演算子のテスト.
        {
            let data = vec![
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::LogicalAnd, "&&".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
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
            let data = vec![
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::Plus, "+".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::LogicalAnd, "&&".to_string()),
                TokenInfo::new(Token::Number, "4".to_string()),
                TokenInfo::new(Token::Plus, "+".to_string()),
                TokenInfo::new(Token::Number, "5".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
            ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::LogicalAnd(
                    Box::new(Expr::Plus(
                        Box::new(Expr::Factor(2)),
                        Box::new(Expr::Factor(3)),
                    )),
                    Box::new(Expr::Plus(
                        Box::new(Expr::Factor(4)),
                        Box::new(Expr::Factor(5)),
                    )),
                )
            )
        }
        {
            let data = vec![
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
                TokenInfo::new(Token::SemiColon, ";".to_string()),
            ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::LogicalAnd(
                    Box::new(Expr::Equal(
                        Box::new(Expr::Plus(
                            Box::new(Expr::Factor(2)),
                            Box::new(Expr::Factor(3)),
                        )),
                        Box::new(Expr::Plus(
                            Box::new(Expr::Factor(4)),
                            Box::new(Expr::Factor(5)),
                        )),
                    )),
                    Box::new(Expr::NotEqual(
                        Box::new(Expr::Plus(
                            Box::new(Expr::Factor(6)),
                            Box::new(Expr::Factor(7)),
                        )),
                        Box::new(Expr::Plus(
                            Box::new(Expr::Factor(8)),
                            Box::new(Expr::Factor(9)),
                        )),
                    )),
                )
            )
        }
        // ||演算子のテスト.
        {
            let data = vec![
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::LogicalOr, "||".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
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
            let data = vec![
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::Plus, "+".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::LogicalOr, "||".to_string()),
                TokenInfo::new(Token::Number, "4".to_string()),
                TokenInfo::new(Token::Plus, "+".to_string()),
                TokenInfo::new(Token::Number, "5".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
            ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::LogicalOr(
                    Box::new(Expr::Plus(
                        Box::new(Expr::Factor(2)),
                        Box::new(Expr::Factor(3)),
                    )),
                    Box::new(Expr::Plus(
                        Box::new(Expr::Factor(4)),
                        Box::new(Expr::Factor(5)),
                    )),
                )
            )
        }
        {
            let data = vec![
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
                TokenInfo::new(Token::SemiColon, ";".to_string()),
            ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::LogicalOr(
                    Box::new(Expr::Equal(
                        Box::new(Expr::Plus(
                            Box::new(Expr::Factor(2)),
                            Box::new(Expr::Factor(3)),
                        )),
                        Box::new(Expr::Plus(
                            Box::new(Expr::Factor(4)),
                            Box::new(Expr::Factor(5)),
                        )),
                    )),
                    Box::new(Expr::NotEqual(
                        Box::new(Expr::Plus(
                            Box::new(Expr::Factor(6)),
                            Box::new(Expr::Factor(7)),
                        )),
                        Box::new(Expr::Plus(
                            Box::new(Expr::Factor(8)),
                            Box::new(Expr::Factor(9)),
                        )),
                    )),
                )
            )
        }
    }

    #[test]
    fn test_mix_logical_operator() {
        {
            let data = vec![
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::LogicalOr, "||".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::LogicalAnd, "&&".to_string()),
                TokenInfo::new(Token::Number, "4".to_string()),
                TokenInfo::new(Token::LogicalOr, "||".to_string()),
                TokenInfo::new(Token::Number, "5".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
            ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::LogicalOr(
                    Box::new(Expr::LogicalAnd(
                        Box::new(Expr::LogicalOr(
                            Box::new(Expr::Factor(2)),
                            Box::new(Expr::Factor(3)),
                        )),
                        Box::new(Expr::Factor(4)),
                    )),
                    Box::new(Expr::Factor(5)),
                )
            )
        }
    }

    #[test]
    fn test_condition_expression() {
        {
            let data = vec![
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::Equal, "==".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::Question, "?".to_string()),
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::Colon, ":".to_string()),
                TokenInfo::new(Token::Number, "5".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
            ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::Condition(
                    Box::new(Expr::Equal(
                        Box::new(Expr::Factor(2)),
                        Box::new(Expr::Factor(3)),
                    )),
                    Box::new(Expr::Factor(1)),
                    Box::new(Expr::Factor(5)),
                )
            )
        }
        {
            let data = vec![
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
                TokenInfo::new(Token::SemiColon, ";".to_string()),
            ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::Condition(
                    Box::new(Expr::Equal(
                        Box::new(Expr::Factor(2)),
                        Box::new(Expr::Factor(3)),
                    )),
                    Box::new(Expr::Condition(
                        Box::new(Expr::Equal(
                            Box::new(Expr::Factor(10)),
                            Box::new(Expr::Factor(11)),
                        )),
                        Box::new(Expr::Factor(12)),
                        Box::new(Expr::Factor(13)),
                    )),
                    Box::new(Expr::Factor(5)),
                )
            )
        }
    }

    #[test]
    fn test_unary_operator() {
        {
            let data = vec![
                TokenInfo::new(Token::Plus, "+".to_string()),
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
            ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(result, Expr::UnPlus(Box::new(Expr::Factor(2))))
        }
        {
            let data = vec![
                TokenInfo::new(Token::Plus, "+".to_string()),
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::Minus, "-".to_string()),
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
            ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::Minus(
                    Box::new(Expr::UnPlus(Box::new(Expr::Factor(2)))),
                    Box::new(Expr::Factor(1)),
                )
            )
        }
        {
            let data = vec![
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::Plus, "+".to_string()),
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::Minus, "-".to_string()),
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
            ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::Minus(
                    Box::new(Expr::UnPlus(Box::new(Expr::Factor(2)))),
                    Box::new(Expr::Factor(1)),
                )
            )
        }
        {
            let data = vec![
                TokenInfo::new(Token::Plus, "+".to_string()),
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::Multi, "*".to_string()),
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
            ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::Multiple(
                    Box::new(Expr::UnPlus(Box::new(Expr::Factor(2)))),
                    Box::new(Expr::Factor(1)),
                )
            )
        }
        // 否定演算子のテスト.
        {
            let data = vec![
                TokenInfo::new(Token::Not, "!".to_string()),
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
            ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(result, Expr::Not(Box::new(Expr::Factor(2))))
        }
        {
            let data = vec![
                TokenInfo::new(Token::Not, "!".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::Equal, "==".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
            ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::Not(Box::new(Expr::Equal(
                    Box::new(Expr::Factor(2)),
                    Box::new(Expr::Factor(3)),
                )))
            )
        }
        // ビット反転演算子.
        {
            let data = vec![
                TokenInfo::new(Token::BitReverse, "~".to_string()),
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
            ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(result, Expr::BitReverse(Box::new(Expr::Factor(2))))
        }
    }

    #[test]
    fn test_shift_operator() {
        {
            let data = vec![
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::LeftShift, "<<".to_string()),
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
            ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::LeftShift(Box::new(Expr::Factor(2)), Box::new(Expr::Factor(1)))
            )
        }
        {
            let data = vec![
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::RightShift, ">>".to_string()),
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
            ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::RightShift(Box::new(Expr::Factor(2)), Box::new(Expr::Factor(1)))
            )
        }
        {
            let data = vec![
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::Plus, "+".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::RightShift, ">>".to_string()),
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
            ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::RightShift(
                    Box::new(Expr::Plus(
                        Box::new(Expr::Factor(2)),
                        Box::new(Expr::Factor(3)),
                    )),
                    Box::new(Expr::Factor(1)),
                )
            )
        }
        {
            let data = vec![
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::LessThan, "<".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::RightShift, ">>".to_string()),
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
            ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::LessThan(
                    Box::new(Expr::Factor(2)),
                    Box::new(Expr::RightShift(
                        Box::new(Expr::Factor(3)),
                        Box::new(Expr::Factor(1)),
                    )),
                )
            )
        }
    }

    // ビット演算子テスト.
    #[test]
    fn test_bit_operator() {
        {
            let data = vec![
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::BitAnd, "&".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
            ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::BitAnd(Box::new(Expr::Factor(2)), Box::new(Expr::Factor(3)))
            )
        }
        {
            let data = vec![
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::BitOr, "&".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
            ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::BitOr(Box::new(Expr::Factor(2)), Box::new(Expr::Factor(3)))
            )
        }
        {
            let data = vec![
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::BitXor, "^".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
            ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::BitXor(Box::new(Expr::Factor(2)), Box::new(Expr::Factor(3)))
            )
        }
        {
            let data = vec![
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::BitAnd, "&".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::BitOr, "|".to_string()),
                TokenInfo::new(Token::Number, "4".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
            ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::BitOr(
                    Box::new(Expr::BitAnd(
                        Box::new(Expr::Factor(2)),
                        Box::new(Expr::Factor(3)),
                    )),
                    Box::new(Expr::Factor(4)),
                )
            )
        }
    }

    #[test]
    fn test_assign_operator() {
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "a".to_string()),
                TokenInfo::new(Token::Assign, "=".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
            ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::Assign(
                    Box::new(Expr::Variable("a".to_string())),
                    Box::new(Expr::Factor(3))
                )
            )
        }
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "a".to_string()),
                TokenInfo::new(Token::Assign, "=".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::Plus, "+".to_string()),
                TokenInfo::new(Token::Number, "1".to_string()),
            ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::Assign(
                    Box::new(Expr::Variable("a".to_string())),
                    Box::new(Expr::Plus(
                        Box::new(Expr::Factor(3)),
                        Box::new(Expr::Factor(1))
                    ))
                )
            )
        }
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "a".to_string()),
                TokenInfo::new(Token::Assign, "=".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::LogicalAnd, "&&".to_string()),
                TokenInfo::new(Token::Number, "1".to_string()),
            ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::Assign(
                    Box::new(Expr::Variable("a".to_string())),
                    Box::new(Expr::LogicalAnd(
                        Box::new(Expr::Factor(3)),
                        Box::new(Expr::Factor(1))
                    ))
                )
            )
        }
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "a".to_string()),
                TokenInfo::new(Token::Assign, "=".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::Multi, "*".to_string()),
                TokenInfo::new(Token::Number, "1".to_string()),
            ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::Assign(
                    Box::new(Expr::Variable("a".to_string())),
                    Box::new(Expr::Multiple(
                        Box::new(Expr::Factor(3)),
                        Box::new(Expr::Factor(1))
                    ))
                )
            )
        }
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "a".to_string()),
                TokenInfo::new(Token::Assign, "=".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::BitOr, "|".to_string()),
                TokenInfo::new(Token::Number, "1".to_string()),
            ];
            let mut ast = Ast::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result,
                Expr::Assign(
                    Box::new(Expr::Variable("a".to_string())),
                    Box::new(Expr::BitOr(
                        Box::new(Expr::Factor(3)),
                        Box::new(Expr::Factor(1))
                    ))
                )
            )
        }
    }
}
