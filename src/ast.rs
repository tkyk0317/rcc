use token::TokenInfo;
use token::Token;

#[derive(Debug,Clone)]
pub struct Ast<'a> {
    tokens: &'a Vec<TokenInfo>,     // トークン配列.
    current_pos: usize,             // 現在読み取り位置.
    ast: Vec<TokenInfo>,            // 抽象構文木.
}

// AST実装.
//
// 文法.
//   Expr ::= Term '+' | Term - | Expr
//   Term ::= Fact
//   Fact ::= NUMBER
impl<'a> Ast<'a> {
    // コンストラクタ.
    pub fn new (tokens: &Vec<TokenInfo>) -> Ast {
        Ast { current_pos: 0, tokens: tokens, ast: Vec::new(), }
    }

    // トークン列を受け取り、抽象構文木を返す.
    pub fn parse(&mut self) {
        // 文法に従いながら解析を行う.
        let _token = self.next();
        match _token.get_token_type() {
            Token::Number => {
                self.expr();
            }
            _ => panic!("Not Support Token")
        }
    }

    // 抽象構文木.
    pub fn get_ast(&self) -> &Vec<TokenInfo> { &self.ast }

    // トークン読み取り.
    fn next(&mut self) -> TokenInfo {
        if self.current_pos >= self.tokens.len() {
            return TokenInfo::new(Token::Unknown, "".to_string());
        }
        self.tokens[self.current_pos].clone()
    }

    // 読み取り位置更新.
    fn consume(&mut self) { self.current_pos = self.current_pos + 1; }

    // expression.
    fn expr(&mut self) {
        let left = self.factor();
        let ope = self.next();

        if ope.get_token_type() == Token::Plus || ope.get_token_type() == Token::Minus {
            self.consume();
            self.ope_add(ope, left);
            self.expr();
        }
        else {
            self.ast.push(left);
        }
   }

    // factor.
    fn factor(&mut self) -> TokenInfo {
        let token = self.next();
        self.consume();
        token
    }

    // 加算演算子.
    fn ope_add(&mut self, ope: TokenInfo, left: TokenInfo) {
        self.ast.push(ope);
        self.ast.push(left);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_operator() {
        let data =
            vec![
                TokenInfo::new(Token::Number, '1'.to_string()),
                TokenInfo::new(Token::Plus, '+'.to_string()),
                TokenInfo::new(Token::Number, '2'.to_string())
            ];
        let mut ast = Ast::new(&data);
        ast.parse();

        // 期待値確認.
        let tree = ast.get_ast();
        assert_eq!(tree[0], TokenInfo::new(Token::Plus, '+'.to_string()));
        assert_eq!(tree[1], TokenInfo::new(Token::Number, '1'.to_string()));
        assert_eq!(tree[2], TokenInfo::new(Token::Number, '2'.to_string()))
    }

    #[test]
    fn test_add_operator_some_augend() {
        let data =
            vec![
                TokenInfo::new(Token::Number, '1'.to_string()),
                TokenInfo::new(Token::Plus, '+'.to_string()),
                TokenInfo::new(Token::Number, '2'.to_string()),
                TokenInfo::new(Token::Plus, '+'.to_string()),
                TokenInfo::new(Token::Number, '3'.to_string())
            ];
        let mut ast = Ast::new(&data);
        ast.parse();

        // 期待値確認.
        let tree = ast.get_ast();
        assert_eq!(tree[0], TokenInfo::new(Token::Plus, '+'.to_string()));
        assert_eq!(tree[1], TokenInfo::new(Token::Number, '1'.to_string()));
        assert_eq!(tree[2], TokenInfo::new(Token::Plus, '+'.to_string()));
        assert_eq!(tree[3], TokenInfo::new(Token::Number, '2'.to_string()));
        assert_eq!(tree[4], TokenInfo::new(Token::Number, '3'.to_string()))
    }

    #[test]
    fn test_sub_operator() {
        let data =
            vec![
                TokenInfo::new(Token::Number, '1'.to_string()),
                TokenInfo::new(Token::Minus, '-'.to_string()),
                TokenInfo::new(Token::Number, '2'.to_string())
            ];
        let mut ast = Ast::new(&data);
        ast.parse();

        // 期待値確認.
        let tree = ast.get_ast();
        assert_eq!(tree[0], TokenInfo::new(Token::Minus, '-'.to_string()));
        assert_eq!(tree[1], TokenInfo::new(Token::Number, '1'.to_string()));
        assert_eq!(tree[2], TokenInfo::new(Token::Number, '2'.to_string()))
    }

    #[test]
    fn test_sub_operator_some_augent() {
        let data =
            vec![
                TokenInfo::new(Token::Number, '1'.to_string()),
                TokenInfo::new(Token::Minus, '-'.to_string()),
                TokenInfo::new(Token::Number, '2'.to_string()),
                TokenInfo::new(Token::Minus, '-'.to_string()),
                TokenInfo::new(Token::Number, '5'.to_string())
            ];
        let mut ast = Ast::new(&data);
        ast.parse();

        // 期待値確認.
        let tree = ast.get_ast();
        assert_eq!(tree[0], TokenInfo::new(Token::Minus, '-'.to_string()));
        assert_eq!(tree[1], TokenInfo::new(Token::Number, '1'.to_string()));
        assert_eq!(tree[2], TokenInfo::new(Token::Minus, '-'.to_string()));
        assert_eq!(tree[3], TokenInfo::new(Token::Number, '2'.to_string()));
        assert_eq!(tree[4], TokenInfo::new(Token::Number, '5'.to_string()))
    }

    #[test]
    fn test_add_sub_operator() {
        let data =
            vec![
                TokenInfo::new(Token::Number, '1'.to_string()),
                TokenInfo::new(Token::Plus, '+'.to_string()),
                TokenInfo::new(Token::Number, '2'.to_string()),
                TokenInfo::new(Token::Minus, '-'.to_string()),
                TokenInfo::new(Token::Number, '5'.to_string())
            ];
        let mut ast = Ast::new(&data);
        ast.parse();

        // 期待値確認.
        let tree = ast.get_ast();
        assert_eq!(tree[0], TokenInfo::new(Token::Plus, '+'.to_string()));
        assert_eq!(tree[1], TokenInfo::new(Token::Number, '1'.to_string()));
        assert_eq!(tree[2], TokenInfo::new(Token::Minus, '-'.to_string()));
        assert_eq!(tree[3], TokenInfo::new(Token::Number, '2'.to_string()));
        assert_eq!(tree[4], TokenInfo::new(Token::Number, '5'.to_string()))
    }
}
