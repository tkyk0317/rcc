use token::{Token, TokenInfo};

#[doc = " 字句解析"]
pub struct LexicalAnalysis<'a> {
    input: &'a str,
    pos: usize,
    tokens: Vec<TokenInfo>,
}

impl<'a> LexicalAnalysis<'a> {
    // コンストラクタ.
    pub fn new(input: &'a str) -> LexicalAnalysis {
        LexicalAnalysis {
            input: input,
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
        while false == self.is_eof() {
            // 空白、改行などは読み飛ばし.
            self.skip_ascii_whitespace();

            // 一文字読み取って、トークン生成.
            match self.next() {
                Some(v) => {
                    let token = match v {
                        s if true == s.is_alphabetic() || s == '_' => {
                            if let Some(t) = self.generate_type(s) {
                                t
                            }
                            else if let Some(t) = self.generate_statement(s) {
                                t
                            } else {
                                self.generate_variable_token(s)
                            }
                        }
                        '=' => {
                            if true == self.is_equal(v) {
                                self.skip(1);
                                TokenInfo::new(Token::Equal, "==".to_string())
                            } else {
                                TokenInfo::new(Token::Assign, v.to_string())
                            }
                        }
                        '!' => {
                            if true == self.is_not_equal(v) {
                                self.skip(1);
                                TokenInfo::new(Token::NotEqual, "!=".to_string())
                            } else {
                                TokenInfo::new(Token::Not, v.to_string())
                            }
                        }
                        '>' => {
                            if true == self.is_greater_than_equal(v) {
                                self.skip(1);
                                TokenInfo::new(Token::GreaterThanEqual, ">=".to_string())
                            } else if true == self.is_right_shift(v) {
                                self.skip(1);
                                TokenInfo::new(Token::RightShift, ">>".to_string())
                            } else {
                                TokenInfo::new(Token::GreaterThan, v.to_string())
                            }
                        }
                        '<' => {
                            if true == self.is_less_than_equal(v) {
                                self.skip(1);
                                TokenInfo::new(Token::LessThanEqual, "<=".to_string())
                            } else if true == self.is_left_shift(v) {
                                self.skip(1);
                                TokenInfo::new(Token::LeftShift, "<<".to_string())
                            } else {
                                TokenInfo::new(Token::LessThan, v.to_string())
                            }
                        }
                        '&' => {
                            if true == self.is_logical_and(v) {
                                self.skip(1);
                                TokenInfo::new(Token::LogicalAnd, "&&".to_string())
                            } else {
                                TokenInfo::new(Token::And, v.to_string())
                            }
                        }
                        '|' => {
                            if true == self.is_logical_or(v) {
                                self.skip(1);
                                TokenInfo::new(Token::LogicalOr, "||".to_string())
                            } else {
                                TokenInfo::new(Token::BitOr, v.to_string())
                            }
                        }
                        '^' => TokenInfo::new(Token::BitXor, v.to_string()),
                        '~' => TokenInfo::new(Token::BitReverse, v.to_string()),
                        '+' => TokenInfo::new(Token::Plus, v.to_string()),
                        '-' => TokenInfo::new(Token::Minus, v.to_string()),
                        '*' => TokenInfo::new(Token::Multi, v.to_string()),
                        '/' => TokenInfo::new(Token::Division, v.to_string()),
                        '%' => TokenInfo::new(Token::Remainder, v.to_string()),
                        '(' => TokenInfo::new(Token::LeftParen, v.to_string()),
                        ')' => TokenInfo::new(Token::RightParen, v.to_string()),
                        '{' => TokenInfo::new(Token::LeftBrace, v.to_string()),
                        '}' => TokenInfo::new(Token::RightBrace, v.to_string()),
                        '?' => TokenInfo::new(Token::Question, v.to_string()),
                        ':' => TokenInfo::new(Token::Colon, v.to_string()),
                        ';' => TokenInfo::new(Token::SemiColon, v.to_string()),
                        ',' => TokenInfo::new(Token::Comma, v.to_string()),
                        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                            self.generate_number_token(v)
                        }
                        _ => panic!("Not Support Lexer {}", v),
                    };
                    self.tokens.push(token);
                }
                _ => {}
            }
        }
        self.tokens
            .push(TokenInfo::new(Token::End, "End".to_string()));
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
            return (d.0, d.1);
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

    // 空白や改行、タブをスキップ.
    fn skip_ascii_whitespace(&mut self) {
        while false == self.is_eof() && self.read().is_ascii_whitespace() {
            self.skip(1);
        }
    }

    // 文字をスキップ.
    fn skip(&mut self, i: usize) {
        self.pos += i;
    }

    // 文字読み取り位置を戻す.
    fn back(&mut self, n: usize) {
        self.pos -= n;
    }

    // 文字列終端チェック.
    fn is_eof(&self) -> bool {
        self.pos > (self.input.len() - 1)
    }

    // 変数候補チェック.
    fn is_variable(&self, c: char) -> bool {
        c.is_alphabetic() || c == '_' || c.is_digit(10)
    }

    // 数値トークン生成.
    fn generate_number_token(&mut self, v: char) -> TokenInfo {
        let mut s = String::new();
        s.push(v);

        while false == self.is_eof() && true == self.read().is_digit(10) {
            s.push(
                self.next()
                    .expect("lexer.rs(generate_number_token): cannot read next char"),
            );
        }
        TokenInfo::new(Token::Number, s)
    }

    // 変数トークン生成.
    fn generate_variable_token(&mut self, v: char) -> TokenInfo {
        let mut s = String::new();
        s.push(v);
        while false == self.is_eof() && self.is_variable(self.read()) {
            s.push(self.read());
            self.skip(1);
        }
        TokenInfo::new(Token::Variable, s)
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

    // type作成
    fn generate_type(&mut self, c: char) -> Option<TokenInfo> {
        if true == self.is_type_int(c) {
            self.skip(2);
            // ポインタ型であるかチェック.
            if self.is_pointer() {
                self.skip(1);
                Some(TokenInfo::new(Token::IntPointer, "int*".to_string()))
            }
            else {
                Some(TokenInfo::new(Token::Int, "int".to_string()))
            }
        } else {
            None
        }
    }

    // int型チェック
    fn is_type_int(&mut self, c: char) -> bool {
        let s = self.read_string(3);
        c == 'i' && s.len() == 3 && &s[0..2] == "nt" && false == self.is_variable(s.chars().last().expect("lexer.rs(is_type_int): read error"))
    }

    // ポインタ演算子が存在するか.
    fn is_pointer(&mut self) -> bool {
        // 空白は読み飛ばして、ポインタ型があるかチェック.
        self.skip_ascii_whitespace();
        '*' == self.read()
    }

    // statement作成.
    fn generate_statement(&mut self, c: char) -> Option<TokenInfo> {
        if true == self.is_statement_if(c) {
            self.skip(1);
            Some(TokenInfo::new(Token::If, "if".to_string()))
        } else if true == self.is_statement_else(c) {
            self.skip(3);
            Some(TokenInfo::new(Token::Else, "else".to_string()))
        } else if true == self.is_statement_while(c) {
            self.skip(4);
            Some(TokenInfo::new(Token::While, "while".to_string()))
        } else if true == self.is_statement_for(c) {
            self.skip(2);
            Some(TokenInfo::new(Token::For, "for".to_string()))
        } else if true == self.is_statement_do(c) {
            self.skip(1);
            Some(TokenInfo::new(Token::Do, "do".to_string()))
        } else if true == self.is_statement_continue(c) {
            self.skip(7);
            Some(TokenInfo::new(Token::Continue, "continue".to_string()))
        } else if true == self.is_statement_break(c) {
            self.skip(4);
            Some(TokenInfo::new(Token::Break, "break".to_string()))
        } else if true == self.is_statement_return(c) {
            self.skip(5);
            Some(TokenInfo::new(Token::Return, "return".to_string()))
        } else {
            None
        }
    }

    // if statementチェック.
    fn is_statement_if(&mut self, v: char) -> bool {
        let s = self.read_string(2);
        v == 'i' && s.len() == 2 && "f" == &s[0..1] && false == self.is_variable(s.chars().last().expect("lexer.rs(is_statement_if): read error"))
    }

    // else statementチェック.
    fn is_statement_else(&mut self, v: char) -> bool {
        let s = self.read_string(4);
        v == 'e' && s.len() == 4 && "lse" == &s[0..3] && false == self.is_variable(s.chars().last().expect("lexer.rs(is_statement_else): read error"))
    }

    // while statementチェック.
    fn is_statement_while(&mut self, v: char) -> bool {
        let s = self.read_string(5);
        v == 'w' && s.len() == 5 && "hile" == &s[0..4] && false == self.is_variable(s.chars().last().expect("lexer.rs(is_statement_while): read error"))
    }

    // do-while statementチェック.
    fn is_statement_do(&mut self, v: char) -> bool {
        let s = self.read_string(2);
        v == 'd' && s.len() == 2 && "o" == &s[0..1] && false == self.is_variable(s.chars().last().expect("lexer.rs(is_statement_do): read error"))
    }

    // for statementチェック.
    fn is_statement_for(&mut self, v: char) -> bool {
        let s = self.read_string(3);
        v == 'f' && s.len() == 3 && "or" == &s[0..2] && false == self.is_variable(s.chars().last().expect("lexer.rs(is_statement_for): read error"))
    }

    // continue statementチェック.
    fn is_statement_continue(&mut self, v: char) -> bool {
        let s = self.read_string(8);
        v == 'c' && s.len() == 8 && "ontinue" == &s[0..7] && false == self.is_variable(s.chars().last().expect("lexer.rs(is_statement_continue): read error"))
    }

    // break statementチェック.
    fn is_statement_break(&mut self, v: char) -> bool {
        let s = self.read_string(5);
        v == 'b' && s.len() == 5 && "reak" == &s[0..4] && false == self.is_variable(s.chars().last().expect("lexer.rs(is_statement_break): read error"))
    }

    // return statementチェック.
    fn is_statement_return(&mut self, v: char) -> bool {
        let s = self.read_string(6);
        v == 'r' && s.len() == 6 && "eturn" == &s[0..5] && false == self.is_variable(s.chars().last().expect("lexer.rs(is_statement_return): read error"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        {
            let input = "2 + 1 / 3 * 7".to_string();
            let mut lexer = LexicalAnalysis::new(&input);

            lexer.read_token();

            assert_eq!(
                TokenInfo::new(Token::Number, "2".to_string()),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(Token::Plus, "+".to_string()),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "1".to_string()),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(Token::Division, "/".to_string()),
                lexer.get_tokens()[3]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "3".to_string()),
                lexer.get_tokens()[4]
            );
            assert_eq!(
                TokenInfo::new(Token::Multi, "*".to_string()),
                lexer.get_tokens()[5]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "7".to_string()),
                lexer.get_tokens()[6]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string()),
                lexer.get_tokens()[7]
            );
        }
        {
            let input = "2 >= 1".to_string();
            let mut lexer = LexicalAnalysis::new(&input);

            lexer.read_token();

            assert_eq!(
                TokenInfo::new(Token::Number, "2".to_string()),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(Token::GreaterThanEqual, ">=".to_string()),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "1".to_string()),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string()),
                lexer.get_tokens()[3]
            );
        }
        {
            let input = "2 <= 1".to_string();
            let mut lexer = LexicalAnalysis::new(&input);

            lexer.read_token();

            assert_eq!(
                TokenInfo::new(Token::Number, "2".to_string()),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(Token::LessThanEqual, "<=".to_string()),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "1".to_string()),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string()),
                lexer.get_tokens()[3]
            );
        }
        {
            let input = "2 == 1".to_string();
            let mut lexer = LexicalAnalysis::new(&input);

            lexer.read_token();

            assert_eq!(
                TokenInfo::new(Token::Number, "2".to_string()),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(Token::Equal, "==".to_string()),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "1".to_string()),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string()),
                lexer.get_tokens()[3]
            );
        }
        {
            let input = "2 != 1".to_string();
            let mut lexer = LexicalAnalysis::new(&input);

            lexer.read_token();

            assert_eq!(
                TokenInfo::new(Token::Number, "2".to_string()),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(Token::NotEqual, "!=".to_string()),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "1".to_string()),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string()),
                lexer.get_tokens()[3]
            );
        }
        {
            let input = "2 << 1".to_string();
            let mut lexer = LexicalAnalysis::new(&input);

            lexer.read_token();

            assert_eq!(
                TokenInfo::new(Token::Number, "2".to_string()),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(Token::LeftShift, "<<".to_string()),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "1".to_string()),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string()),
                lexer.get_tokens()[3]
            );
        }
        {
            let input = "2 >> 1".to_string();
            let mut lexer = LexicalAnalysis::new(&input);

            lexer.read_token();

            assert_eq!(
                TokenInfo::new(Token::Number, "2".to_string()),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(Token::RightShift, ">>".to_string()),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "1".to_string()),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string()),
                lexer.get_tokens()[3]
            );
        }
        {
            let input = "~1".to_string();
            let mut lexer = LexicalAnalysis::new(&input);

            lexer.read_token();

            assert_eq!(
                TokenInfo::new(Token::BitReverse, "~".to_string()),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "1".to_string()),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string()),
                lexer.get_tokens()[2]
            );
        }
        {
            let input = "1 + 2;".to_string();
            let mut lexer = LexicalAnalysis::new(&input);

            lexer.read_token();

            assert_eq!(
                TokenInfo::new(Token::Number, "1".to_string()),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(Token::Plus, "+".to_string()),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "2".to_string()),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                lexer.get_tokens()[3]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string()),
                lexer.get_tokens()[4]
            );
        }
        {
            let input = "1 + 2; 3 >= 2".to_string();
            let mut lexer = LexicalAnalysis::new(&input);

            lexer.read_token();

            assert_eq!(
                TokenInfo::new(Token::Number, "1".to_string()),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(Token::Plus, "+".to_string()),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "2".to_string()),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                lexer.get_tokens()[3]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "3".to_string()),
                lexer.get_tokens()[4]
            );
            assert_eq!(
                TokenInfo::new(Token::GreaterThanEqual, ">=".to_string()),
                lexer.get_tokens()[5]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "2".to_string()),
                lexer.get_tokens()[6]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string()),
                lexer.get_tokens()[7]
            );
        }
        {
            let input = "a = 1 + 2;".to_string();
            let mut lexer = LexicalAnalysis::new(&input);

            lexer.read_token();

            assert_eq!(
                TokenInfo::new(Token::Variable, "a".to_string()),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(Token::Assign, "=".to_string()),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "1".to_string()),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(Token::Plus, "+".to_string()),
                lexer.get_tokens()[3]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "2".to_string()),
                lexer.get_tokens()[4]
            );
            assert_eq!(
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                lexer.get_tokens()[5]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string()),
                lexer.get_tokens()[6]
            );
        }
        {
            let input = "_a_aa = 1 + 2;".to_string();
            let mut lexer = LexicalAnalysis::new(&input);

            lexer.read_token();

            assert_eq!(
                TokenInfo::new(Token::Variable, "_a_aa".to_string()),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(Token::Assign, "=".to_string()),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "1".to_string()),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(Token::Plus, "+".to_string()),
                lexer.get_tokens()[3]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "2".to_string()),
                lexer.get_tokens()[4]
            );
            assert_eq!(
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                lexer.get_tokens()[5]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string()),
                lexer.get_tokens()[6]
            );
        }
        {
            let input = "a, b;".to_string();
            let mut lexer = LexicalAnalysis::new(&input);

            lexer.read_token();

            assert_eq!(
                TokenInfo::new(Token::Variable, "a".to_string()),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(Token::Comma, ",".to_string()),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(Token::Variable, "b".to_string()),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                lexer.get_tokens()[3]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string()),
                lexer.get_tokens()[4]
            );
        }
        {
            let input = "{ a = b; }".to_string();
            let mut lexer = LexicalAnalysis::new(&input);

            lexer.read_token();

            assert_eq!(
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(Token::Variable, "a".to_string()),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(Token::Assign, "=".to_string()),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(Token::Variable, "b".to_string()),
                lexer.get_tokens()[3]
            );
            assert_eq!(
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                lexer.get_tokens()[4]
            );
            assert_eq!(
                TokenInfo::new(Token::RightBrace, "}".to_string()),
                lexer.get_tokens()[5]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string()),
                lexer.get_tokens()[6]
            );
        }
        {
            let input = "if { i = 2; }".to_string();
            let mut lexer = LexicalAnalysis::new(&input);

            lexer.read_token();

            assert_eq!(
                TokenInfo::new(Token::If, "if".to_string()),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(Token::Variable, "i".to_string()),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(Token::Assign, "=".to_string()),
                lexer.get_tokens()[3]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "2".to_string()),
                lexer.get_tokens()[4]
            );
            assert_eq!(
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                lexer.get_tokens()[5]
            );
            assert_eq!(
                TokenInfo::new(Token::RightBrace, "}".to_string()),
                lexer.get_tokens()[6]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string()),
                lexer.get_tokens()[7]
            );
        }
        {
            let input = "if { i = 2; } else { e = 3; }".to_string();
            let mut lexer = LexicalAnalysis::new(&input);

            lexer.read_token();

            assert_eq!(
                TokenInfo::new(Token::If, "if".to_string()),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(Token::Variable, "i".to_string()),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(Token::Assign, "=".to_string()),
                lexer.get_tokens()[3]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "2".to_string()),
                lexer.get_tokens()[4]
            );
            assert_eq!(
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                lexer.get_tokens()[5]
            );
            assert_eq!(
                TokenInfo::new(Token::RightBrace, "}".to_string()),
                lexer.get_tokens()[6]
            );
            assert_eq!(
                TokenInfo::new(Token::Else, "else".to_string()),
                lexer.get_tokens()[7]
            );
            assert_eq!(
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                lexer.get_tokens()[8]
            );
            assert_eq!(
                TokenInfo::new(Token::Variable, "e".to_string()),
                lexer.get_tokens()[9]
            );
            assert_eq!(
                TokenInfo::new(Token::Assign, "=".to_string()),
                lexer.get_tokens()[10]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "3".to_string()),
                lexer.get_tokens()[11]
            );
            assert_eq!(
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                lexer.get_tokens()[12]
            );
            assert_eq!(
                TokenInfo::new(Token::RightBrace, "}".to_string()),
                lexer.get_tokens()[13]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string()),
                lexer.get_tokens()[14]
            );
        }
        {
            let input = "while(a == b) { c = 3; }".to_string();
            let mut lexer = LexicalAnalysis::new(&input);

            lexer.read_token();

            assert_eq!(
                TokenInfo::new(Token::While, "while".to_string()),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(Token::LeftParen, "(".to_string()),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(Token::Variable, "a".to_string()),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(Token::Equal, "==".to_string()),
                lexer.get_tokens()[3]
            );
            assert_eq!(
                TokenInfo::new(Token::Variable, "b".to_string()),
                lexer.get_tokens()[4]
            );
            assert_eq!(
                TokenInfo::new(Token::RightParen, ")".to_string()),
                lexer.get_tokens()[5]
            );
            assert_eq!(
                TokenInfo::new(Token::LeftBrace, "{".to_string()),
                lexer.get_tokens()[6]
            );
            assert_eq!(
                TokenInfo::new(Token::Variable, "c".to_string()),
                lexer.get_tokens()[7]
            );
            assert_eq!(
                TokenInfo::new(Token::Assign, "=".to_string()),
                lexer.get_tokens()[8]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "3".to_string()),
                lexer.get_tokens()[9]
            );
            assert_eq!(
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                lexer.get_tokens()[10]
            );
            assert_eq!(
                TokenInfo::new(Token::RightBrace, "}".to_string()),
                lexer.get_tokens()[11]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string()),
                lexer.get_tokens()[12]
            );
        }
        {
            let input = "return 0;".to_string();
            let mut lexer = LexicalAnalysis::new(&input);

            lexer.read_token();
            assert_eq!(
                TokenInfo::new(Token::Return, "return".to_string()),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "0".to_string()),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string()),
                lexer.get_tokens()[3]
            );
        }
    }

    #[test]
    fn test_type() {
        {
            let input = "int a = 2;".to_string();
            let mut lexer = LexicalAnalysis::new(&input);

            lexer.read_token();
            assert_eq!(
                TokenInfo::new(Token::Int, "int".to_string()),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(Token::Variable, "a".to_string()),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(Token::Assign, "=".to_string()),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "2".to_string()),
                lexer.get_tokens()[3]
            );
            assert_eq!(
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                lexer.get_tokens()[4]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string()),
                lexer.get_tokens()[5]
            );
        }
    }

    #[test]
    fn test_variable() {
        {
            let input = "int ifi = 0;".to_string();
            let mut lexer = LexicalAnalysis::new(&input);

            lexer.read_token();
            assert_eq!(
                TokenInfo::new(Token::Int, "int".to_string()),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(Token::Variable, "ifi".to_string()),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(Token::Assign, "=".to_string()),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "0".to_string()),
                lexer.get_tokens()[3]
            );
            assert_eq!(
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                lexer.get_tokens()[4]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string()),
                lexer.get_tokens()[5]
            );
        }
        {
            let input = "int elsee = 0;".to_string();
            let mut lexer = LexicalAnalysis::new(&input);

            lexer.read_token();
            assert_eq!(
                TokenInfo::new(Token::Int, "int".to_string()),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(Token::Variable, "elsee".to_string()),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(Token::Assign, "=".to_string()),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "0".to_string()),
                lexer.get_tokens()[3]
            );
            assert_eq!(
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                lexer.get_tokens()[4]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string()),
                lexer.get_tokens()[5]
            );
        }
        {
            let input = "int do_ = 0;".to_string();
            let mut lexer = LexicalAnalysis::new(&input);

            lexer.read_token();
            assert_eq!(
                TokenInfo::new(Token::Int, "int".to_string()),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(Token::Variable, "do_".to_string()),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(Token::Assign, "=".to_string()),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "0".to_string()),
                lexer.get_tokens()[3]
            );
            assert_eq!(
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                lexer.get_tokens()[4]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string()),
                lexer.get_tokens()[5]
            );
        }
        {
            let input = "int while0 = 0;".to_string();
            let mut lexer = LexicalAnalysis::new(&input);

            lexer.read_token();
            assert_eq!(
                TokenInfo::new(Token::Int, "int".to_string()),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(Token::Variable, "while0".to_string()),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(Token::Assign, "=".to_string()),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "0".to_string()),
                lexer.get_tokens()[3]
            );
            assert_eq!(
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                lexer.get_tokens()[4]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string()),
                lexer.get_tokens()[5]
            );
        }
        {
            let input = "int forf = 0;".to_string();
            let mut lexer = LexicalAnalysis::new(&input);

            lexer.read_token();
            assert_eq!(
                TokenInfo::new(Token::Int, "int".to_string()),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(Token::Variable, "forf".to_string()),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(Token::Assign, "=".to_string()),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "0".to_string()),
                lexer.get_tokens()[3]
            );
            assert_eq!(
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                lexer.get_tokens()[4]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string()),
                lexer.get_tokens()[5]
            );
        }
        {
            let input = "int break_ = 0;".to_string();
            let mut lexer = LexicalAnalysis::new(&input);

            lexer.read_token();
            assert_eq!(
                TokenInfo::new(Token::Int, "int".to_string()),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(Token::Variable, "break_".to_string()),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(Token::Assign, "=".to_string()),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "0".to_string()),
                lexer.get_tokens()[3]
            );
            assert_eq!(
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                lexer.get_tokens()[4]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string()),
                lexer.get_tokens()[5]
            );
        }
        {
            let input = "int continue1 = 0;".to_string();
            let mut lexer = LexicalAnalysis::new(&input);

            lexer.read_token();
            assert_eq!(
                TokenInfo::new(Token::Int, "int".to_string()),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(Token::Variable, "continue1".to_string()),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(Token::Assign, "=".to_string()),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "0".to_string()),
                lexer.get_tokens()[3]
            );
            assert_eq!(
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                lexer.get_tokens()[4]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string()),
                lexer.get_tokens()[5]
            );
        }
        {
            let input = "int return_val = 0;".to_string();
            let mut lexer = LexicalAnalysis::new(&input);

            lexer.read_token();
            assert_eq!(
                TokenInfo::new(Token::Int, "int".to_string()),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(Token::Variable, "return_val".to_string()),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(Token::Assign, "=".to_string()),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "0".to_string()),
                lexer.get_tokens()[3]
            );
            assert_eq!(
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                lexer.get_tokens()[4]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string()),
                lexer.get_tokens()[5]
            );
        }
        {
            let input = "int* a = 2;".to_string();
            let mut lexer = LexicalAnalysis::new(&input);

            lexer.read_token();
            assert_eq!(
                TokenInfo::new(Token::IntPointer, "int*".to_string()),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(Token::Variable, "a".to_string()),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(Token::Assign, "=".to_string()),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "2".to_string()),
                lexer.get_tokens()[3]
            );
            assert_eq!(
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                lexer.get_tokens()[4]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string()),
                lexer.get_tokens()[5]
            );
        }
        {
            let input = "int *a = 2;".to_string();
            let mut lexer = LexicalAnalysis::new(&input);

            lexer.read_token();
            assert_eq!(
                TokenInfo::new(Token::IntPointer, "int*".to_string()),
                lexer.get_tokens()[0]
            );
            assert_eq!(
                TokenInfo::new(Token::Variable, "a".to_string()),
                lexer.get_tokens()[1]
            );
            assert_eq!(
                TokenInfo::new(Token::Assign, "=".to_string()),
                lexer.get_tokens()[2]
            );
            assert_eq!(
                TokenInfo::new(Token::Number, "2".to_string()),
                lexer.get_tokens()[3]
            );
            assert_eq!(
                TokenInfo::new(Token::SemiColon, ";".to_string()),
                lexer.get_tokens()[4]
            );
            assert_eq!(
                TokenInfo::new(Token::End, "End".to_string()),
                lexer.get_tokens()[5]
            );
        }
    }
}
