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
pub enum AstType {
    FuncDef(String, Box<AstType>, Box<AstType>),
    Statement(Vec<AstType>),
    While(Box<AstType>, Box<AstType>), // 条件式、ブロック部.
    If(Box<AstType>, Box<AstType>, Box<Option<AstType>>), // 条件式、真ブロック、偽ブロック.
    For(Box<Option<AstType>>, Box<Option<AstType>>, Box<Option<AstType>>, Box<AstType>), // 初期条件、終了条件、更新部、ブロック部.
    Condition(Box<AstType>, Box<AstType>, Box<AstType>),
    LogicalAnd(Box<AstType>, Box<AstType>),
    LogicalOr(Box<AstType>, Box<AstType>),
    BitAnd(Box<AstType>, Box<AstType>),
    BitOr(Box<AstType>, Box<AstType>),
    BitXor(Box<AstType>, Box<AstType>),
    Equal(Box<AstType>, Box<AstType>),
    NotEqual(Box<AstType>, Box<AstType>),
    LessThan(Box<AstType>, Box<AstType>),
    GreaterThan(Box<AstType>, Box<AstType>),
    LessThanEqual(Box<AstType>, Box<AstType>),
    GreaterThanEqual(Box<AstType>, Box<AstType>),
    Plus(Box<AstType>, Box<AstType>),
    Minus(Box<AstType>, Box<AstType>),
    LeftShift(Box<AstType>, Box<AstType>),
    RightShift(Box<AstType>, Box<AstType>),
    Multiple(Box<AstType>, Box<AstType>),
    Division(Box<AstType>, Box<AstType>),
    Remainder(Box<AstType>, Box<AstType>),
    UnPlus(Box<AstType>),
    UnMinus(Box<AstType>),
    Not(Box<AstType>),
    BitReverse(Box<AstType>),
    Assign(Box<AstType>, Box<AstType>),
    Factor(i64),
    Variable(String),
    CallFunc(Box<AstType>, Box<AstType>),
    Argment(Vec<AstType>),
}

impl AstType {
    // 式判定.
    pub fn is_expr(&self) -> bool {
        match self {
            AstType::If(_, _, _) |
            AstType::For(_, _, _, _) |
            AstType::While(_, _) => false,
            _ => true,
        }
    }
}

#[derive(Debug)]
pub struct AstGen<'a> {
    tokens: &'a Vec<TokenInfo>, // トークン配列.
    current_pos: usize, // 現在読み取り位置.
    var_table: SymbolTable, // シンボルテーブル.
    func_table: SymbolTable, // 関数シンボルテーブル.
}

pub struct AstTree {
    tree: Vec<AstType>, // 抽象構文木.
}

// 抽象構文木.
impl AstTree {
    // コンストラクタ.
    fn new(tree: Vec<AstType>) -> Self {
        AstTree { tree: tree }
    }

    // 抽象構文木取得.
    pub fn get_tree(&self) -> &Vec<AstType> {
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
            var_table: SymbolTable::new(),
            func_table: SymbolTable::new(),
        }
    }

    // シンボルテーブル取得.
    pub fn get_var_symbol_table(&self) -> &SymbolTable {
        &self.var_table
    }

    // 関数シンボルテーブル取得.
    pub fn get_func_symbol_table(&self) -> &SymbolTable {
        &self.func_table
    }

    // トークン列を受け取り、抽象構文木を返す.
    pub fn parse(&mut self) -> AstTree {
        let mut s = vec![];
        while self.next().get_token_type() != Token::End {
            let expr = self.func_def();
            s.push(expr);
        }
        AstTree::new(s)
    }

    // func def.
    fn func_def(&mut self) -> AstType {
        // 関数定義から始まらないとだめ（関数の中に様々な処理が入っている）.
        let token = self.next_consume();
        match token.get_token_type() {
            Token::Variable => {
                // 既に同じシンボルが登録されていればエラー.
                if None != self.func_table.search(&token.get_token_value()) {
                    panic!("ast.rs(func_def): already define {}", token.get_token_value());
                }

                // 関数シンボルを登録.
                self.func_table.push(
                    token.get_token_value(),
                    token.get_token_value().to_string(),
                );
                AstType::FuncDef(
                    token.get_token_value(),
                    Box::new(self.func_args()),
                    Box::new(self.statement()),
                )
            }
            _ => panic!("ast.rs(func_def): Not Exists Function def {:?}", token),
        }
    }

    // func argment.
    fn func_args(&mut self) -> AstType {
        let token = self.next_consume();
        match token.get_token_type() {
            Token::LeftBracket => {
                // 引数を処理.
                let tmp = vec![];
                let args = AstType::Argment(self.recur_func_args(tmp));

                // 閉じ括弧.
                let next_token = self.next_consume();
                self.panic_token(&next_token, Token::RightBracket, format!("ast.rs(func_arg): Not Exists RightBracket {:?}", next_token));
                args
            }
            _ => panic!("ast.rs(func_arg): Not Exists LeftBracket {:?}", token),
        }
    }

    // recur func argment.
    fn recur_func_args(&mut self, a: Vec<AstType>) -> Vec<AstType> {
        let token = self.next();
        match token.get_token_type() {
            Token::Variable => {
                // シンボルを登録し、引数vec生成.
                self.consume();
                self.var_table.push(token.get_token_value(), "".to_string());
                let mut args = a;
                args.push(AstType::Variable(token.get_token_value()));

                // カンマがあれば引き続き.
                let comma = self.next();
                if Token::Comma == comma.get_token_type() {
                    self.consume();
                    return self.recur_func_args(args);
                }
                return args.clone();
            }
            _ => a.clone(),
        }
    }

    // statement.
    fn statement(&mut self) -> AstType {
        AstType::Statement(self.sub_statement(&vec![]))
    }

    // sub statement.
    fn sub_statement(&mut self, expr: &Vec<AstType>) -> Vec<AstType> {
        // トークンがなくなるまで、構文木生成.
        let mut stmt = expr.clone();
        let token = self.next_consume();
        match token.get_token_type() {
            Token::If => {
                stmt.push(self.statement_if());
                self.sub_statement(&stmt)
            }
            Token::While => {
                stmt.push(self.statement_while());
                self.sub_statement(&stmt)
            }
            Token::For => {
                stmt.push(self.statement_for());
                self.sub_statement(&stmt)
            }
            Token::SemiColon => self.sub_statement(&stmt),
            Token::LeftBrace => self.sub_statement(&stmt),
            Token::RightBrace =>  stmt,
            _ => {
                self.back(1);
                stmt.push(self.assign());
                self.sub_statement(&stmt)
            }
        }
    }

    // if statement.
    fn statement_if(&mut self) -> AstType {
        let next_l = self.next_consume();
        self.panic_token(&next_l, Token::LeftBracket, format!("ast.rs(statement_if): Not Exists LeftBracket {:?}", next_l));

        // 条件式を解析.
        let condition = self.assign();
        let next_r = self.next_consume();
        self.panic_token(&next_r, Token::RightBracket, format!("ast.rs(statement_if): Not Exists RightBracket {:?}", next_r));

        // ifブロック内を解析.
        let stmt = self.statement();

        // else部分解析.
        if Token::Else == self.next().get_token_type() {
            self.consume();
            AstType::If(Box::new(condition), Box::new(stmt), Box::new(Some(self.statement())))
        }
        else {
            AstType::If(Box::new(condition), Box::new(stmt), Box::new(None))
        }
    }

    // while statement.
    fn statement_while(&mut self) -> AstType {
        let next_l = self.next_consume();
        self.panic_token(&next_l, Token::LeftBracket, format!("ast.rs(statement_while): Not Exists LeftBracket {:?}", next_l));

        // 条件式を解析.
        let condition = self.assign();
        let next_r = self.next_consume();
        self.panic_token(&next_r, Token::RightBracket, format!("ast.rs(statement_while): Not Exists RightBracket {:?}", next_r));

        AstType::While(Box::new(condition), Box::new(self.statement()))
    }

    // for statement.
    fn statement_for(&mut self) -> AstType {
        let l_bracket = self.next_consume();
        self.panic_token(&l_bracket, Token::LeftBracket, format!("ast.rs(statement_for): Not Exists LeftBracket {:?}", l_bracket));

        // 各種条件を解析.
        let begin = if Token::SemiColon == self.next().get_token_type() { None } else { Some(self.assign()) };
        let mut semi_colon = self.next_consume();
        self.panic_token(&semi_colon, Token::SemiColon, format!("ast.rs(statement_for): Not Exists Semicolon {:?}", semi_colon));

        let condition = if Token::SemiColon == self.next().get_token_type() { None } else { Some(self.assign()) };
        semi_colon = self.next_consume();
        self.panic_token(&semi_colon, Token::SemiColon, format!("ast.rs(statement_for): Not Exists Semicolon {:?}", semi_colon));

        let end = if Token::RightBracket == self.next().get_token_type() { None } else { Some(self.assign()) };
        let r_bracket = self.next_consume();
        self.panic_token(&r_bracket, Token::RightBracket, format!("ast.rs(statement_for): Not Exists RightBracket {:?}", r_bracket));

        AstType::For(Box::new(begin), Box::new(condition), Box::new(end), Box::new(self.statement()))
    }

    // assign.
    fn assign(&mut self) -> AstType {
        let left = self.next();
        match left.get_token_type() {
            Token::Variable => {
                // 代入演算子判定.
                let var = self.factor();
                match self.next().get_token_type() {
                    Token::Assign => {
                        self.consume();
                        AstType::Assign(Box::new(var), Box::new(self.condition()))
                    }
                    Token::LeftBracket => self.call_func(var),
                    _ => {
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
    fn call_func(&mut self, acc: AstType) -> AstType {
        let token = self.next_consume();
        match token.get_token_type() {
            Token::LeftBracket => {
                let call_func = AstType::CallFunc(Box::new(acc), Box::new(self.argment(AstType::Argment(vec![]))));
                let next = self.next_consume();
                self.panic_token(&next, Token::RightBracket, format!("ast.rs(call_func): Not exists RightBracket {:?}", next));
                call_func
            }
            _ => panic!("ast.rs(call_func): Not exists LeftBracket")
        }
    }

    // argment.
    fn argment(&mut self, acc: AstType) -> AstType {
        // 右括弧が表れるまで、引数とみなす
        let token = self.next();
        match token.get_token_type() {
            Token::RightBracket => acc,
            Token::Variable | Token::Number => {
                match acc {
                    AstType::Argment(a) => {
                        let mut args = a;

                        if Token::Variable == token.get_token_type() {
                            args.push(AstType::Variable(token.get_token_value()));
                        } else {
                            args.push(self.number(token));
                        }
                        self.next_consume();

                        // カンマがあれば引き続き、引数とみなす.
                        if Token::Comma == self.next().get_token_type() {
                            self.next_consume();
                            self.argment(AstType::Argment(args))
                        } else {
                            AstType::Argment(args)
                        }
                    }
                    _ => panic!("ast.rs(argment): Error"),
                }
            }
            _ => acc,
        }
    }

    // condition.
    fn condition(&mut self) -> AstType {
        let left = self.logical();
        self.sub_condition(left)
    }

    // sub condition.
    fn sub_condition(&mut self, acc: AstType) -> AstType {
        let ope_type = self.next().get_token_type();
        match ope_type {
            Token::Question => {
                self.consume();
                let middle = self.logical();

                // コロンがない場合、終了.
                let colon = self.next_consume();
                self.panic_token(&colon, Token::Colon, format!("ast.rs(sub_condition): Not exists Colon {:?}", colon));

                let right = self.logical();
                let tree = AstType::Condition(Box::new(acc), Box::new(middle), Box::new(right));
                self.sub_condition(tree)
            }
            _ => acc,
        }
    }

    // logical.
    fn logical(&mut self) -> AstType {
        let left = self.bit_operator();
        self.sub_logical(left)
    }

    // sub logical.
    fn sub_logical(&mut self, acc: AstType) -> AstType {
        let create = |ope: Token, left, right| match ope {
            Token::LogicalAnd => AstType::LogicalAnd(Box::new(left), Box::new(right)),
            Token::Assign => AstType::Assign(Box::new(left), Box::new(right)),
            _ => AstType::LogicalOr(Box::new(left), Box::new(right)),
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
    fn bit_operator(&mut self) -> AstType {
        let left = self.relation();
        self.sub_bit_operator(left)
    }

    // sub bit operator.
    fn sub_bit_operator(&mut self, acc: AstType) -> AstType {
        let create = |ope, left, right| match ope {
            Token::BitOr => AstType::BitOr(Box::new(left), Box::new(right)),
            Token::BitAnd => AstType::BitAnd(Box::new(left), Box::new(right)),
            Token::BitXor => AstType::BitXor(Box::new(left), Box::new(right)),
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
    fn relation(&mut self) -> AstType {
        let left = self.shift();
        self.sub_relation(left)
    }

    // sub relation.
    fn sub_relation(&mut self, acc: AstType) -> AstType {
        let create = |ope: Token, left, right| match ope {
            Token::Equal => AstType::Equal(Box::new(left), Box::new(right)),
            Token::NotEqual => AstType::NotEqual(Box::new(left), Box::new(right)),
            Token::LessThan => AstType::LessThan(Box::new(left), Box::new(right)),
            Token::GreaterThan => AstType::GreaterThan(Box::new(left), Box::new(right)),
            Token::LessThanEqual => AstType::LessThanEqual(Box::new(left), Box::new(right)),
            Token::GreaterThanEqual => AstType::GreaterThanEqual(Box::new(left), Box::new(right)),
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
    fn shift(&mut self) -> AstType {
        let left = self.expr();
        self.sub_shift(left)
    }

    fn sub_shift(&mut self, acc: AstType) -> AstType {
        let create = |ope: Token, left, right| match ope {
            Token::LeftShift => AstType::LeftShift(Box::new(left), Box::new(right)),
            Token::RightShift => AstType::RightShift(Box::new(left), Box::new(right)),
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
    fn expr(&mut self) -> AstType {
        let left = self.term();
        self.expr_add_sub(left)
    }

    // add or sub expression.
    fn expr_add_sub(&mut self, acc: AstType) -> AstType {
        let create = |ope, left, right| match ope {
            Token::Plus => AstType::Plus(Box::new(left), Box::new(right)),
            _ => AstType::Minus(Box::new(left), Box::new(right)),
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
    fn term(&mut self) -> AstType {
        let left = self.factor();
        self.term_multi_div(left)
    }

    // multiple and division term.
    fn term_multi_div(&mut self, acc: AstType) -> AstType {
        let create = |ope, left, right| match ope {
            Token::Multi => AstType::Multiple(Box::new(left), Box::new(right)),
            Token::Division => AstType::Division(Box::new(left), Box::new(right)),
            _ => AstType::Remainder(Box::new(left), Box::new(right)),
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
    fn factor(&mut self) -> AstType {
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
                let bracket = self.next_consume();
                self.panic_token(&bracket, Token::RightBracket, format!("ast.rs(factor): Not exists RightBracket {:?}", bracket));
                tree
            }
            Token::Plus => {
                self.consume();
                AstType::UnPlus(Box::new(self.factor()))
            }
            Token::Minus => {
                self.consume();
                AstType::UnMinus(Box::new(self.factor()))
            }
            Token::Not => {
                self.consume();
                AstType::Not(Box::new(self.factor()))
            }
            Token::BitReverse => {
                self.consume();
                AstType::BitReverse(Box::new(self.factor()))
            }
            Token::Variable => {
                // シンボルテーブルへ保存（未登録の場合）.
                if None == self.var_table.search(&token.get_token_value()) {
                    self.var_table.push(token.get_token_value(), "".to_string());
                }
                self.consume();
                AstType::Variable(token.get_token_value())
            }
            _ => panic!("ast.rs: failed in factor {:?}", token),
        }
    }

    // number
    fn number(&self, token: TokenInfo) -> AstType {
        AstType::Factor(token.get_token_value().parse::<i64>().unwrap())
    }

    // トークン読み取り.
    fn next(&mut self) -> TokenInfo {
        self.tokens[self.current_pos].clone()
    }

    // 読み取り位置更新.
    #[allow(dead_code)]
    fn next_consume(&mut self) -> TokenInfo {
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

    // 指定されたトークンでない場合、panicメッセージ表示.
    fn panic_token(&self, d: &TokenInfo, t: Token, m: String) {
        if d.get_token_type() != t { panic!(m) }
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::Plus(
                            Box::new(AstType::Factor(1)),
                            Box::new(AstType::Factor(2))
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::Plus(
                            Box::new(AstType::Plus(
                                Box::new(AstType::Factor(1)),
                                Box::new(AstType::Factor(2)),
                            )),
                            Box::new(AstType::Factor(3))
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::Plus(
                            Box::new(AstType::Plus(
                                Box::new(AstType::Plus(
                                    Box::new(AstType::Factor(1)),
                                    Box::new(AstType::Factor(2)),
                                )),
                                Box::new(AstType::Factor(3)),
                            )),
                            Box::new(AstType::Factor(4))
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::Minus(
                            Box::new(AstType::Factor(1)),
                            Box::new(AstType::Factor(2))
                        ),
                    ])),
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
                TokenInfo::new(Token::Number, "100".to_string()),
                TokenInfo::new(Token::Minus, '-'.to_string()),
                TokenInfo::new(Token::Number, '2'.to_string()),
                TokenInfo::new(Token::Minus, '-'.to_string()),
                TokenInfo::new(Token::Number, '3'.to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::Minus(
                            Box::new(AstType::Minus(
                                Box::new(AstType::Factor(100)),
                                Box::new(AstType::Factor(2)),
                            )),
                            Box::new(AstType::Factor(3))
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::Minus(
                            Box::new(AstType::Minus(
                                Box::new(AstType::Minus(
                                    Box::new(AstType::Factor(1)),
                                    Box::new(AstType::Factor(2)),
                                )),
                                Box::new(AstType::Factor(3)),
                            )),
                            Box::new(AstType::Factor(4))
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::Multiple(
                            Box::new(AstType::Factor(1)),
                            Box::new(AstType::Factor(2))
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::Multiple(
                            Box::new(AstType::Multiple(
                                Box::new(AstType::Factor(1)),
                                Box::new(AstType::Factor(2)),
                            )),
                            Box::new(AstType::Factor(3))
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::Multiple(
                            Box::new(AstType::Multiple(
                                Box::new(AstType::Multiple(
                                    Box::new(AstType::Factor(1)),
                                    Box::new(AstType::Factor(2)),
                                )),
                                Box::new(AstType::Factor(3)),
                            )),
                            Box::new(AstType::Factor(4))
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::Division(
                            Box::new(AstType::Factor(1)),
                            Box::new(AstType::Factor(2))
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::Division(
                            Box::new(AstType::Division(
                                Box::new(AstType::Factor(1)),
                                Box::new(AstType::Factor(2)),
                            )),
                            Box::new(AstType::Factor(3))
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::Division(
                            Box::new(AstType::Division(
                                Box::new(AstType::Division(
                                    Box::new(AstType::Factor(1)),
                                    Box::new(AstType::Factor(2)),
                                )),
                                Box::new(AstType::Factor(3)),
                            )),
                            Box::new(AstType::Factor(4))
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::Plus(
                            Box::new(AstType::Multiple(
                                Box::new(AstType::Factor(1)),
                                Box::new(AstType::Factor(2)),
                            )),
                            Box::new(AstType::Factor(3))
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::Plus(
                            Box::new(AstType::Factor(1)),
                            Box::new(AstType::Multiple(
                                Box::new(AstType::Factor(2)),
                                Box::new(AstType::Factor(3)),
                            ))
                        ),
                    ])),
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
                TokenInfo::new(Token::RightBrace, "}".to_string()),
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::Plus(
                            Box::new(AstType::Division(
                                Box::new(AstType::Factor(1)),
                                Box::new(AstType::Factor(2)),
                            )),
                            Box::new(AstType::Factor(3))
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::Plus(
                            Box::new(AstType::Factor(1)),
                            Box::new(AstType::Division(
                                Box::new(AstType::Factor(2)),
                                Box::new(AstType::Factor(3)),
                            ))
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::GreaterThanEqual(
                            Box::new(AstType::Equal(
                                Box::new(AstType::LessThan(
                                    Box::new(AstType::Factor(2)),
                                    Box::new(AstType::Factor(3)),
                                )),
                                Box::new(AstType::Factor(4)),
                            )),
                            Box::new(AstType::Factor(5))
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::Plus(
                            Box::new(AstType::Factor(1)),
                            Box::new(AstType::Factor(2))
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::Plus(
                            Box::new(AstType::Factor(1)),
                            Box::new(AstType::Plus(
                                Box::new(AstType::Factor(2)),
                                Box::new(AstType::Factor(3)),
                            ))
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::Equal(
                            Box::new(AstType::Plus(
                                Box::new(AstType::Factor(1)),
                                Box::new(AstType::Factor(2)),
                            )),
                            Box::new(AstType::Plus(
                                Box::new(AstType::Factor(3)),
                                Box::new(AstType::Factor(4)),
                            ))
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::Equal(
                            Box::new(AstType::Multiple(
                                Box::new(AstType::Factor(1)),
                                Box::new(AstType::Factor(2)),
                            )),
                            Box::new(AstType::Multiple(
                                Box::new(AstType::Factor(3)),
                                Box::new(AstType::Factor(4)),
                            ))
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::Equal(
                            Box::new(AstType::Plus(
                                Box::new(AstType::Multiple(
                                    Box::new(AstType::Factor(1)),
                                    Box::new(AstType::Factor(2)),
                                )),
                                Box::new(AstType::Factor(1)),
                            )),
                            Box::new(AstType::Minus(
                                Box::new(AstType::Factor(3)),
                                Box::new(AstType::Factor(4)),
                            ))
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::NotEqual(
                            Box::new(AstType::Plus(
                                Box::new(AstType::Multiple(
                                    Box::new(AstType::Factor(1)),
                                    Box::new(AstType::Factor(2)),
                                )),
                                Box::new(AstType::Factor(1)),
                            )),
                            Box::new(AstType::Minus(
                                Box::new(AstType::Factor(3)),
                                Box::new(AstType::Factor(4)),
                            ))
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::LessThan(
                            Box::new(AstType::Plus(
                                Box::new(AstType::Multiple(
                                    Box::new(AstType::Factor(1)),
                                    Box::new(AstType::Factor(2)),
                                )),
                                Box::new(AstType::Factor(1)),
                            )),
                            Box::new(AstType::Minus(
                                Box::new(AstType::Factor(3)),
                                Box::new(AstType::Factor(4)),
                            ))
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::LessThanEqual(
                            Box::new(AstType::Plus(
                                Box::new(AstType::Multiple(
                                    Box::new(AstType::Factor(1)),
                                    Box::new(AstType::Factor(2)),
                                )),
                                Box::new(AstType::Factor(1)),
                            )),
                            Box::new(AstType::Minus(
                                Box::new(AstType::Factor(3)),
                                Box::new(AstType::Factor(4)),
                            ))
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::GreaterThan(
                            Box::new(AstType::Plus(
                                Box::new(AstType::Multiple(
                                    Box::new(AstType::Factor(1)),
                                    Box::new(AstType::Factor(2)),
                                )),
                                Box::new(AstType::Factor(1)),
                            )),
                            Box::new(AstType::Minus(
                                Box::new(AstType::Factor(3)),
                                Box::new(AstType::Factor(4)),
                            ))
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::GreaterThanEqual(
                            Box::new(AstType::Plus(
                                Box::new(AstType::Multiple(
                                    Box::new(AstType::Factor(1)),
                                    Box::new(AstType::Factor(2)),
                                )),
                                Box::new(AstType::Factor(1)),
                            )),
                            Box::new(AstType::Minus(
                                Box::new(AstType::Factor(3)),
                                Box::new(AstType::Factor(4)),
                            ))
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::LogicalAnd(
                            Box::new(AstType::Factor(2)),
                            Box::new(AstType::Factor(3))
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::LogicalAnd(
                            Box::new(AstType::Plus(
                                Box::new(AstType::Factor(2)),
                                Box::new(AstType::Factor(3)),
                            )),
                            Box::new(AstType::Plus(
                                Box::new(AstType::Factor(4)),
                                Box::new(AstType::Factor(5)),
                            ))
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::LogicalAnd(
                            Box::new(AstType::Equal(
                                Box::new(AstType::Plus(
                                    Box::new(AstType::Factor(2)),
                                    Box::new(AstType::Factor(3)),
                                )),
                                Box::new(AstType::Plus(
                                    Box::new(AstType::Factor(4)),
                                    Box::new(AstType::Factor(5)),
                                )),
                            )),
                            Box::new(AstType::NotEqual(
                                Box::new(AstType::Plus(
                                    Box::new(AstType::Factor(6)),
                                    Box::new(AstType::Factor(7)),
                                )),
                                Box::new(AstType::Plus(
                                    Box::new(AstType::Factor(8)),
                                    Box::new(AstType::Factor(9)),
                                )),
                            ))
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::LogicalOr(
                            Box::new(AstType::Factor(2)),
                            Box::new(AstType::Factor(3))
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::LogicalOr(
                            Box::new(AstType::Plus(
                                Box::new(AstType::Factor(2)),
                                Box::new(AstType::Factor(3)),
                            )),
                            Box::new(AstType::Plus(
                                Box::new(AstType::Factor(4)),
                                Box::new(AstType::Factor(5)),
                            ))
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::LogicalOr(
                            Box::new(AstType::Equal(
                                Box::new(AstType::Plus(
                                    Box::new(AstType::Factor(2)),
                                    Box::new(AstType::Factor(3)),
                                )),
                                Box::new(AstType::Plus(
                                    Box::new(AstType::Factor(4)),
                                    Box::new(AstType::Factor(5)),
                                )),
                            )),
                            Box::new(AstType::NotEqual(
                                Box::new(AstType::Plus(
                                    Box::new(AstType::Factor(6)),
                                    Box::new(AstType::Factor(7)),
                                )),
                                Box::new(AstType::Plus(
                                    Box::new(AstType::Factor(8)),
                                    Box::new(AstType::Factor(9)),
                                )),
                            ))
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::LogicalOr(
                            Box::new(AstType::LogicalAnd(
                                Box::new(AstType::LogicalOr(
                                    Box::new(AstType::Factor(2)),
                                    Box::new(AstType::Factor(3)),
                                )),
                                Box::new(AstType::Factor(4)),
                            )),
                            Box::new(AstType::Factor(5))
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::Condition(
                            Box::new(AstType::Equal(
                                Box::new(AstType::Factor(2)),
                                Box::new(AstType::Factor(3)),
                            )),
                            Box::new(AstType::Factor(1)),
                            Box::new(AstType::Factor(5))
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::Condition(
                            Box::new(AstType::Equal(
                                Box::new(AstType::Factor(2)),
                                Box::new(AstType::Factor(3)),
                            )),
                            Box::new(AstType::Condition(
                                Box::new(AstType::Equal(
                                    Box::new(AstType::Factor(10)),
                                    Box::new(AstType::Factor(11)),
                                )),
                                Box::new(AstType::Factor(12)),
                                Box::new(AstType::Factor(13)),
                            )),
                            Box::new(AstType::Factor(5))
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(
                        vec![AstType::UnPlus(Box::new(AstType::Factor(2)))],
                    )),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::Minus(
                            Box::new(AstType::UnPlus(Box::new(AstType::Factor(2)))),
                            Box::new(AstType::Factor(1))
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::Minus(
                            Box::new(AstType::UnPlus(Box::new(AstType::Factor(2)))),
                            Box::new(AstType::Factor(1))
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::Multiple(
                            Box::new(AstType::UnPlus(Box::new(AstType::Factor(2)))),
                            Box::new(AstType::Factor(1))
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![AstType::Not(Box::new(AstType::Factor(2)))])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::Not(Box::new(AstType::Equal(
                            Box::new(AstType::Factor(2)),
                            Box::new(AstType::Factor(3)),
                        ))),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(
                        vec![AstType::BitReverse(Box::new(AstType::Factor(2)))],
                    )),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::LeftShift(
                            Box::new(AstType::Factor(2)),
                            Box::new(AstType::Factor(1))
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::RightShift(
                            Box::new(AstType::Factor(2)),
                            Box::new(AstType::Factor(1))
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::RightShift(
                            Box::new(AstType::Plus(
                                Box::new(AstType::Factor(2)),
                                Box::new(AstType::Factor(3)),
                            )),
                            Box::new(AstType::Factor(1))
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::LessThan(
                            Box::new(AstType::Factor(2)),
                            Box::new(AstType::RightShift(
                                Box::new(AstType::Factor(3)),
                                Box::new(AstType::Factor(1)),
                            ))
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::BitAnd(
                            Box::new(AstType::Factor(2)),
                            Box::new(AstType::Factor(3))
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::BitOr(
                            Box::new(AstType::Factor(2)),
                            Box::new(AstType::Factor(3))
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::BitXor(
                            Box::new(AstType::Factor(2)),
                            Box::new(AstType::Factor(3))
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::BitOr(
                            Box::new(AstType::BitAnd(
                                Box::new(AstType::Factor(2)),
                                Box::new(AstType::Factor(3)),
                            )),
                            Box::new(AstType::Factor(4))
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::Assign(
                            Box::new(AstType::Variable("a".to_string())),
                            Box::new(AstType::Factor(3))
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::Assign(
                            Box::new(AstType::Variable("a".to_string())),
                            Box::new(AstType::Plus(
                                Box::new(AstType::Factor(3)),
                                Box::new(AstType::Factor(1)),
                            ))
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::Assign(
                            Box::new(AstType::Variable("a".to_string())),
                            Box::new(AstType::LogicalAnd(
                                Box::new(AstType::Factor(3)),
                                Box::new(AstType::Factor(1)),
                            ))
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::Assign(
                            Box::new(AstType::Variable("a".to_string())),
                            Box::new(AstType::Multiple(
                                Box::new(AstType::Factor(3)),
                                Box::new(AstType::Factor(1)),
                            ))
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::Assign(
                            Box::new(AstType::Variable("a".to_string())),
                            Box::new(AstType::BitOr(
                                Box::new(AstType::Factor(3)),
                                Box::new(AstType::Factor(1)),
                            ))
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::CallFunc(
                            Box::new(AstType::Variable("a".to_string())),
                            Box::new(AstType::Argment(vec![]))
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::CallFunc(
                            Box::new(AstType::Variable("a".to_string())),
                            Box::new(
                                AstType::Argment(vec![AstType::Variable('b'.to_string())]),
                            )
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::CallFunc(
                            Box::new(AstType::Variable("a".to_string())),
                            Box::new(AstType::Argment(vec![
                                AstType::Variable('b'.to_string()),
                                AstType::Variable('c'.to_string()),
                            ]))
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::Assign(
                            Box::new(AstType::Variable("a".to_string())),
                            Box::new(AstType::Factor(3))
                        ),
                        AstType::Assign(
                            Box::new(AstType::Variable("a".to_string())),
                            Box::new(AstType::Plus(
                                Box::new(AstType::Variable("a".to_string())),
                                Box::new(AstType::Factor(3)),
                            ))
                        ),
                    ])),
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
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::Assign(
                            Box::new(AstType::Variable("a".to_string())),
                            Box::new(AstType::Factor(3))
                        ),
                        AstType::Plus(
                            Box::new(AstType::Multiple(
                                Box::new(AstType::Variable("a".to_string())),
                                Box::new(AstType::Variable("a".to_string())),
                            )),
                            Box::new(AstType::Factor(1))
                        ),
                    ])),
                )
            )
        }
    }

    #[test]
    fn test_some_func_def() {
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
                TokenInfo::new(Token::Variable, "test".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Variable, "b".to_string()),
                TokenInfo::new(Token::Assign, "=".to_string()),
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::Assign(
                            Box::new(AstType::Variable("a".to_string())),
                            Box::new(AstType::Factor(3))
                        ),
                    ])),
                )
            );
            assert_eq!(
                result.get_tree()[1],
                AstType::FuncDef(
                    "test".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::Assign(
                            Box::new(AstType::Variable("b".to_string())),
                            Box::new(AstType::Factor(1))
                        ),
                    ])),
                )
            );
        }
    }

    #[test]
    fn test_func_def_with_args() {
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::Variable, "a".to_string()),
                TokenInfo::new(Token::Comma, ",".to_string()),
                TokenInfo::new(Token::Variable, "b".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Variable, "c".to_string()),
                TokenInfo::new(Token::Assign, "=".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![
                        AstType::Variable("a".to_string()),
                        AstType::Variable("b".to_string()),
                    ])),
                    Box::new(AstType::Statement(vec![
                        AstType::Assign(
                            Box::new(AstType::Variable("c".to_string())),
                            Box::new(AstType::Factor(3))
                        ),
                    ])),
                )
            );
        }
    }

    #[test]
    fn test_statement_if() {
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::If, "if".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::Variable, "a".to_string()),
                TokenInfo::new(Token::Equal, "==".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::Variable, "b".to_string()),
                TokenInfo::new(Token::Assign, "=".to_string()),
                TokenInfo::new(Token::Number, "10".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::If(
                            Box::new(AstType::Equal(
                                Box::new(AstType::Variable("a".to_string())),
                                Box::new(AstType::Factor(3))
                            )),
                            Box::new(AstType::Statement(
                                vec![
                                    AstType::Factor(1),
                                    AstType::Assign(
                                        Box::new(AstType::Variable("b".to_string())),
                                        Box::new(AstType::Factor(10))
                                    )
                                ],
                            )),
                            Box::new(None)
                        )
                    ]))
                )
            );
        }
    }

    #[test]
    fn test_statement_else() {
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::If, "if".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::Variable, "a".to_string()),
                TokenInfo::new(Token::Equal, "==".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::Variable, "b".to_string()),
                TokenInfo::new(Token::Assign, "=".to_string()),
                TokenInfo::new(Token::Number, "10".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),

                TokenInfo::new(Token::Else, "else".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Variable, "e".to_string()),
                TokenInfo::new(Token::Assign, "=".to_string()),
                TokenInfo::new(Token::Number, "9".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),

                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::If(
                            Box::new(AstType::Equal(
                                Box::new(AstType::Variable("a".to_string())),
                                Box::new(AstType::Factor(3))
                            )),
                            Box::new(AstType::Statement(
                                vec![
                                    AstType::Factor(1),
                                    AstType::Assign(
                                        Box::new(AstType::Variable("b".to_string())),
                                        Box::new(AstType::Factor(10))
                                    )
                                ],
                            )),
                            Box::new(
                                Some(AstType::Statement(
                                    vec![
                                        AstType::Assign(
                                            Box::new(AstType::Variable("e".to_string())),
                                            Box::new(AstType::Factor(9))
                                        )
                                    ],
                                ))
                            ),
                        ),
                    ]))
                )
            );
        }
    }

    #[test]
    fn test_statement_while() {
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::While, "while".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::Variable, "a".to_string()),
                TokenInfo::new(Token::Equal, "==".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::Variable, "b".to_string()),
                TokenInfo::new(Token::Assign, "=".to_string()),
                TokenInfo::new(Token::Number, "10".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::While(
                            Box::new(AstType::Equal(
                                Box::new(AstType::Variable("a".to_string())),
                                Box::new(AstType::Factor(3))
                            )),
                            Box::new(AstType::Statement(
                                vec![
                                    AstType::Factor(1),
                                    AstType::Assign(
                                        Box::new(AstType::Variable("b".to_string())),
                                        Box::new(AstType::Factor(10))
                                    )
                                ],
                            ))
                        )
                    ]))
                )
            );
        }
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::While, "while".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::Variable, "a".to_string()),
                TokenInfo::new(Token::Equal, "==".to_string()),
                TokenInfo::new(Token::Number, "3".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::Variable, "b".to_string()),
                TokenInfo::new(Token::Assign, "=".to_string()),
                TokenInfo::new(Token::Number, "10".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
                TokenInfo::new(Token::Variable, "b".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::While(
                            Box::new(AstType::Equal(
                                Box::new(AstType::Variable("a".to_string())),
                                Box::new(AstType::Factor(3))
                            )),
                            Box::new(AstType::Statement(
                                vec![
                                    AstType::Factor(1),
                                    AstType::Assign(
                                        Box::new(AstType::Variable("b".to_string())),
                                        Box::new(AstType::Factor(10))
                                    )
                                ],
                            ))
                        ),
                        AstType::Variable("b".to_string())
                    ]))
                )
            );
        }
    }

    #[test]
    fn test_statement_for() {
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::For, "for".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::Variable, "b".to_string()),
                TokenInfo::new(Token::Assign, "=".to_string()),
                TokenInfo::new(Token::Number, "10".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::For(
                            Box::new(None),
                            Box::new(None),
                            Box::new(None),
                            Box::new(AstType::Statement(
                                vec![
                                    AstType::Factor(1),
                                    AstType::Assign(
                                        Box::new(AstType::Variable("b".to_string())),
                                        Box::new(AstType::Factor(10))
                                    )
                                ],
                            ))
                        )
                    ]))
                )
            );
        }
        {
            let data = vec![
                TokenInfo::new(Token::Variable, "main".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::For, "for".to_string()),
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
                TokenInfo::new(Token::Variable, "i".to_string()),
                TokenInfo::new(Token::Assign, "=".to_string()),
                TokenInfo::new(Token::Number, "0".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::Variable, "i".to_string()),
                TokenInfo::new(Token::LessThan, "<".to_string()),
                TokenInfo::new(Token::Number, "10".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::Variable, "i".to_string()),
                TokenInfo::new(Token::Assign, "=".to_string()),
                TokenInfo::new(Token::Variable, "i".to_string()),
                TokenInfo::new(Token::Plus, "+".to_string()),
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::RightBracket, ")".to_string()),
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                TokenInfo::new(Token::Number, "1".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::Variable, "b".to_string()),
                TokenInfo::new(Token::Assign, "=".to_string()),
                TokenInfo::new(Token::Number, "10".to_string()),
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
                TokenInfo::new(Token::RightBrace, "}".to_string()),
                TokenInfo::new(Token::End, "End".to_string()),
            ];
            let mut ast = AstGen::new(&data);
            let result = ast.parse();

            // 期待値確認.
            assert_eq!(
                result.get_tree()[0],
                AstType::FuncDef(
                    "main".to_string(),
                    Box::new(AstType::Argment(vec![])),
                    Box::new(AstType::Statement(vec![
                        AstType::For(
                            Box::new(Some(
                                AstType::Assign(
                                    Box::new(AstType::Variable("i".to_string())),
                                    Box::new(AstType::Factor(0))
                                ),
                            )),
                            Box::new(Some(
                                AstType::LessThan(
                                    Box::new(AstType::Variable("i".to_string())),
                                    Box::new(AstType::Factor(10))
                                ),
                            )),
                            Box::new(Some(
                                AstType::Assign(
                                    Box::new(AstType::Variable("i".to_string())),
                                    Box::new(AstType::Plus(
                                        Box::new(AstType::Variable("i".to_string())),
                                        Box::new(AstType::Factor(1))
                                    ))
                                )
                            )),
                            Box::new(AstType::Statement(
                                vec![
                                    AstType::Factor(1),
                                    AstType::Assign(
                                        Box::new(AstType::Variable("b".to_string())),
                                        Box::new(AstType::Factor(10))
                                    )
                                ],
                            ))
                        )
                    ]))
                )
            );
        }
    }
}
