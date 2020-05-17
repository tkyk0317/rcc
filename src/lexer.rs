use token::{Token, TokenInfo};

#[doc = " 字句解析"]
pub struct LexicalAnalysis<'a> {
    name: String,
    input: &'a str,
    row: usize,
    col: usize,
    pos: usize,
    tokens: Vec<TokenInfo>,
}

impl<'a> LexicalAnalysis<'a> {
    // コンストラクタ.
    pub fn new(n: String, i: &'a str) -> LexicalAnalysis {
        LexicalAnalysis {
            name: n,
            input: i,
            row: 1,
            col: 0,
            pos: 0,
            tokens: vec![],
        }
    }

    // トークン群取得.
    pub fn get_tokens(&self) -> &Vec<TokenInfo> {
        &self.tokens
    }

    // トークン読み込み.
    pub fn read_token(&mut self) {
        // 終了まで読み込み、字句解析を行う.
        while !self.is_eof() {
            // 空白、改行などは読み飛ばし.
            self.skip_ascii_whitespace();
            // コメント読み飛ばし
            self.skip_comment();

            // 一文字読み取って、トークン生成.
            //match self.next() {
            if let Some(v) = self.next() {
                let token = match v {
                    s if s.is_alphabetic() || s == '_' => {
                        if let Some(t) = self.generate_type(s) {
                            t
                        } else if let Some(t) = self.generate_statement(s) {
                            t
                        } else if let Some(t) = self.generate_sizeof(s) {
                            t
                        } else {
                            self.generate_variable_token(s)
                        }
                    }
                    '=' => {
                        if self.is_equal(v) {
                            let t = self.create_token(Token::Equal, "==".to_string());
                            self.skip(1);
                            t
                        } else {
                            self.create_token(Token::Assign, v.to_string())
                        }
                    }
                    '!' => {
                        if self.is_not_equal(v) {
                            let t = self.create_token(Token::NotEqual, "!=".to_string());
                            self.skip(1);
                            t
                        } else {
                            self.create_token(Token::Not, v.to_string())
                        }
                    }
                    '>' => {
                        if self.is_greater_than_equal(v) {
                            let t =
                                self.create_token(Token::GreaterThanEqual, ">=".to_string());
                            self.skip(1);
                            t
                        } else if self.is_right_shift(v) {
                            let t = self.create_token(Token::RightShift, ">>".to_string());
                            self.skip(1);
                            t
                        } else {
                            self.create_token(Token::GreaterThan, v.to_string())
                        }
                    }
                    '<' => {
                        if self.is_less_than_equal(v) {
                            let t = self.create_token(Token::LessThanEqual, "<=".to_string());
                            self.skip(1);
                            t
                        } else if self.is_left_shift(v) {
                            let t = self.create_token(Token::LeftShift, "<<".to_string());
                            self.skip(1);
                            t
                        } else {
                            self.create_token(Token::LessThan, v.to_string())
                        }
                    }
                    '&' => {
                        if self.is_logical_and(v) {
                            self.skip(1);
                            self.create_token(Token::LogicalAnd, "&&".to_string())
                        } else {
                            self.create_token(Token::And, v.to_string())
                        }
                    }
                    '|' => {
                        if self.is_logical_or(v) {
                            self.skip(1);
                            self.create_token(Token::LogicalOr, "||".to_string())
                        } else {
                            self.create_token(Token::BitOr, v.to_string())
                        }
                    }
                    '+' => {
                        if self.is_increment(v) {
                            let token = self.create_token(Token::Inc, "++".to_string());
                            self.skip(1);
                            token
                        } else if self.is_plus_assign(v) {
                            let token = self.create_token(Token::PlusAssign, "+=".to_string());
                            self.skip(1);
                            token
                        } else {
                            self.create_token(Token::Plus, v.to_string())
                        }
                    }
                    '-' => {
                        if self.is_decrement(v) {
                            let token = self.create_token(Token::Dec, "--".to_string());
                            self.skip(1);
                            token
                        } else if self.is_minus_assign(v)  {
                            let token = self.create_token(Token::MinusAssign, "-=".to_string());
                            self.skip(1);
                            token
                        } else {
                            self.create_token(Token::Minus, v.to_string())
                        }
                    }
                    '*' => {
                        if self.is_multiple_assign(v) {
                            let token = self.create_token(Token::MultipleAssign, "*=".to_string());
                            self.skip(1);
                            token
                        } else {
                            self.create_token(Token::Multi, v.to_string())
                        }
                    }
                    '/' => {
                        if self.is_division_assign(v) {
                            let token = self.create_token(Token::DivisionAssign, "/=".to_string());
                            self.skip(1);
                            token
                        } else {
                            self.create_token(Token::Division, v.to_string())
                        }
                    }
                    '%' => {
                        if self.is_remainder_assign(v) {
                            let token = self.create_token(Token::RemainderAssign, "%=".to_string());
                            self.skip(1);
                            token
                        } else {
                            self.create_token(Token::Remainder, v.to_string())
                        }
                    }
                    '"' => self.generate_string(),
                    '^' => self.create_token(Token::BitXor, v.to_string()),
                    '~' => self.create_token(Token::BitReverse, v.to_string()),
                    '(' => self.create_token(Token::LeftParen, v.to_string()),
                    ')' => self.create_token(Token::RightParen, v.to_string()),
                    '{' => self.create_token(Token::LeftBrace, v.to_string()),
                    '}' => self.create_token(Token::RightBrace, v.to_string()),
                    '[' => self.create_token(Token::LeftBracket, v.to_string()),
                    ']' => self.create_token(Token::RightBracket, v.to_string()),
                    '?' => self.create_token(Token::Question, v.to_string()),
                    ':' => self.create_token(Token::Colon, v.to_string()),
                    ';' => self.create_token(Token::SemiColon, v.to_string()),
                    ',' => self.create_token(Token::Comma, v.to_string()),
                    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                        self.generate_number_token(v)
                    }
                    _ => panic!("{} {}: Not Support Lexer {}", file!(), line!(), v),
                };
                self.tokens.push(token);
            }
        }
        let end = self.create_token(Token::End, "End".to_string());
        self.tokens.push(end);
    }

    // トークン作成
    fn create_token(&self, t: Token, s: String) -> TokenInfo {
        TokenInfo::new(t, s, (self.name.clone(), self.row, self.col))
    }

    // 文字を読み出す.
    fn read(&self) -> char {
        self.input
            .chars()
            .nth(self.pos)
            .expect("lexer.rs(read): cannot read next char")
    }

    // 文字列を取得.
    fn read_string(&mut self, n: usize) -> String {
        // 指定文字数をread.
        let (s, c) = (0..n).fold((String::new(), 0), |d, _| {
            if self.is_eof() {
                return (d.0, d.1);
            }
            if let Some(c) = self.next() {
                return (d.0 + &c.to_string(), d.1 + 1);
            }
            d
        });
        self.back(c);
        s
    }

    // 文字を読み出して次へ進める.
    fn next(&mut self) -> Option<char> {
        let s = self.input.chars().nth(self.pos);
        self.skip(1);
        s
    }

    // 改行文字判定
    fn is_linefeed(&self, s: &str) -> bool {
        s == "\n"
    }

    // 空白や改行、タブをスキップ.
    fn skip_ascii_whitespace(&mut self) {
        while !self.is_eof() && self.read().is_ascii_whitespace() {
            let next = self.read_string(1);
            if self.is_linefeed(&next) {
                // 行とカラムを更新
                self.row += 1;
                self.col = 0;
            }
            self.skip(1);
        }
    }

    // コメント読み飛ばし
    fn skip_comment(&mut self) {
        if self.read_string(2) == "//" {
            // 改行コードまで読み飛ばし
            self.skip(2);
            let mut next = self.read_string(1);
            while !self.is_eof() && !self.is_linefeed(&next) {
                self.skip(1);
                next = self.read_string(1);
            }
            if self.is_linefeed(&next) {
                self.skip(1);
                self.row += 1;
                self.col = 0;

                // 改行後、先頭に空白がある可能性を考慮
                self.skip_ascii_whitespace();
            }
        }
    }

    // 文字をスキップ.
    fn skip(&mut self, i: usize) {
        self.pos += i;
        self.col += i;
    }

    // 文字読み取り位置を戻す.
    fn back(&mut self, n: usize) {
        self.pos -= n;
        self.col -= n;
    }

    // 文字列終端チェック.
    fn is_eof(&self) -> bool {
        self.pos > (self.input.len() - 1)
    }

    // 変数候補チェック.
    fn is_variable(&self, c: char) -> bool {
        c.is_alphabetic() || c == '_' || c.is_digit(10)
    }

    // 文字列トークン生成
    fn generate_string(&mut self) -> TokenInfo {
        // 文字列先頭位置を退避
        let col = self.col;
        let mut s = String::new();
        while '"' != self.read() && !self.is_eof() {
            let c = self.next();
            s.push(c.expect("lexer.rs(generate_string): cannot read next char"));
        }
        // 最後のダブルクォテーションを消費
        self.skip(1);

        // 退避した文字列先頭位置へ
        let mut t = self.create_token(Token::StringLiteral, s);
        t.pos.col = col;
        t
    }

    // 数値トークン生成.
    fn generate_number_token(&mut self, v: char) -> TokenInfo {
        let mut s = String::new();
        s.push(v);

        while !self.is_eof() && self.read().is_digit(10) {
            let n = self.next();
            s.push(n.expect("lexer.rs(generate_number_token): cannot read next char"));
        }
        self.create_token(Token::Number, s)
    }

    // 変数トークン生成.
    fn generate_variable_token(&mut self, v: char) -> TokenInfo {
        let mut s = String::new();
        s.push(v);
        while !self.is_eof() && self.is_variable(self.read()) {
            s.push(self.read());
            self.skip(1);
        }
        // 位置が文字列の先頭を指すように調整
        let l = s.len();
        let mut t = self.create_token(Token::Variable, s);
        t.pos.col -= l - 1;
        t
    }

    // 等価演算子チェック.
    fn is_equal(&mut self, v: char) -> bool {
        v == '=' && self.read() == '='
    }

    // 否等価演算子チェック.
    fn is_not_equal(&mut self, v: char) -> bool {
        v == '!' && self.read() == '='
    }

    // 比較演算子(>=)チェック
    fn is_greater_than_equal(&self, v: char) -> bool {
        v == '>' && self.read() == '='
    }

    // 比較演算子(<=)チェック
    fn is_less_than_equal(&self, v: char) -> bool {
        v == '<' && self.read() == '='
    }

    // &&演算子チェック
    fn is_logical_and(&self, v: char) -> bool {
        v == '&' && self.read() == '&'
    }

    // ||演算子チェック
    fn is_logical_or(&self, v: char) -> bool {
        v == '|' && self.read() == '|'
    }

    // 左シフト演算子チェック.
    fn is_left_shift(&self, v: char) -> bool {
        v == '<' && self.read() == '<'
    }

    // 右シフト演算子チェック.
    fn is_right_shift(&self, v: char) -> bool {
        v == '>' && self.read() == '>'
    }

    // ++演算子チェック
    fn is_increment(&self, v: char) -> bool {
        v == '+' && self.read() == '+'
    }

    fn is_decrement(&self, v: char) -> bool {
        v == '-' && self.read() == '-'
    }

    // plus assign演算子
    fn is_plus_assign(&self, v: char) -> bool {
        v == '+' && self.read() == '='
    }

    // minus assign演算子
    fn is_minus_assign(&self, v: char) -> bool {
        v == '-' && self.read() == '='
    }

    // multiple assign演算子
    fn is_multiple_assign(&self, v: char) -> bool {
        v == '*' && self.read() == '='
    }

    // division assign演算子
    fn is_division_assign(&self, v: char) -> bool {
        v == '/' && self.read() == '='
    }

    // remainder assign演算子
    fn is_remainder_assign(&self, v: char) -> bool {
        v == '%' && self.read() == '='
    }

    // type int作成
    fn generate_type_int(&mut self) -> TokenInfo {
        let col = self.col;
        self.skip(2);

        // ポインタ型であるかチェック.
        if self.is_pointer() {
            // 位置が先頭を指し示すように修正
            self.skip(1);
            let mut t = self.create_token(Token::IntPointer, "int*".to_string());
            t.pos.col = col;
            t
        } else {
            // 位置が先頭を指し示すように修正
            let mut t = self.create_token(Token::Int, "int".to_string());
            t.pos.col = col;
            t
        }
    }

    // type char作成
    fn generate_type_char(&mut self) -> TokenInfo {
        let col = self.col;
        self.skip(3);

        // ポインタ型であるかチェック.
        if self.is_pointer() {
            // 位置が先頭を指し示すように修正
            self.skip(1);
            let mut t = self.create_token(Token::CharPointer, "char*".to_string());
            t.pos.col = col;
            t
        } else {
            // 位置が先頭を指し示すように修正
            let mut t = self.create_token(Token::Char, "char".to_string());
            t.pos.col = col;
            t
        }
    }

    // type作成
    fn generate_type(&mut self, c: char) -> Option<TokenInfo> {
        if self.is_type_int(c) {
            Some(self.generate_type_int())
        } else if self.is_type_char(c) {
            Some(self.generate_type_char())
        } else {
            None
        }
    }

    // char型
    fn is_type_char(&mut self, c: char) -> bool {
        let s = self.read_string(4);
        let l = s.chars().last();

        // charx等の変数とは区別する為、最後の文字をチェック
        c == 'c'
            && s.len() == 4
            && &s[0..3] == "har"
            && !self.is_variable(l.expect("lexer.rs(is_type_char): read error"))
    }

    // int型チェック
    fn is_type_int(&mut self, c: char) -> bool {
        let s = self.read_string(3);
        let l = s.chars().last();

        // inta等の変数と区別する為、最後の文字をチェック
        c == 'i'
            && s.len() == 3
            && &s[0..2] == "nt"
            && !self.is_variable(l.expect("lexer.rs(is_type_int): read error"))
    }

    // sizeof演算
    fn generate_sizeof(&mut self, c: char) -> Option<TokenInfo> {
        if self.is_sizeof(c) {
            let t = Some(self.create_token(Token::SizeOf, "sizeof".to_string()));
            self.skip(5);
            t
        } else {
            None
        }
    }

    // sizeof演算子チェック
    fn is_sizeof(&mut self, c: char) -> bool {
        let s = self.read_string(6);
        let l = s.chars().last();
        c == 's'
            && s.len() == 6
            && "izeof" == &s[0..5]
            && !self.is_variable(l.expect("lexer.rs(generate_sizeof): read error"))
    }

    // ポインタ演算子が存在するか.
    fn is_pointer(&mut self) -> bool {
        // 空白は読み飛ばして、ポインタ型があるかチェック.
        self.skip_ascii_whitespace();
        '*' == self.read()
    }

    // statement作成.
    fn generate_statement(&mut self, c: char) -> Option<TokenInfo> {
        if self.is_statement_if(c) {
            let s = Some(self.create_token(Token::If, "if".to_string()));
            self.skip(1);
            s
        } else if self.is_statement_else(c) {
            let s = Some(self.create_token(Token::Else, "else".to_string()));
            self.skip(3);
            s
        } else if self.is_statement_while(c) {
            let s = Some(self.create_token(Token::While, "while".to_string()));
            self.skip(4);
            s
        } else if self.is_statement_for(c) {
            let s = Some(self.create_token(Token::For, "for".to_string()));
            self.skip(2);
            s
        } else if self.is_statement_do(c) {
            let s = Some(self.create_token(Token::Do, "do".to_string()));
            self.skip(1);
            s
        } else if self.is_statement_continue(c) {
            let s = Some(self.create_token(Token::Continue, "continue".to_string()));
            self.skip(7);
            s
        } else if self.is_statement_break(c) {
            let s = Some(self.create_token(Token::Break, "break".to_string()));
            self.skip(4);
            s
        } else if self.is_statement_return(c) {
            let s = Some(self.create_token(Token::Return, "return".to_string()));
            self.skip(5);
            s
        } else {
            None
        }
    }

    // if statementチェック.
    fn is_statement_if(&mut self, v: char) -> bool {
        let s = self.read_string(2);
        let l = s.chars().last();
        v == 'i'
            && s.len() == 2
            && "f" == &s[0..1]
            && !self.is_variable(l.expect("lexer.rs(is_statement_if): read error"))
    }

    // else statementチェック.
    fn is_statement_else(&mut self, v: char) -> bool {
        let s = self.read_string(4);
        let l = s.chars().last();
        v == 'e'
            && s.len() == 4
            && "lse" == &s[0..3]
            && !self.is_variable(l.expect("lexer.rs(is_statement_else): read error"))
    }

    // while statementチェック.
    fn is_statement_while(&mut self, v: char) -> bool {
        let s = self.read_string(5);
        let l = s.chars().last();
        v == 'w'
            && s.len() == 5
            && "hile" == &s[0..4]
            && !self.is_variable(l.expect("lexer.rs(is_statement_while): read error"))
    }

    // do-while statementチェック.
    fn is_statement_do(&mut self, v: char) -> bool {
        let s = self.read_string(2);
        let l = s.chars().last();
        v == 'd'
            && s.len() == 2
            && "o" == &s[0..1]
            && !self.is_variable(l.expect("lexer.rs(is_statement_do): read error"))
    }

    // for statementチェック.
    fn is_statement_for(&mut self, v: char) -> bool {
        let s = self.read_string(3);
        let l = s.chars().last();
        v == 'f'
            && s.len() == 3
            && "or" == &s[0..2]
            && !self.is_variable(l.expect("lexer.rs(is_statement_for): read error"))
    }

    // continue statementチェック.
    fn is_statement_continue(&mut self, v: char) -> bool {
        let s = self.read_string(8);
        let l = s.chars().last();
        v == 'c'
            && s.len() == 8
            && "ontinue" == &s[0..7]
            && !self.is_variable(l.expect("lexer.rs(is_statement_continue): read error"))
    }

    // break statementチェック.
    fn is_statement_break(&mut self, v: char) -> bool {
        let s = self.read_string(5);
        let l = s.chars().last();
        v == 'b'
            && s.len() == 5
            && "reak" == &s[0..4]
            && !self.is_variable(l.expect("lexer.rs(is_statement_break): read error"))
    }

    // return statementチェック.
    fn is_statement_return(&mut self, v: char) -> bool {
        let s = self.read_string(6);
        let l = s.chars().last();
        v == 'r'
            && s.len() == 6
            && "eturn" == &s[0..5]
            && !self.is_variable(l.expect("lexer.rs(is_statement_return): read error"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        {
            let input = "2 + 1 / 3 * 7".to_string();
            let mut lexer = LexicalAnalysis::new("test.c".to_string(), &input);

            lexer.read_token();

            assert_eq!(
                TokenInfo::new(Token::Number, "2".to_string(), ("test.c".to_string(), 1, 1)),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(Token::Plus, "+".to_string(), ("test.c".to_string(), 1, 3)),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "1".to_string(), ("test.c".to_string(), 1, 5)),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Division,
                    "/".to_string(),
                    ("test.c".to_string(), 1, 7)
                ),
                lexer.get_tokens()[3]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "3".to_string(), ("test.c".to_string(), 1, 9)),
                lexer.get_tokens()[4]
            );
            assert_eq!(
                TokenInfo::new(Token::Multi, "*".to_string(), ("test.c".to_string(), 1, 11)),
                lexer.get_tokens()[5]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Number,
                    "7".to_string(),
                    ("test.c".to_string(), 1, 13)
                ),
                lexer.get_tokens()[6]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string(), ("test.c".to_string(), 1, 13)),
                lexer.get_tokens()[7]
            );
        }
        {
            let input = "2 >= 1".to_string();
            let mut lexer = LexicalAnalysis::new("test.c".to_string(), &input);

            lexer.read_token();

            assert_eq!(
                TokenInfo::new(Token::Number, "2".to_string(), ("test.c".to_string(), 1, 1)),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::GreaterThanEqual,
                    ">=".to_string(),
                    ("test.c".to_string(), 1, 3)
                ),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "1".to_string(), ("test.c".to_string(), 1, 6)),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string(), ("test.c".to_string(), 1, 6)),
                lexer.get_tokens()[3]
            );
        }
        {
            let input = "2 <= 1".to_string();
            let mut lexer = LexicalAnalysis::new("test.c".to_string(), &input);

            lexer.read_token();

            assert_eq!(
                TokenInfo::new(Token::Number, "2".to_string(), ("test.c".to_string(), 1, 1)),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::LessThanEqual,
                    "<=".to_string(),
                    ("test.c".to_string(), 1, 3)
                ),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "1".to_string(), ("test.c".to_string(), 1, 6)),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string(), ("test.c".to_string(), 1, 6)),
                lexer.get_tokens()[3]
            );
        }
        {
            let input = "2 == 1".to_string();
            let mut lexer = LexicalAnalysis::new("test.c".to_string(), &input);

            lexer.read_token();

            assert_eq!(
                TokenInfo::new(Token::Number, "2".to_string(), ("test.c".to_string(), 1, 1)),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(Token::Equal, "==".to_string(), ("test.c".to_string(), 1, 3)),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "1".to_string(), ("test.c".to_string(), 1, 6)),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string(), ("test.c".to_string(), 1, 6)),
                lexer.get_tokens()[3]
            );
        }
        {
            let input = "2 != 1".to_string();
            let mut lexer = LexicalAnalysis::new("test.c".to_string(), &input);

            lexer.read_token();

            assert_eq!(
                TokenInfo::new(Token::Number, "2".to_string(), ("test.c".to_string(), 1, 1)),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::NotEqual,
                    "!=".to_string(),
                    ("test.c".to_string(), 1, 3)
                ),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "1".to_string(), ("test.c".to_string(), 1, 6)),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string(), ("test.c".to_string(), 1, 6)),
                lexer.get_tokens()[3]
            );
        }
        {
            let input = "2 << 1".to_string();
            let mut lexer = LexicalAnalysis::new("test.c".to_string(), &input);

            lexer.read_token();

            assert_eq!(
                TokenInfo::new(Token::Number, "2".to_string(), ("test.c".to_string(), 1, 1)),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::LeftShift,
                    "<<".to_string(),
                    ("test.c".to_string(), 1, 3)
                ),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "1".to_string(), ("test.c".to_string(), 1, 6)),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string(), ("test.c".to_string(), 1, 6)),
                lexer.get_tokens()[3]
            );
        }
        {
            let input = "2 >> 1".to_string();
            let mut lexer = LexicalAnalysis::new("test.c".to_string(), &input);

            lexer.read_token();

            assert_eq!(
                TokenInfo::new(Token::Number, "2".to_string(), ("test.c".to_string(), 1, 1)),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::RightShift,
                    ">>".to_string(),
                    ("test.c".to_string(), 1, 3)
                ),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "1".to_string(), ("test.c".to_string(), 1, 6)),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string(), ("test.c".to_string(), 1, 6)),
                lexer.get_tokens()[3]
            );
        }
        {
            let input = "~1".to_string();
            let mut lexer = LexicalAnalysis::new("test.c".to_string(), &input);

            lexer.read_token();

            assert_eq!(
                TokenInfo::new(
                    Token::BitReverse,
                    "~".to_string(),
                    ("test.c".to_string(), 1, 1)
                ),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "1".to_string(), ("test.c".to_string(), 1, 2)),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string(), ("test.c".to_string(), 1, 2)),
                lexer.get_tokens()[2]
            );
        }
        {
            let input = "1 + 2;".to_string();
            let mut lexer = LexicalAnalysis::new("test.c".to_string(), &input);

            lexer.read_token();

            assert_eq!(
                TokenInfo::new(Token::Number, "1".to_string(), ("test.c".to_string(), 1, 1)),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(Token::Plus, "+".to_string(), ("test.c".to_string(), 1, 3)),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "2".to_string(), ("test.c".to_string(), 1, 5)),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::SemiColon,
                    ";".to_string(),
                    ("test.c".to_string(), 1, 6)
                ),
                lexer.get_tokens()[3]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string(), ("test.c".to_string(), 1, 6)),
                lexer.get_tokens()[4]
            );
        }
        {
            let input = "1 + 2; 3 >= 2".to_string();
            let mut lexer = LexicalAnalysis::new("test.c".to_string(), &input);

            lexer.read_token();

            assert_eq!(
                TokenInfo::new(Token::Number, "1".to_string(), ("test.c".to_string(), 1, 1)),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(Token::Plus, "+".to_string(), ("test.c".to_string(), 1, 3)),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "2".to_string(), ("test.c".to_string(), 1, 5)),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::SemiColon,
                    ";".to_string(),
                    ("test.c".to_string(), 1, 6)
                ),
                lexer.get_tokens()[3]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "3".to_string(), ("test.c".to_string(), 1, 8)),
                lexer.get_tokens()[4]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::GreaterThanEqual,
                    ">=".to_string(),
                    ("test.c".to_string(), 1, 10)
                ),
                lexer.get_tokens()[5]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Number,
                    "2".to_string(),
                    ("test.c".to_string(), 1, 13)
                ),
                lexer.get_tokens()[6]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string(), ("test.c".to_string(), 1, 13)),
                lexer.get_tokens()[7]
            );
        }
        {
            let input = "a = 1 + 2;".to_string();
            let mut lexer = LexicalAnalysis::new("test.c".to_string(), &input);

            lexer.read_token();

            assert_eq!(
                TokenInfo::new(
                    Token::Variable,
                    "a".to_string(),
                    ("test.c".to_string(), 1, 1)
                ),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(Token::Assign, "=".to_string(), ("test.c".to_string(), 1, 3)),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "1".to_string(), ("test.c".to_string(), 1, 5)),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(Token::Plus, "+".to_string(), ("test.c".to_string(), 1, 7)),
                lexer.get_tokens()[3]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "2".to_string(), ("test.c".to_string(), 1, 9)),
                lexer.get_tokens()[4]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::SemiColon,
                    ";".to_string(),
                    ("test.c".to_string(), 1, 10)
                ),
                lexer.get_tokens()[5]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string(), ("test.c".to_string(), 1, 10)),
                lexer.get_tokens()[6]
            );
        }
        {
            let input = "_a_aa = 1 + 2;".to_string();
            let mut lexer = LexicalAnalysis::new("test.c".to_string(), &input);

            lexer.read_token();

            assert_eq!(
                TokenInfo::new(
                    Token::Variable,
                    "_a_aa".to_string(),
                    ("test.c".to_string(), 1, 1)
                ),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(Token::Assign, "=".to_string(), ("test.c".to_string(), 1, 7)),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "1".to_string(), ("test.c".to_string(), 1, 9)),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(Token::Plus, "+".to_string(), ("test.c".to_string(), 1, 11)),
                lexer.get_tokens()[3]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Number,
                    "2".to_string(),
                    ("test.c".to_string(), 1, 13)
                ),
                lexer.get_tokens()[4]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::SemiColon,
                    ";".to_string(),
                    ("test.c".to_string(), 1, 14)
                ),
                lexer.get_tokens()[5]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string(), ("test.c".to_string(), 1, 14)),
                lexer.get_tokens()[6]
            );
        }
        {
            let input = "a, b;".to_string();
            let mut lexer = LexicalAnalysis::new("test.c".to_string(), &input);

            lexer.read_token();

            assert_eq!(
                TokenInfo::new(
                    Token::Variable,
                    "a".to_string(),
                    ("test.c".to_string(), 1, 1)
                ),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(Token::Comma, ",".to_string(), ("test.c".to_string(), 1, 2)),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Variable,
                    "b".to_string(),
                    ("test.c".to_string(), 1, 4)
                ),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::SemiColon,
                    ";".to_string(),
                    ("test.c".to_string(), 1, 5)
                ),
                lexer.get_tokens()[3]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string(), ("test.c".to_string(), 1, 5)),
                lexer.get_tokens()[4]
            );
        }
        {
            let input = "{ a = b; }".to_string();
            let mut lexer = LexicalAnalysis::new("test.c".to_string(), &input);

            lexer.read_token();

            assert_eq!(
                TokenInfo::new(
                    Token::LeftBrace,
                    "{".to_string(),
                    ("test.c".to_string(), 1, 1)
                ),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Variable,
                    "a".to_string(),
                    ("test.c".to_string(), 1, 3)
                ),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(Token::Assign, "=".to_string(), ("test.c".to_string(), 1, 5)),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Variable,
                    "b".to_string(),
                    ("test.c".to_string(), 1, 7)
                ),
                lexer.get_tokens()[3]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::SemiColon,
                    ";".to_string(),
                    ("test.c".to_string(), 1, 8)
                ),
                lexer.get_tokens()[4]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::RightBrace,
                    "}".to_string(),
                    ("test.c".to_string(), 1, 10)
                ),
                lexer.get_tokens()[5]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string(), ("test.c".to_string(), 1, 10)),
                lexer.get_tokens()[6]
            );
        }
        {
            let input = "if { i = 2; }".to_string();
            let mut lexer = LexicalAnalysis::new("test.c".to_string(), &input);

            lexer.read_token();

            assert_eq!(
                TokenInfo::new(Token::If, "if".to_string(), ("test.c".to_string(), 1, 1)),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::LeftBrace,
                    "{".to_string(),
                    ("test.c".to_string(), 1, 4)
                ),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Variable,
                    "i".to_string(),
                    ("test.c".to_string(), 1, 6)
                ),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(Token::Assign, "=".to_string(), ("test.c".to_string(), 1, 8)),
                lexer.get_tokens()[3]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Number,
                    "2".to_string(),
                    ("test.c".to_string(), 1, 10)
                ),
                lexer.get_tokens()[4]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::SemiColon,
                    ";".to_string(),
                    ("test.c".to_string(), 1, 11)
                ),
                lexer.get_tokens()[5]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::RightBrace,
                    "}".to_string(),
                    ("test.c".to_string(), 1, 13)
                ),
                lexer.get_tokens()[6]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string(), ("test.c".to_string(), 1, 13)),
                lexer.get_tokens()[7]
            );
        }
        {
            let input = "if { i = 2; } else { e = 3; }".to_string();
            let mut lexer = LexicalAnalysis::new("test.c".to_string(), &input);

            lexer.read_token();

            assert_eq!(
                TokenInfo::new(Token::If, "if".to_string(), ("test.c".to_string(), 1, 1)),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::LeftBrace,
                    "{".to_string(),
                    ("test.c".to_string(), 1, 4)
                ),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Variable,
                    "i".to_string(),
                    ("test.c".to_string(), 1, 6)
                ),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(Token::Assign, "=".to_string(), ("test.c".to_string(), 1, 8)),
                lexer.get_tokens()[3]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Number,
                    "2".to_string(),
                    ("test.c".to_string(), 1, 10)
                ),
                lexer.get_tokens()[4]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::SemiColon,
                    ";".to_string(),
                    ("test.c".to_string(), 1, 11)
                ),
                lexer.get_tokens()[5]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::RightBrace,
                    "}".to_string(),
                    ("test.c".to_string(), 1, 13)
                ),
                lexer.get_tokens()[6]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Else,
                    "else".to_string(),
                    ("test.c".to_string(), 1, 15)
                ),
                lexer.get_tokens()[7]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::LeftBrace,
                    "{".to_string(),
                    ("test.c".to_string(), 1, 20)
                ),
                lexer.get_tokens()[8]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Variable,
                    "e".to_string(),
                    ("test.c".to_string(), 1, 22)
                ),
                lexer.get_tokens()[9]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Assign,
                    "=".to_string(),
                    ("test.c".to_string(), 1, 24)
                ),
                lexer.get_tokens()[10]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Number,
                    "3".to_string(),
                    ("test.c".to_string(), 1, 26)
                ),
                lexer.get_tokens()[11]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::SemiColon,
                    ";".to_string(),
                    ("test.c".to_string(), 1, 27)
                ),
                lexer.get_tokens()[12]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::RightBrace,
                    "}".to_string(),
                    ("test.c".to_string(), 1, 29)
                ),
                lexer.get_tokens()[13]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string(), ("test.c".to_string(), 1, 29)),
                lexer.get_tokens()[14]
            );
        }
        {
            let input = "while(a == b) { c = 3; }".to_string();
            let mut lexer = LexicalAnalysis::new("test.c".to_string(), &input);

            lexer.read_token();

            assert_eq!(
                TokenInfo::new(
                    Token::While,
                    "while".to_string(),
                    ("test.c".to_string(), 1, 1)
                ),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::LeftParen,
                    "(".to_string(),
                    ("test.c".to_string(), 1, 6)
                ),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Variable,
                    "a".to_string(),
                    ("test.c".to_string(), 1, 7)
                ),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(Token::Equal, "==".to_string(), ("test.c".to_string(), 1, 9)),
                lexer.get_tokens()[3]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Variable,
                    "b".to_string(),
                    ("test.c".to_string(), 1, 12)
                ),
                lexer.get_tokens()[4]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::RightParen,
                    ")".to_string(),
                    ("test.c".to_string(), 1, 13)
                ),
                lexer.get_tokens()[5]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::LeftBrace,
                    "{".to_string(),
                    ("test.c".to_string(), 1, 15)
                ),
                lexer.get_tokens()[6]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Variable,
                    "c".to_string(),
                    ("test.c".to_string(), 1, 17)
                ),
                lexer.get_tokens()[7]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Assign,
                    "=".to_string(),
                    ("test.c".to_string(), 1, 19)
                ),
                lexer.get_tokens()[8]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Number,
                    "3".to_string(),
                    ("test.c".to_string(), 1, 21)
                ),
                lexer.get_tokens()[9]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::SemiColon,
                    ";".to_string(),
                    ("test.c".to_string(), 1, 22)
                ),
                lexer.get_tokens()[10]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::RightBrace,
                    "}".to_string(),
                    ("test.c".to_string(), 1, 24)
                ),
                lexer.get_tokens()[11]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string(), ("test.c".to_string(), 1, 24)),
                lexer.get_tokens()[12]
            );
        }
        {
            let input = "return 0;".to_string();
            let mut lexer = LexicalAnalysis::new("test.c".to_string(), &input);

            lexer.read_token();
            assert_eq!(
                TokenInfo::new(
                    Token::Return,
                    "return".to_string(),
                    ("test.c".to_string(), 1, 1)
                ),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "0".to_string(), ("test.c".to_string(), 1, 8)),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::SemiColon,
                    ";".to_string(),
                    ("test.c".to_string(), 1, 9)
                ),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string(), ("test.c".to_string(), 1, 9)),
                lexer.get_tokens()[3]
            );
        }
        {
            let input = "2++".to_string();
            let mut lexer = LexicalAnalysis::new("test.c".to_string(), &input);

            lexer.read_token();

            assert_eq!(
                TokenInfo::new(Token::Number, "2".to_string(), ("test.c".to_string(), 1, 1)),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(Token::Inc, "++".to_string(), ("test.c".to_string(), 1, 2)),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string(), ("test.c".to_string(), 1, 3)),
                lexer.get_tokens()[2]
            );
        }
        {
            let input = "++2".to_string();
            let mut lexer = LexicalAnalysis::new("test.c".to_string(), &input);

            lexer.read_token();

            assert_eq!(
                TokenInfo::new(Token::Inc, "++".to_string(), ("test.c".to_string(), 1, 1)),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "2".to_string(), ("test.c".to_string(), 1, 3)),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string(), ("test.c".to_string(), 1, 3)),
                lexer.get_tokens()[2]
            );
        }
        {
            let input = "2--".to_string();
            let mut lexer = LexicalAnalysis::new("test.c".to_string(), &input);

            lexer.read_token();

            assert_eq!(
                TokenInfo::new(Token::Number, "2".to_string(), ("test.c".to_string(), 1, 1)),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(Token::Dec, "--".to_string(), ("test.c".to_string(), 1, 2)),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string(), ("test.c".to_string(), 1, 3)),
                lexer.get_tokens()[2]
            );
        }
        {
            let input = "--2".to_string();
            let mut lexer = LexicalAnalysis::new("test.c".to_string(), &input);

            lexer.read_token();

            assert_eq!(
                TokenInfo::new(Token::Dec, "--".to_string(), ("test.c".to_string(), 1, 1)),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "2".to_string(), ("test.c".to_string(), 1, 3)),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string(), ("test.c".to_string(), 1, 3)),
                lexer.get_tokens()[2]
            );
        }
    }

    #[test]
    fn test_type() {
        {
            let input = "int a = 2;".to_string();
            let mut lexer = LexicalAnalysis::new("test.c".to_string(), &input);

            lexer.read_token();
            assert_eq!(
                TokenInfo::new(Token::Int, "int".to_string(), ("test.c".to_string(), 1, 1)),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Variable,
                    "a".to_string(),
                    ("test.c".to_string(), 1, 5)
                ),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(Token::Assign, "=".to_string(), ("test.c".to_string(), 1, 7)),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "2".to_string(), ("test.c".to_string(), 1, 9)),
                lexer.get_tokens()[3]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::SemiColon,
                    ";".to_string(),
                    ("test.c".to_string(), 1, 10)
                ),
                lexer.get_tokens()[4]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string(), ("test.c".to_string(), 1, 10)),
                lexer.get_tokens()[5]
            );
        }
    }

    #[test]
    fn test_variable() {
        {
            let input = "int ifi = 0;".to_string();
            let mut lexer = LexicalAnalysis::new("test.c".to_string(), &input);

            lexer.read_token();
            assert_eq!(
                TokenInfo::new(Token::Int, "int".to_string(), ("test.c".to_string(), 1, 1)),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Variable,
                    "ifi".to_string(),
                    ("test.c".to_string(), 1, 5)
                ),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(Token::Assign, "=".to_string(), ("test.c".to_string(), 1, 9)),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Number,
                    "0".to_string(),
                    ("test.c".to_string(), 1, 11)
                ),
                lexer.get_tokens()[3]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::SemiColon,
                    ";".to_string(),
                    ("test.c".to_string(), 1, 12)
                ),
                lexer.get_tokens()[4]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string(), ("test.c".to_string(), 1, 12)),
                lexer.get_tokens()[5]
            );
        }
        {
            let input = "int elsee = 0;".to_string();
            let mut lexer = LexicalAnalysis::new("test.c".to_string(), &input);

            lexer.read_token();
            assert_eq!(
                TokenInfo::new(Token::Int, "int".to_string(), ("test.c".to_string(), 1, 1)),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Variable,
                    "elsee".to_string(),
                    ("test.c".to_string(), 1, 5)
                ),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Assign,
                    "=".to_string(),
                    ("test.c".to_string(), 1, 11)
                ),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Number,
                    "0".to_string(),
                    ("test.c".to_string(), 1, 13)
                ),
                lexer.get_tokens()[3]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::SemiColon,
                    ";".to_string(),
                    ("test.c".to_string(), 1, 14)
                ),
                lexer.get_tokens()[4]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string(), ("test.c".to_string(), 1, 14)),
                lexer.get_tokens()[5]
            );
        }
        {
            let input = "int do_ = 0;".to_string();
            let mut lexer = LexicalAnalysis::new("test.c".to_string(), &input);

            lexer.read_token();
            assert_eq!(
                TokenInfo::new(Token::Int, "int".to_string(), ("test.c".to_string(), 1, 1)),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Variable,
                    "do_".to_string(),
                    ("test.c".to_string(), 1, 5)
                ),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(Token::Assign, "=".to_string(), ("test.c".to_string(), 1, 9)),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Number,
                    "0".to_string(),
                    ("test.c".to_string(), 1, 11)
                ),
                lexer.get_tokens()[3]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::SemiColon,
                    ";".to_string(),
                    ("test.c".to_string(), 1, 12)
                ),
                lexer.get_tokens()[4]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string(), ("test.c".to_string(), 1, 12)),
                lexer.get_tokens()[5]
            );
        }
        {
            let input = "int while0 = 0;".to_string();
            let mut lexer = LexicalAnalysis::new("test.c".to_string(), &input);

            lexer.read_token();
            assert_eq!(
                TokenInfo::new(Token::Int, "int".to_string(), ("test.c".to_string(), 1, 1)),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Variable,
                    "while0".to_string(),
                    ("test.c".to_string(), 1, 5)
                ),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Assign,
                    "=".to_string(),
                    ("test.c".to_string(), 1, 12)
                ),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Number,
                    "0".to_string(),
                    ("test.c".to_string(), 1, 14)
                ),
                lexer.get_tokens()[3]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::SemiColon,
                    ";".to_string(),
                    ("test.c".to_string(), 1, 15)
                ),
                lexer.get_tokens()[4]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string(), ("test.c".to_string(), 1, 15)),
                lexer.get_tokens()[5]
            );
        }
        {
            let input = "int forf = 0;".to_string();
            let mut lexer = LexicalAnalysis::new("test.c".to_string(), &input);

            lexer.read_token();
            assert_eq!(
                TokenInfo::new(Token::Int, "int".to_string(), ("test.c".to_string(), 1, 1)),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Variable,
                    "forf".to_string(),
                    ("test.c".to_string(), 1, 5)
                ),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Assign,
                    "=".to_string(),
                    ("test.c".to_string(), 1, 10)
                ),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Number,
                    "0".to_string(),
                    ("test.c".to_string(), 1, 12)
                ),
                lexer.get_tokens()[3]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::SemiColon,
                    ";".to_string(),
                    ("test.c".to_string(), 1, 13)
                ),
                lexer.get_tokens()[4]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string(), ("test.c".to_string(), 1, 13)),
                lexer.get_tokens()[5]
            );
        }
        {
            let input = "int break_ = 0;".to_string();
            let mut lexer = LexicalAnalysis::new("test.c".to_string(), &input);

            lexer.read_token();
            assert_eq!(
                TokenInfo::new(Token::Int, "int".to_string(), ("test.c".to_string(), 1, 1)),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Variable,
                    "break_".to_string(),
                    ("test.c".to_string(), 1, 5)
                ),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Assign,
                    "=".to_string(),
                    ("test.c".to_string(), 1, 12)
                ),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Number,
                    "0".to_string(),
                    ("test.c".to_string(), 1, 14)
                ),
                lexer.get_tokens()[3]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::SemiColon,
                    ";".to_string(),
                    ("test.c".to_string(), 1, 15)
                ),
                lexer.get_tokens()[4]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string(), ("test.c".to_string(), 1, 15)),
                lexer.get_tokens()[5]
            );
        }
        {
            let input = "int continue1 = 0;".to_string();
            let mut lexer = LexicalAnalysis::new("test.c".to_string(), &input);

            lexer.read_token();
            assert_eq!(
                TokenInfo::new(Token::Int, "int".to_string(), ("test.c".to_string(), 1, 1)),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Variable,
                    "continue1".to_string(),
                    ("test.c".to_string(), 1, 5)
                ),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Assign,
                    "=".to_string(),
                    ("test.c".to_string(), 1, 15)
                ),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Number,
                    "0".to_string(),
                    ("test.c".to_string(), 1, 17)
                ),
                lexer.get_tokens()[3]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::SemiColon,
                    ";".to_string(),
                    ("test.c".to_string(), 1, 18)
                ),
                lexer.get_tokens()[4]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string(), ("test.c".to_string(), 1, 18)),
                lexer.get_tokens()[5]
            );
        }
        {
            let input = "int return_val = 0;".to_string();
            let mut lexer = LexicalAnalysis::new("test.c".to_string(), &input);

            lexer.read_token();
            assert_eq!(
                TokenInfo::new(Token::Int, "int".to_string(), ("test.c".to_string(), 1, 1)),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Variable,
                    "return_val".to_string(),
                    ("test.c".to_string(), 1, 5)
                ),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Assign,
                    "=".to_string(),
                    ("test.c".to_string(), 1, 16)
                ),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Number,
                    "0".to_string(),
                    ("test.c".to_string(), 1, 18)
                ),
                lexer.get_tokens()[3]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::SemiColon,
                    ";".to_string(),
                    ("test.c".to_string(), 1, 19)
                ),
                lexer.get_tokens()[4]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string(), ("test.c".to_string(), 1, 19)),
                lexer.get_tokens()[5]
            );
        }
        {
            let input = "int* a = 2;".to_string();
            let mut lexer = LexicalAnalysis::new("test.c".to_string(), &input);

            lexer.read_token();
            assert_eq!(
                TokenInfo::new(
                    Token::IntPointer,
                    "int*".to_string(),
                    ("test.c".to_string(), 1, 1)
                ),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Variable,
                    "a".to_string(),
                    ("test.c".to_string(), 1, 6)
                ),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(Token::Assign, "=".to_string(), ("test.c".to_string(), 1, 8)),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Number,
                    "2".to_string(),
                    ("test.c".to_string(), 1, 10)
                ),
                lexer.get_tokens()[3]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::SemiColon,
                    ";".to_string(),
                    ("test.c".to_string(), 1, 11)
                ),
                lexer.get_tokens()[4]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string(), ("test.c".to_string(), 1, 11)),
                lexer.get_tokens()[5]
            );
        }
        {
            let input = "int *a = 2;".to_string();
            let mut lexer = LexicalAnalysis::new("test.c".to_string(), &input);

            lexer.read_token();
            assert_eq!(
                TokenInfo::new(
                    Token::IntPointer,
                    "int*".to_string(),
                    ("test.c".to_string(), 1, 1)
                ),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Variable,
                    "a".to_string(),
                    ("test.c".to_string(), 1, 6)
                ),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(Token::Assign, "=".to_string(), ("test.c".to_string(), 1, 8)),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Number,
                    "2".to_string(),
                    ("test.c".to_string(), 1, 10)
                ),
                lexer.get_tokens()[3]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::SemiColon,
                    ";".to_string(),
                    ("test.c".to_string(), 1, 11)
                ),
                lexer.get_tokens()[4]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string(), ("test.c".to_string(), 1, 11)),
                lexer.get_tokens()[5]
            );
        }
        {
            let input = "char ifi = 0;".to_string();
            let mut lexer = LexicalAnalysis::new("test.c".to_string(), &input);

            lexer.read_token();
            assert_eq!(
                TokenInfo::new(
                    Token::Char,
                    "char".to_string(),
                    ("test.c".to_string(), 1, 1)
                ),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Variable,
                    "ifi".to_string(),
                    ("test.c".to_string(), 1, 6)
                ),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Assign,
                    "=".to_string(),
                    ("test.c".to_string(), 1, 10)
                ),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Number,
                    "0".to_string(),
                    ("test.c".to_string(), 1, 12)
                ),
                lexer.get_tokens()[3]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::SemiColon,
                    ";".to_string(),
                    ("test.c".to_string(), 1, 13)
                ),
                lexer.get_tokens()[4]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string(), ("test.c".to_string(), 1, 13)),
                lexer.get_tokens()[5]
            );
        }
    }

    #[test]
    fn test_array() {
        {
            let input = "int a[3];".to_string();
            let mut lexer = LexicalAnalysis::new("test.c".to_string(), &input);

            lexer.read_token();
            assert_eq!(
                TokenInfo::new(Token::Int, "int".to_string(), ("test.c".to_string(), 1, 1)),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Variable,
                    "a".to_string(),
                    ("test.c".to_string(), 1, 5)
                ),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::LeftBracket,
                    "[".to_string(),
                    ("test.c".to_string(), 1, 6)
                ),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "3".to_string(), ("test.c".to_string(), 1, 7)),
                lexer.get_tokens()[3]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::RightBracket,
                    "]".to_string(),
                    ("test.c".to_string(), 1, 8)
                ),
                lexer.get_tokens()[4]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::SemiColon,
                    ";".to_string(),
                    ("test.c".to_string(), 1, 9)
                ),
                lexer.get_tokens()[5]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string(), ("test.c".to_string(), 1, 9)),
                lexer.get_tokens()[6]
            );
        }
    }

    #[test]
    fn test_comment() {
        {
            let input = "2 + 1 / 3 * 7\n // comment".to_string();
            let mut lexer = LexicalAnalysis::new("test.c".to_string(), &input);

            lexer.read_token();

            assert_eq!(
                TokenInfo::new(Token::Number, "2".to_string(), ("test.c".to_string(), 1, 1)),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(Token::Plus, "+".to_string(), ("test.c".to_string(), 1, 3)),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "1".to_string(), ("test.c".to_string(), 1, 5)),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Division,
                    "/".to_string(),
                    ("test.c".to_string(), 1, 7)
                ),
                lexer.get_tokens()[3]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "3".to_string(), ("test.c".to_string(), 1, 9)),
                lexer.get_tokens()[4]
            );
            assert_eq!(
                TokenInfo::new(Token::Multi, "*".to_string(), ("test.c".to_string(), 1, 11)),
                lexer.get_tokens()[5]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Number,
                    "7".to_string(),
                    ("test.c".to_string(), 1, 13)
                ),
                lexer.get_tokens()[6]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string(), ("test.c".to_string(), 2, 13)),
                lexer.get_tokens()[7]
            );
        }
        {
            let input = "// comment\n2 + 1 / 3 * 7".to_string();
            let mut lexer = LexicalAnalysis::new("test.c".to_string(), &input);

            lexer.read_token();

            assert_eq!(
                TokenInfo::new(Token::Number, "2".to_string(), ("test.c".to_string(), 2, 1)),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(Token::Plus, "+".to_string(), ("test.c".to_string(), 2, 3)),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "1".to_string(), ("test.c".to_string(), 2, 5)),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Division,
                    "/".to_string(),
                    ("test.c".to_string(), 2, 7)
                ),
                lexer.get_tokens()[3]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "3".to_string(), ("test.c".to_string(), 2, 9)),
                lexer.get_tokens()[4]
            );
            assert_eq!(
                TokenInfo::new(Token::Multi, "*".to_string(), ("test.c".to_string(), 2, 11)),
                lexer.get_tokens()[5]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Number,
                    "7".to_string(),
                    ("test.c".to_string(), 2, 13)
                ),
                lexer.get_tokens()[6]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string(), ("test.c".to_string(), 2, 13)),
                lexer.get_tokens()[7]
            );
        }
    }

    #[test]
    fn test_string() {
        {
            let input = r#""test""#.to_string();
            let mut lexer = LexicalAnalysis::new("test.c".to_string(), &input);

            lexer.read_token();
            assert_eq!(
                TokenInfo::new(
                    Token::StringLiteral,
                    "test".to_string(),
                    ("test.c".to_string(), 1, 1)
                ),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string(), ("test.c".to_string(), 1, 6)),
                lexer.get_tokens()[1]
            );
        }
        {
            let input = r#""test, test world""#.to_string();
            let mut lexer = LexicalAnalysis::new("test.c".to_string(), &input);

            lexer.read_token();
            assert_eq!(
                TokenInfo::new(
                    Token::StringLiteral,
                    "test, test world".to_string(),
                    ("test.c".to_string(), 1, 1)
                ),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string(), ("test.c".to_string(), 1, 18)),
                lexer.get_tokens()[1]
            );
        }
    }

    #[test]
    fn test_sizeof() {
        {
            let input = "sizeof(a);".to_string();
            let mut lexer = LexicalAnalysis::new("test.c".to_string(), &input);

            lexer.read_token();
            assert_eq!(
                TokenInfo::new(
                    Token::SizeOf,
                    "sizeof".to_string(),
                    ("test.c".to_string(), 1, 1)
                ),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::LeftParen,
                    "(".to_string(),
                    ("test.c".to_string(), 1, 7)
                ),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Variable,
                    "a".to_string(),
                    ("test.c".to_string(), 1, 8)
                ),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::RightParen,
                    ")".to_string(),
                    ("test.c".to_string(), 1, 9)
                ),
                lexer.get_tokens()[3]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::SemiColon,
                    ";".to_string(),
                    ("test.c".to_string(), 1, 10)
                ),
                lexer.get_tokens()[4]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string(), ("test.c".to_string(), 1, 10)),
                lexer.get_tokens()[5]
            );
        }
    }

    #[test]
    fn test_plus_assign() {
        {
            let input = "a += 1;".to_string();
            let mut lexer = LexicalAnalysis::new("test.c".to_string(), &input);

            lexer.read_token();
            assert_eq!(
                TokenInfo::new(
                    Token::Variable,
                    "a".to_string(),
                    ("test.c".to_string(), 1, 1)
                ),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::PlusAssign,
                    "+=".to_string(),
                    ("test.c".to_string(), 1, 3)
                ),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Number,
                    "1".to_string(),
                    ("test.c".to_string(), 1, 6)
                ),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::SemiColon,
                    ";".to_string(),
                    ("test.c".to_string(), 1, 7)
                ),
                lexer.get_tokens()[3]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string(), ("test.c".to_string(), 1, 7)),
                lexer.get_tokens()[4]
            );
        }
    }

    #[test]
    fn test_minus_assign() {
        {
            let input = "a -= 1;".to_string();
            let mut lexer = LexicalAnalysis::new("test.c".to_string(), &input);

            lexer.read_token();
            assert_eq!(
                TokenInfo::new(
                    Token::Variable,
                    "a".to_string(),
                    ("test.c".to_string(), 1, 1)
                ),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::MinusAssign,
                    "-=".to_string(),
                    ("test.c".to_string(), 1, 3)
                ),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Number,
                    "1".to_string(),
                    ("test.c".to_string(), 1, 6)
                ),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::SemiColon,
                    ";".to_string(),
                    ("test.c".to_string(), 1, 7)
                ),
                lexer.get_tokens()[3]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string(), ("test.c".to_string(), 1, 7)),
                lexer.get_tokens()[4]
            );
        }
    }

    #[test]
    fn test_multiple_assign() {
        {
            let input = "a *= 1;".to_string();
            let mut lexer = LexicalAnalysis::new("test.c".to_string(), &input);

            lexer.read_token();
            assert_eq!(
                TokenInfo::new(
                    Token::Variable,
                    "a".to_string(),
                    ("test.c".to_string(), 1, 1)
                ),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::MultipleAssign,
                    "*=".to_string(),
                    ("test.c".to_string(), 1, 3)
                ),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Number,
                    "1".to_string(),
                    ("test.c".to_string(), 1, 6)
                ),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::SemiColon,
                    ";".to_string(),
                    ("test.c".to_string(), 1, 7)
                ),
                lexer.get_tokens()[3]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string(), ("test.c".to_string(), 1, 7)),
                lexer.get_tokens()[4]
            );
        }
    }

    #[test]
    fn test_division_assign() {
        {
            let input = "a /= 1;".to_string();
            let mut lexer = LexicalAnalysis::new("test.c".to_string(), &input);

            lexer.read_token();
            assert_eq!(
                TokenInfo::new(
                    Token::Variable,
                    "a".to_string(),
                    ("test.c".to_string(), 1, 1)
                ),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::DivisionAssign,
                    "/=".to_string(),
                    ("test.c".to_string(), 1, 3)
                ),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Number,
                    "1".to_string(),
                    ("test.c".to_string(), 1, 6)
                ),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::SemiColon,
                    ";".to_string(),
                    ("test.c".to_string(), 1, 7)
                ),
                lexer.get_tokens()[3]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string(), ("test.c".to_string(), 1, 7)),
                lexer.get_tokens()[4]
            );
        }
    }

    #[test]
    fn test_remainder_assign() {
        {
            let input = "a %= 1;".to_string();
            let mut lexer = LexicalAnalysis::new("test.c".to_string(), &input);

            lexer.read_token();
            assert_eq!(
                TokenInfo::new(
                    Token::Variable,
                    "a".to_string(),
                    ("test.c".to_string(), 1, 1)
                ),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::RemainderAssign,
                    "%=".to_string(),
                    ("test.c".to_string(), 1, 3)
                ),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::Number,
                    "1".to_string(),
                    ("test.c".to_string(), 1, 6)
                ),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(
                    Token::SemiColon,
                    ";".to_string(),
                    ("test.c".to_string(), 1, 7)
                ),
                lexer.get_tokens()[3]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string(), ("test.c".to_string(), 1, 7)),
                lexer.get_tokens()[4]
            );
        }
    }
}
