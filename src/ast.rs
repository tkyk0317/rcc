use std::fmt;
use token::TokenInfo;
use token::Token;
use symbol::SymbolTable;

// 文法.
//   <FuncDef> ::= VARIABLE '()' <FuncBody>
//   <FuncBody> ::= '{' <Statement> '}'
//   <Statement> ::= <Assign>* ';'
//   <Assign> ::= VARIABLE '=' <Condition>
//   <CallFunc> ::= VARIABLE '(' <Argment> ')'
//   <Argment> ::= [ '' |  <Expression> ',']
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
//   <Factor> ::= '(' NUMBER ')' | <UnAry> | <Expression> | <CallFunc>
//   <UnAry> ::= ['!'|'+'|'-'|'~'] NUMBER

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    FuncDef(Box<Expr>),
    Statement(Vec<Expr>),
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
    CallFunc(Box<Expr>, Box<Expr>),
    Argment(Vec<Expr>),
}

// 出力フォーマット定義.
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expr::FuncDef(ref a) => write!(f, "{}", *a),
            Expr::Statement(ref a) => write!(f, "{:?}", a),
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
            Expr::CallFunc(ref v, ref a) => write!(f, "{}({})", v, *a),
            Expr::Argment(ref v) => write!(f, "{:?}", v),
        }
    }
}

#[derive(Debug)]
pub struct AstGen<'a> {
    tokens: &'a Vec<TokenInfo>, // トークン配列.
    current_pos: usize, // 現在読み取り位置.
    s_table: SymbolTable, // シンボルテーブル.
}

pub struct AstTree {
    tree: Vec<Expr>,  // 抽象構文木.
}

// 抽象構文木.
impl AstTree {
    // コンストラクタ.
    fn new (tree: Vec<Expr>) -> Self {
        AstTree { tree: tree }
    }

    // 抽象構文木取得.
    pub fn get_tree(&self) -> &Vec<Expr> {
        &self.tree
    }
}

// 抽象構文木をトークン列から作成する
impl<'a> AstGen<'a> {
    // コンストラクタ.
    pub fn new(tokens: &'a Vec<TokenInfo>) -> AstGen<'a> {
        AstGen {
            current_pos: 0,
            tokens: tokens,
            s_table: SymbolTable::new(),
        }
    }

    // シンボルテーブル取得.
    pub fn get_symbol_table(&self) -> &SymbolTable {
        &self.s_table
    }

    // トークン列を受け取り、抽象構文木を返す.
    pub fn parse(&mut self) -> AstTree {
        AstTree::new(
            vec![self.func_def()]
        )
    }

    // func def.
    fn func_def(&mut self) -> Expr {
        // 関数定義から始まらないとだめ.
        let token= self.next_consume();
        match token.get_token_type() {
            Token::Variable => {
                if Token::LeftBracket != self.next_consume().get_token_type() {
                    panic!("ast.rs(func_def): Not Exists Left Bracket")
                }
                if Token::RightBracket != self.next_consume().get_token_type() {
                    panic!("ast.rs(func_def): Not Exists Right Bracket")
                }
                Expr::FuncDef(Box::new(self.statement()))
            }
            _ => panic!("ast.rs(func_def): Not Exists Function def")
        }
    }

    // statement.
    fn statement(&mut self) -> Expr {
        Expr::Statement(self.sub_statement(&vec![]))
    }

    // sub statement.
    fn sub_statement(&mut self, expr: &Vec<Expr>) -> Vec<Expr> {
        // トークンがなくなるまで、構文木生成.
        let mut stmt = expr.clone();
        let token = self.next();
        match token.get_token_type() {
            Token::LeftBrace => {
                self.consume();
                self.sub_statement(&stmt)
            }
            Token::RightBrace => expr.clone(),
            Token::SemiColon => {
                self.consume();
                self.sub_statement(&stmt)
            }
            Token::Unknown => expr.clone(),
            _ => {
                stmt.push(self.assign());
                self.sub_statement(&stmt)
            }
        }
    }

    // assign.
    fn assign(&mut self) -> Expr {
        let left = self.next();
        match left.get_token_type() {
            Token::Variable => {
                // 代入演算子判定.
                let var = self.factor();
                match self.next().get_token_type() {
                    Token::Assign => {
                        self.consume();
                        Expr::Assign(Box::new(var), Box::new(self.condition()))
                    }
                    Token::LeftBracket => self.call_func(var),
                    _ =>{
                        // variable分を巻き戻し.
                        self.back(1);
                        self.condition()
                    }
                }
            }
            _ => self.condition(),
        }
    }

    // func call.
    fn call_func(&mut self, acc: Expr) -> Expr {
        if Token::LeftBracket == self.next_consume().get_token_type()
        {
            let call_func = Expr::CallFunc(
                Box::new(acc),
                Box::new(self.argment(Expr::Argment(vec![])))
            );
            if Token::RightBracket != self.next_consume().get_token_type() {
                panic!("ast.rs(call_func): Not exists RightBracket")
            }
            return call_func;
        }
        panic!("ast.rs(call_func): Not exists LeftBracket")
    }

    // argment.
    fn argment(&mut self, acc: Expr) -> Expr {
        // 右括弧が表れるまで、引数とみなす
        let token = self.next();
        match token.get_token_type() {
            Token::RightBracket => acc,
            Token::Variable | Token::Number => {
                match acc {
                    Expr::Argment(a) => {
                        let mut args = a;

                        if Token::Variable == token.get_token_type() {
                            args.push(Expr::Variable(token.get_token_value()));
                        } else {
                            args.push(self.number(token));
                        }
                        self.next_consume();

                        // カンマがあれば引き続き、引数とみなす.
                        if Token::Comma == self.next().get_token_type() {
                            self.next_consume();
                            self.argment(Expr::Argment(args))
                        } else {
                            Expr::Argment(args)
                        }
                    }
                    _ => panic!("ast.rs(argment): Error")
                }
            }
            _ => acc
        }
    }

    // condition.
    fn condition(&mut self) -> Expr {
        let left = self.logical();
        self.sub_condition(left)
    }

    // sub condition.
    fn sub_condition(&mut self, acc: Expr) -> Expr {
        let ope_type = self.next().get_token_type();
        match ope_type {
            Token::Question => {
                self.consume();
                let middle = self.logical();

                // コロンがない場合、終了.
                if self.next_consume().get_token_type() != Token::Colon {
                    panic!("Not Exists Colon")
                } else {
                    let right = self.logical();
                    let tree = Expr::Condition(Box::new(acc), Box::new(middle), Box::new(right));
                    self.sub_condition(tree)
                }
            }
            _ => acc,
        }
    }

    // logical.
    fn logical(&mut self) -> Expr {
        let left = self.bit_operator();
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
                let right = self.bit_operator();
                self.sub_logical(create(ope_type, acc, right))
            }
            _ => acc,
        }
    }

    // bit operator.
    fn bit_operator(&mut self) -> Expr {
        let left = self.relation();
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
                let right = self.relation();
                self.sub_bit_operator(create(token.get_token_type(), acc, right))
            }
            _ => acc,
        }
    }

    // relation.
    fn relation(&mut self) -> Expr {
        let left = self.shift();
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
                let right = self.shift();
                self.sub_relation(create(ope_type, acc, right))
            }
            _ => acc,
        }
    }

    // shift operation.
    fn shift(&mut self) -> Expr {
        let left = self.expr();
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
                let right = self.expr();
                self.sub_shift(create(token.get_token_type(), acc, right))
            }
            _ => acc,
        }
    }

    // expression
    fn expr(&mut self) -> Expr {
        let left = self.term();
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
                let right = self.term();
                self.expr_add_sub(create(ope.get_token_type(), acc, right))
            }
            _ => acc,
        }
    }

    // term.
    fn term(&mut self) -> Expr {
        let left = self.factor();
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
                let right = self.factor();
                self.term_multi_div(create(ope.get_token_type(), acc, right))
            }
            _ => acc,
        }
    }

    // factor.
    fn factor(&mut self) -> Expr {
        let token = self.next();
        match token.get_token_type() {
            Token::Number => {
                self.consume();
                self.number(token)
            }
            Token::LeftBracket => {
                self.consume();
                let tree = self.assign();

                // 閉じカッコがあるかどうかチェック.
                if Token::RightBracket != self.next_consume().get_token_type() {
                    panic!("Not Exists Right Bracket")
                }
                tree
            }
            Token::Plus => {
                self.consume();
                Expr::UnPlus(Box::new(self.factor()))
            }
            Token::Minus => {
                self.consume();
                Expr::UnMinus(Box::new(self.factor()))
            }
            Token::Not => {
                self.consume();
                Expr::Not(Box::new(self.factor()))
            }
            Token::BitReverse => {
                self.consume();
                Expr::BitReverse(Box::new(self.factor()))
            }
            Token::Variable => {
                // シンボルテーブルへ保存.
                self.s_table.push(token.get_token_value(), "".to_string());
                self.consume();
                Expr::Variable(token.get_token_value())
            }
            _ => panic!("ast.rs: failed in factor {:?}", token),
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
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::Plus, '+'.to_string()),
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![
                            Expr::Plus(Box::new(Expr::Factor(1)), Box::new(Expr::Factor(2)))
                        ]
                    ))
                )
            )
        }
        // 複数の加算テスト.
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, '('.to_string()),
                TokenInfo::new(Token::RightBracket, ')'.to_string()),
                TokenInfo::new(Token::LeftBrace, '{'.to_string()),
                TokenInfo::new(Token::Number, '1'.to_string()),
                TokenInfo::new(Token::Plus, '+'.to_string()),
                TokenInfo::new(Token::Number, '2'.to_string()),
                TokenInfo::new(Token::Plus, '+'.to_string()),
                TokenInfo::new(Token::Number, '3'.to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, '}'.to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![
                            Expr::Plus(
                                Box::new(Expr::Plus(
                                    Box::new(Expr::Factor(1)),
                                    Box::new(Expr::Factor(2)),
                                )),
                                Box::new(Expr::Factor(3))
                            )
                        ]
                    ))
                )
            )
        }
        // 複数の加算テスト.
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Number, '1'.to_string()),
                TokenInfo::new(Token::Plus, '+'.to_string()),
                TokenInfo::new(Token::Number, '2'.to_string()),
                TokenInfo::new(Token::Plus, '+'.to_string()),
                TokenInfo::new(Token::Number, '3'.to_string()),
                TokenInfo::new(Token::Plus, '+'.to_string()),
                TokenInfo::new(Token::Number, '4'.to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![
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
                        ]
                    ))
                )
            )
        }
    }

    #[test]
    fn test_sub_operator() {
        // 単純な減算テスト.
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::Minus, '-'.to_string()),
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![Expr::Minus(Box::new(Expr::Factor(1)), Box::new(Expr::Factor(2)))]
                    )
                ))
            )
        }
        // 複数の減算テスト.
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Number, "100".to_string()),
                TokenInfo::new(Token::Minus, '-'.to_string()),
                TokenInfo::new(Token::Number, '2'.to_string()),
                TokenInfo::new(Token::Minus, '-'.to_string()),
                TokenInfo::new(Token::Number, '3'.to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![
                            Expr::Minus(
                                Box::new(Expr::Minus(
                                    Box::new(Expr::Factor(100)),
                                    Box::new(Expr::Factor(2)),
                                )),
                                Box::new(Expr::Factor(3)),
                            )
                        ]
                    ))
                )
            )
        }
        // 複数の減算テスト.
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Number, '1'.to_string()),
                TokenInfo::new(Token::Minus, '-'.to_string()),
                TokenInfo::new(Token::Number, '2'.to_string()),
                TokenInfo::new(Token::Minus, '-'.to_string()),
                TokenInfo::new(Token::Number, '3'.to_string()),
                TokenInfo::new(Token::Minus, '-'.to_string()),
                TokenInfo::new(Token::Number, '4'.to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "{".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![
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
                        ]
                    ))
                )
            )
        }
    }

    #[test]
    fn test_mul_operator() {
        // 単純な乗算テスト.
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::Multi, '*'.to_string()),
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![Expr::Multiple(Box::new(Expr::Factor(1)), Box::new(Expr::Factor(2)))]
                    ))
                )
            )
        }
        // 複数の減算テスト.
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Number, '1'.to_string()),
                TokenInfo::new(Token::Multi, '*'.to_string()),
                TokenInfo::new(Token::Number, '2'.to_string()),
                TokenInfo::new(Token::Multi, '*'.to_string()),
                TokenInfo::new(Token::Number, '3'.to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![
                            Expr::Multiple(
                                Box::new(Expr::Multiple(
                                    Box::new(Expr::Factor(1)),
                                    Box::new(Expr::Factor(2)),
                                )),
                                Box::new(Expr::Factor(3)),
                            )
                        ]
                    ))
                )
           )
        }
        // 複数の減算テスト.
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Number, '1'.to_string()),
                TokenInfo::new(Token::Multi, '*'.to_string()),
                TokenInfo::new(Token::Number, '2'.to_string()),
                TokenInfo::new(Token::Multi, '*'.to_string()),
                TokenInfo::new(Token::Number, '3'.to_string()),
                TokenInfo::new(Token::Multi, '*'.to_string()),
                TokenInfo::new(Token::Number, '4'.to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![
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
                        ]
                    ))
                )
            )
        }
    }

    #[test]
    fn test_div_operator() {
        // 単純な乗算テスト.
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::Division, '/'.to_string()),
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![Expr::Division(Box::new(Expr::Factor(1)), Box::new(Expr::Factor(2)))]
                    ))
                )
            )
        }
        // 複数の減算テスト.
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Number, '1'.to_string()),
                TokenInfo::new(Token::Division, '/'.to_string()),
                TokenInfo::new(Token::Number, '2'.to_string()),
                TokenInfo::new(Token::Division, '/'.to_string()),
                TokenInfo::new(Token::Number, '3'.to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![
                            Expr::Division(
                                Box::new(Expr::Division(
                                    Box::new(Expr::Factor(1)),
                                    Box::new(Expr::Factor(2)),
                                )),
                                Box::new(Expr::Factor(3)),
                            )
                        ]
                    ))
                )
            )
        }
        // 複数の減算テスト.
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Number, '1'.to_string()),
                TokenInfo::new(Token::Division, '/'.to_string()),
                TokenInfo::new(Token::Number, '2'.to_string()),
                TokenInfo::new(Token::Division, '/'.to_string()),
                TokenInfo::new(Token::Number, '3'.to_string()),
                TokenInfo::new(Token::Division, '/'.to_string()),
                TokenInfo::new(Token::Number, '4'.to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![
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
                        ]
                    ))
                )
            )
        }
    }

    #[test]
    fn test_mix_operator() {
        // 複数演算子のテスト.
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Number, '1'.to_string()),
                TokenInfo::new(Token::Multi, '*'.to_string()),
                TokenInfo::new(Token::Number, '2'.to_string()),
                TokenInfo::new(Token::Plus, '+'.to_string()),
                TokenInfo::new(Token::Number, '3'.to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![
                            Expr::Plus(
                                Box::new(Expr::Multiple(
                                    Box::new(Expr::Factor(1)),
                                    Box::new(Expr::Factor(2)),
                                )),
                                Box::new(Expr::Factor(3)),
                            )
                        ]
                    ))
                )
            )
        }
        // 複数演算子のテスト.
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Number, '1'.to_string()),
                TokenInfo::new(Token::Plus, '+'.to_string()),
                TokenInfo::new(Token::Number, '2'.to_string()),
                TokenInfo::new(Token::Multi, '*'.to_string()),
                TokenInfo::new(Token::Number, '3'.to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "{".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![
                            Expr::Plus(
                                Box::new(Expr::Factor(1)),
                                Box::new(Expr::Multiple(
                                    Box::new(Expr::Factor(2)),
                                    Box::new(Expr::Factor(3)),
                                )),
                            )
                        ]
                    ))
                )
            )
        }
        // 複数演算子のテスト.
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Number, '1'.to_string()),
                TokenInfo::new(Token::Division, '/'.to_string()),
                TokenInfo::new(Token::Number, '2'.to_string()),
                TokenInfo::new(Token::Plus, '+'.to_string()),
                TokenInfo::new(Token::Number, '3'.to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::LeftBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![
                            Expr::Plus(
                                Box::new(Expr::Division(
                                    Box::new(Expr::Factor(1)),
                                    Box::new(Expr::Factor(2)),
                                )),
                                Box::new(Expr::Factor(3)),
                            )
                        ]
                    ))
                )
            )
        }
        // 複数演算子のテスト.
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Number, '1'.to_string()),
                TokenInfo::new(Token::Plus, '+'.to_string()),
                TokenInfo::new(Token::Number, '2'.to_string()),
                TokenInfo::new(Token::Division, '/'.to_string()),
                TokenInfo::new(Token::Number, '3'.to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![
                            Expr::Plus(
                                Box::new(Expr::Factor(1)),
                                Box::new(Expr::Division(
                                    Box::new(Expr::Factor(2)),
                                    Box::new(Expr::Factor(3)),
                                )),
                            )
                        ]
                    ))
                )
            )
        }
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::LessThan, "<".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::Equal, "==".to_string()),
                TokenInfo::new(Token::Number, "4".to_string()),
                TokenInfo::new(Token::GreaterThanEqual, ">=".to_string()),
                TokenInfo::new(Token::Number, "5".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![
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
                        ]
                    ))
                )
            )
        }
    }

    #[test]
    fn test_bracket() {
        // カッコのテスト.
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::Plus, '+'.to_string()),
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![
                            Expr::Plus(Box::new(Expr::Factor(1)), Box::new(Expr::Factor(2)))
                        ]
                    ))
                )
            )
        }
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::Plus, '+'.to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::Plus, '+'.to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![
                            Expr::Plus(
                                Box::new(Expr::Factor(1)),
                                Box::new(Expr::Plus(
                                    Box::new(Expr::Factor(2)),
                                    Box::new(Expr::Factor(3)),
                                )),
                            )
                        ]
                    ))
                )
            )
        }
    }

    #[test]
    fn test_equal_operator() {
        // 等価演算子テスト.
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::Plus, '+'.to_string()),
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::Equal, "==".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::Plus, '+'.to_string()),
                TokenInfo::new(Token::Number, "4".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![
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
                        ]
                    ))
                )
            )
        }
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::Multi, '*'.to_string()),
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::Equal, "==".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::Multi, '*'.to_string()),
                TokenInfo::new(Token::Number, "4".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![
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
                        ]
                    ))
                )

            )
        }
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
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
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![
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
                        ]
                    ))
                )
            )
        }
    }

    #[test]
    fn test_not_equal_operator() {
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
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
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![
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
                        ]
                    ))
                )
            )
        }
    }

    #[test]
    fn test_less_than_operator() {
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
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
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![
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
                        ]
                    ))
                )
            )
        }
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
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
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![
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
                        ]
                    ))
                )
            )
        }
    }

    #[test]
    fn test_greater_than_operator() {
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
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
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![
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
                        ]
                    ))
                )
            )
        }
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
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
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![
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
                        ]
                    ))
                )
            )
        }
    }

    #[test]
    fn test_logical_operator() {
        // &&演算子のテスト.
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::LogicalAnd, "&&".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![Expr::LogicalAnd(Box::new(Expr::Factor(2)), Box::new(Expr::Factor(3)))]
                    ))
                )
            )
        }
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::Plus, "+".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::LogicalAnd, "&&".to_string()),
                TokenInfo::new(Token::Number, "4".to_string()),
                TokenInfo::new(Token::Plus, "+".to_string()),
                TokenInfo::new(Token::Number, "5".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![
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
                        ]
                    ))
                )
            )
        }
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
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
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![
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
                        ]
                    ))
                )
            )
        }
        // ||演算子のテスト.
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::LogicalOr, "||".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![Expr::LogicalOr(Box::new(Expr::Factor(2)), Box::new(Expr::Factor(3)))]
                    ))
                )
            )
        }
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::Plus, "+".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::LogicalOr, "||".to_string()),
                TokenInfo::new(Token::Number, "4".to_string()),
                TokenInfo::new(Token::Plus, "+".to_string()),
                TokenInfo::new(Token::Number, "5".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![
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
                        ]
                    ))
                )
            )
        }
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
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
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![
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
                        ]
                    ))
                )
            )
        }
    }

    #[test]
    fn test_mix_logical_operator() {
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::LogicalOr, "||".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::LogicalAnd, "&&".to_string()),
                TokenInfo::new(Token::Number, "4".to_string()),
                TokenInfo::new(Token::LogicalOr, "||".to_string()),
                TokenInfo::new(Token::Number, "5".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![
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
                        ]
                    ))
                )
            )
        }
    }

    #[test]
    fn test_condition_expression() {
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::Equal, "==".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::Question, "?".to_string()),
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::Colon, ":".to_string()),
                TokenInfo::new(Token::Number, "5".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![
                            Expr::Condition(
                                Box::new(Expr::Equal(
                                    Box::new(Expr::Factor(2)),
                                    Box::new(Expr::Factor(3)),
                                )),
                                Box::new(Expr::Factor(1)),
                                Box::new(Expr::Factor(5)),
                            )
                        ]
                    ))
                )
            )
        }
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
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
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![
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
                        ]
                    ))
                )
            )
        }
    }

    #[test]
    fn test_unary_operator() {
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Plus, "+".to_string()),
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![Expr::UnPlus(Box::new(Expr::Factor(2)))]
                    ))
                )
            )
        }
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Plus, "+".to_string()),
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::Minus, "-".to_string()),
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![
                            Expr::Minus(
                                Box::new(Expr::UnPlus(Box::new(Expr::Factor(2)))),
                                Box::new(Expr::Factor(1)),
                            )
                        ]
                    ))
                )
            )
        }
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::Plus, "+".to_string()),
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::Minus, "-".to_string()),
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![
                            Expr::Minus(
                                Box::new(Expr::UnPlus(Box::new(Expr::Factor(2)))),
                                Box::new(Expr::Factor(1)),
                            )
                        ]
                    ))
                )
            )
        }
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Plus, "+".to_string()),
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::Multi, "*".to_string()),
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![
                            Expr::Multiple(
                                Box::new(Expr::UnPlus(Box::new(Expr::Factor(2)))),
                                Box::new(Expr::Factor(1)),
                            )
                        ]
                    ))
                )
            )
        }
        // 否定演算子のテスト.
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Not, "!".to_string()),
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![Expr::Not(Box::new(Expr::Factor(2)))]
                    ))
                )
            )
        }
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Not, "!".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::Equal, "==".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![
                            Expr::Not(Box::new(Expr::Equal(
                                Box::new(Expr::Factor(2)),
                                Box::new(Expr::Factor(3)),
                            )))
                        ]
                    ))
                )
            )
        }
        // ビット反転演算子.
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::BitReverse, "~".to_string()),
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![Expr::BitReverse(Box::new(Expr::Factor(2)))]
                    ))
                )
            )
        }
    }

    #[test]
    fn test_shift_operator() {
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::LeftShift, "<<".to_string()),
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![ Expr::LeftShift(Box::new(Expr::Factor(2)), Box::new(Expr::Factor(1))) ]
                    ))
                )
            )
        }
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::RightShift, ">>".to_string()),
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![Expr::RightShift(Box::new(Expr::Factor(2)), Box::new(Expr::Factor(1)))]
                    ))
                )
            )
        }
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::Plus, "+".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::RightShift, ">>".to_string()),
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![
                            Expr::RightShift(
                                Box::new(Expr::Plus(
                                    Box::new(Expr::Factor(2)),
                                    Box::new(Expr::Factor(3)),
                                )),
                                Box::new(Expr::Factor(1)),
                            )
                        ]
                    ))
                )
            )
        }
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::LessThan, "<".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::RightShift, ">>".to_string()),
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![
                            Expr::LessThan(
                                Box::new(Expr::Factor(2)),
                                Box::new(Expr::RightShift(
                                    Box::new(Expr::Factor(3)),
                                    Box::new(Expr::Factor(1)),
                                )),
                            )
                        ]
                    ))
                )
            )
        }
    }

    // ビット演算子テスト.
    #[test]
    fn test_bit_operator() {
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::BitAnd, "&".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![Expr::BitAnd(Box::new(Expr::Factor(2)), Box::new(Expr::Factor(3)))]
                    ))
                )
            )
        }
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::BitOr, "&".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![
                            Expr::BitOr(Box::new(Expr::Factor(2)), Box::new(Expr::Factor(3)))
                        ]
                    ))
                )
            )
        }
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::BitXor, "^".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![ Expr::BitXor(Box::new(Expr::Factor(2)), Box::new(Expr::Factor(3))) ]
                    ))
                )
            )
        }
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Number, "2".to_string()),
                TokenInfo::new(Token::BitAnd, "&".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::BitOr, "|".to_string()),
                TokenInfo::new(Token::Number, "4".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![
                            Expr::BitOr(
                                Box::new(Expr::BitAnd(
                                    Box::new(Expr::Factor(2)),
                                    Box::new(Expr::Factor(3)),
                                )),
                                Box::new(Expr::Factor(4)),
                            )
                        ]
                    ))
                )
            )
        }
    }

    #[test]
    fn test_assign_operator() {
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Variable, "a".to_string()),
                TokenInfo::new(Token::Assign, "=".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![
                            Expr::Assign(
                                Box::new(Expr::Variable("a".to_string())),
                                Box::new(Expr::Factor(3)),
                            )
                        ]
                    ))
                )
            )
        }
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Variable, "a".to_string()),
                TokenInfo::new(Token::Assign, "=".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::Plus, "+".to_string()),
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![
                            Expr::Assign(
                                Box::new(Expr::Variable("a".to_string())),
                                Box::new(Expr::Plus(
                                    Box::new(Expr::Factor(3)),
                                    Box::new(Expr::Factor(1)),
                                )),
                            )
                        ]
                    ))
                )
            )
        }
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Variable, "a".to_string()),
                TokenInfo::new(Token::Assign, "=".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::LogicalAnd, "&&".to_string()),
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![
                            Expr::Assign(
                                Box::new(Expr::Variable("a".to_string())),
                                Box::new(Expr::LogicalAnd(
                                    Box::new(Expr::Factor(3)),
                                    Box::new(Expr::Factor(1)),
                                )),
                            )
                        ]
                    ))
                )
            )
        }
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Variable, "a".to_string()),
                TokenInfo::new(Token::Assign, "=".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::Multi, "*".to_string()),
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![
                            Expr::Assign(
                                Box::new(Expr::Variable("a".to_string())),
                                Box::new(Expr::Multiple(
                                    Box::new(Expr::Factor(3)),
                                    Box::new(Expr::Factor(1)),
                                )),
                            )
                        ]
                    ))
                )
            )
        }
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Variable, "a".to_string()),
                TokenInfo::new(Token::Assign, "=".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::BitOr, "|".to_string()),
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![
                            Expr::Assign(
                                Box::new(Expr::Variable("a".to_string())),
                                Box::new(Expr::BitOr(
                                    Box::new(Expr::Factor(3)),
                                    Box::new(Expr::Factor(1)),
                                )),
                            )
                        ]
                    ))
                )
            )
        }
    }

    #[test]
    fn test_call_func() {
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Variable, "a".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![
                            Expr::CallFunc(Box::new(Expr::Variable("a".to_string())), Box::new(Expr::Argment(vec![])))
                        ]
                    ))
                )
            )
        }
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Variable, "a".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::Variable, "b".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![
                            Expr::CallFunc(
                                Box::new(Expr::Variable("a".to_string())),
                                Box::new(Expr::Argment(vec![Expr::Variable('b'.to_string())]))
                            )
                        ]
                    ))
                )
            )
        }
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Variable, "a".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::Variable, "b".to_string()),
                TokenInfo::new(Token::Comma, ",".to_string()),
                TokenInfo::new(Token::Variable, "c".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![
                            Expr::CallFunc(
                                Box::new(Expr::Variable("a".to_string())),
                                Box::new(Expr::Argment(vec![Expr::Variable('b'.to_string()), Expr::Variable('c'.to_string())]))
                            )
                        ]
                    ))
                )
            )
        }
    }

    #[test]
    fn test_compound() {
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Variable, "a".to_string()),
                TokenInfo::new(Token::Assign, "=".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::Variable, "a".to_string()),
                TokenInfo::new(Token::Assign, "=".to_string()),
                TokenInfo::new(Token::Variable, "a".to_string()),
                TokenInfo::new(Token::Plus, "+".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "{".to_string()),
             ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![
                            Expr::Assign(
                                Box::new(Expr::Variable("a".to_string())),
                                Box::new(Expr::Factor(3)),
                            ),
                            Expr::Assign(
                                Box::new(Expr::Variable("a".to_string())),
                                Box::new(Expr::Plus(
                                    Box::new(Expr::Variable("a".to_string())),
                                    Box::new(Expr::Factor(3))
                                ))
                            )
                        ]
                    ))
                )
            );
         }
         {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Variable, "a".to_string()),
                TokenInfo::new(Token::Assign, "=".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::Variable, "a".to_string()),
                TokenInfo::new(Token::Multi, "*".to_string()),
                TokenInfo::new(Token::Variable, "a".to_string()),
                TokenInfo::new(Token::Plus, "+".to_string()),
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                Expr::FuncDef(
                    Box::new(Expr::Statement(
                        vec![
                            Expr::Assign(
                                Box::new(Expr::Variable("a".to_string())),
                                Box::new(Expr::Factor(3)),
                            ),
                            Expr::Plus(
                                Box::new(Expr::Multiple(
                                    Box::new(Expr::Variable("a".to_string())),
                                    Box::new(Expr::Variable("a".to_string()))
                                )),
                                Box::new(Expr::Factor(1))
                            )
                        ]
                    ))
                )
            )
         }
    }
}
