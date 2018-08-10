use token::TokenInfo;
use token::Token;

/**
 * 抽象構文木.
 *
 * 以下の文法を元に抽象構文木を作成する.
 *
 * expr ::= term "+" term
 * term ::= factor
 * factor ::= NUMBER
 */

type AstNode = Box<Node>;

// 抽象構文木ノード.
#[derive(Debug,Clone)]
pub struct Node {
    token: TokenInfo,        // トークン情報.
    left: Option<AstNode>,   // 左ノード.
    right: Option<AstNode>,  // 右ノード.
}

#[derive(Debug,Clone)]
pub struct Ast<'a> {
    tokens: &'a Vec<TokenInfo>,     // トークン配列.
    current_pos: usize,             // 現在読み取り位置.
    current_node: Option<AstNode>,  // 現在ノード.
}

// AST実装.
impl<'a> Ast<'a> {
    // コンストラクタ.
    pub fn new (tokens: &Vec<TokenInfo>) -> Ast {
        Ast {
            current_node: None,
            current_pos: 0,
            tokens: tokens
        }
    }

    // トークン列を受け取り、抽象構文木を返す.
    pub fn parse(&mut self) -> AstNode {
        // 文法に従いながら解析を行う.
        let _token = self.next();
        match _token.get_token_type() {
            Token::Number => {
                self.expr()
            }
            _ => {
                Box::new(Node::new(TokenInfo::new(Token::Unknown, "".to_string())))
            }
        }
    }

    // トークン読み取り.
    fn next(&mut self) -> TokenInfo {
        self.tokens[self.current_pos].clone()
    }

    // 読み取り位置更新.
    fn consume(&mut self) {
        self.current_pos = self.current_pos + 1;
    }

    // 現在位置を巻き戻す.
    fn recover(&mut self, i: usize) {
        self.current_pos = self.current_pos - i
    }

    // expression.
    fn expr(&mut self) -> AstNode {
        let left_token = self.next();
        self.consume();
        let left = self.term(left_token);
        let ope_token = self.next();

        if Token::Plus != ope_token.get_token_type() {
            // トークンを巻き戻す.
            self.recover(1);
            return Box::new(Node::new(TokenInfo::new(Token::Unknown, "".to_string())))
        }

        // 加算演算子生成.
        self.consume();
        let right_token = self.next();
        self.consume();
        let right = self.term(right_token);
        self.ope_add(ope_token, left, right)
    }

    // 加算演算子.
    fn ope_add(&mut self, token: TokenInfo, left: AstNode, right: AstNode) -> AstNode {
        Box::new(Node { token: token, left: Some(left), right: Some(right) })
    }

    // ターム.
    fn term(&mut self, token: TokenInfo) -> AstNode {
        self.factor(token)
    }

    // ファクター.
    fn factor(&mut self, token: TokenInfo) -> AstNode {
        Box::new(Node::new(token))
    }
}

impl Node {
    // コンストラクタ.
    pub fn new(token: TokenInfo) -> Node {
        Node { token: token, left: None, right: None }
    }
}
