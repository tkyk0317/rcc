use token::TokenInfo;
use token::Token;

#[doc = " 字句解析"]
pub struct LexicalAnalysis<'a> {
    input: &'a String,
    pos: usize,
    tokens: Vec<TokenInfo>,
}

impl<'a> LexicalAnalysis<'a> {
    // コンストラクタ.
    pub fn new(input: &'a String) -> LexicalAnalysis {
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
            // 空白部分は読み飛ばし.
            self.skip_space();

            // 一文字読み取って、トークン生成.
            match self.next() {
                Some(v) => {
                    let mut token;
                    match v {
                        s if true == s.is_alphabetic() || s == '_' => {
                            if true == self.is_statement_if(s) {
                                self.skip(1);
                                token = TokenInfo::new(Token::If, "if".to_string());
                            }
                            else if true == self.is_statement_else(s) {
                                self.skip(3);
                                token = TokenInfo::new(Token::Else, "else".to_string());
                            }
                            else if true == self.is_statement_while(s) {
                                self.skip(4);
                                token = TokenInfo::new(Token::While, "while".to_string());
                            }
                            else if true == self.is_statement_for(s) {
                                self.skip(2);
                                token = TokenInfo::new(Token::For, "for".to_string());
                            }
                            else if true == self.is_statement_do(s) {
                                self.skip(1);
                                token = TokenInfo::new(Token::Do, "do".to_string());
                            }
                            else if true == self.is_statement_continue(s) {
                                self.skip(7);
                                token = TokenInfo::new(Token::Continue, "continue".to_string());
                            }
                            else if true == self.is_statement_break(s) {
                                self.skip(4);
                                token = TokenInfo::new(Token::Break, "break".to_string());
                            }
                            else if true == self.is_statement_return(s) {
                                self.skip(5);
                                token = TokenInfo::new(Token::Return, "return".to_string());
                            }
                            else {
                                token = self.generate_variable_token(s);
                            }
                        }
                        '=' => {
                            if true == self.is_equal(v) {
                                self.skip(1);
                                token = TokenInfo::new(Token::Equal, "==".to_string());
                            } else {
                                token = TokenInfo::new(Token::Assign, v.to_string());
                            }
                        }
                        '!' => {
                            if true == self.is_not_equal(v) {
                                self.skip(1);
                                token = TokenInfo::new(Token::NotEqual, "!=".to_string());
                            } else {
                                token = TokenInfo::new(Token::Not, v.to_string());
                            }
                        }
                        '>' => {
                            if true == self.is_greater_than_equal(v) {
                                self.skip(1);
                                token = TokenInfo::new(Token::GreaterThanEqual, ">=".to_string());
                            } else if true == self.is_right_shift(v) {
                                self.skip(1);
                                token = TokenInfo::new(Token::RightShift, ">>".to_string());
                            } else {
                                token = TokenInfo::new(Token::GreaterThan, v.to_string());
                            }
                        }
                        '<' => {
                            if true == self.is_less_than_equal(v) {
                                self.skip(1);
                                token = TokenInfo::new(Token::LessThanEqual, "<=".to_string());
                            } else if true == self.is_left_shift(v) {
                                self.skip(1);
                                token = TokenInfo::new(Token::LeftShift, "<<".to_string());
                            } else {
                                token = TokenInfo::new(Token::LessThan, v.to_string());
                            }
                        }
                        '&' => {
                            if true == self.is_logical_and(v) {
                                self.skip(1);
                                token = TokenInfo::new(Token::LogicalAnd, "&&".to_string());
                            } else {
                                token = TokenInfo::new(Token::BitAnd, v.to_string());
                            }
                        }
                        '|' => {
                            if true == self.is_logical_or(v) {
                                self.skip(1);
                                token = TokenInfo::new(Token::LogicalOr, "||".to_string());
                            } else {
                                token = TokenInfo::new(Token::BitOr, v.to_string());
                            }
                        }
                        '^' => token = TokenInfo::new(Token::BitXor, v.to_string()),
                        '~' => token = TokenInfo::new(Token::BitReverse, v.to_string()),
                        '+' => token = TokenInfo::new(Token::Plus, v.to_string()),
                        '-' => token = TokenInfo::new(Token::Minus, v.to_string()),
                        '*' => token = TokenInfo::new(Token::Multi, v.to_string()),
                        '/' => token = TokenInfo::new(Token::Division, v.to_string()),
                        '%' => token = TokenInfo::new(Token::Remainder, v.to_string()),
                        '(' => token = TokenInfo::new(Token::LeftBracket, v.to_string()),
                        ')' => token = TokenInfo::new(Token::RightBracket, v.to_string()),
                        '{' => token = TokenInfo::new(Token::LeftBrace, v.to_string()),
                        '}' => token = TokenInfo::new(Token::RightBrace, v.to_string()),
                        '?' => token = TokenInfo::new(Token::Question, v.to_string()),
                        ':' => token = TokenInfo::new(Token::Colon, v.to_string()),
                        ';' => token = TokenInfo::new(Token::SemiColon, v.to_string()),
                        ',' => token = TokenInfo::new(Token::Comma, v.to_string()),
                        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => token = self.generate_number_token(v),
                        _ => panic!("Not Support Lexer {}", v),
                    }
                    self.tokens.push(token);
                }
                _ => {}
            }
        }
        self.tokens.push(
            TokenInfo::new(Token::End, "End".to_string()),
        );
    }

    // 文字を読み出す.
    fn read(&self) -> char {
        self.input.chars().nth(self.pos).unwrap()
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

    // 空白読み飛ばし.
    fn skip_space(&mut self) {
        while false == self.is_eof() && true == self.read().is_whitespace() {
            self.skip(1);
        }
    }

    // 数値トークン生成.
    fn generate_number_token(&mut self, v: char) -> TokenInfo {
        let mut s = String::new();
        s.push(v);

        while false == self.is_eof() && true == self.read().is_digit(10) {
            s.push(self.next().unwrap());
        }
        TokenInfo::new(Token::Number, s)
    }

    // 変数トークン生成.
    fn generate_variable_token(&mut self, v: char) -> TokenInfo {
        let is_variable = |s: char| s.is_alphabetic() || s == '_' || s.is_digit(10);

        let mut s = String::new();
        s.push(v);

        while false == self.is_eof() && is_variable(self.read()) {
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

    // if statementチェック.
    fn is_statement_if(&mut self, v: char) -> bool {
        v == 'i' && 'f' == self.read()
    }

    // else statementチェック.
    fn is_statement_else(&mut self, v: char) -> bool {
        v == 'e' && "lse" == self.read_string(3)
    }

    // while statementチェック.
    fn is_statement_while(&mut self, v: char) -> bool {
        v == 'w' && "hile" == self.read_string(4)
    }

    // do-while statementチェック.
    fn is_statement_do(&mut self, v: char) -> bool {
        v == 'd' && "o" == self.read_string(1)
    }

    // for statementチェック.
    fn is_statement_for(&mut self, v: char) -> bool {
        v == 'f' && "or" == self.read_string(2)
    }

    // continue statementチェック.
    fn is_statement_continue(&mut self, v: char) -> bool {
        v == 'c' && "ontinue" == self.read_string(7)
    }

    // break statementチェック.
    fn is_statement_break(&mut self, v: char) -> bool {
        v == 'b' && "reak" == self.read_string(4)
    }

    // return statementチェック.
    fn is_statement_return(&mut self, v: char) -> bool {
        v == 'r' && "eturn" == self.read_string(5)
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
                TokenInfo::new(Token::LeftBracket, "(".to_string()),
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
                TokenInfo::new(Token::RightBracket, ")".to_string()),
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
    }
}
