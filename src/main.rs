use std::io;
use std::io::ErrorKind;

#[derive(PartialEq)]
enum Token
{
    Integer(i64),
    Plus,
    Minus,
    Mul,
    Div,
    Lparen,
    Rparen,
    Eof
}

struct Lexer
{
    text: String,
    pos: usize,
    current_char: Option<char>
}

impl Lexer
{
    //Advance the `pos` pointer and set the `current_char` variable.
    fn advance(&mut self)
    {
        self.pos += 1;
        if self.pos > self.text.len() - 1
        {
            self.current_char = None;
        }
        else
        {
            self.current_char = self.text.chars().nth(self.pos)
        }
    }

    fn skip_whitespaces(&mut self)
    {
        while self.current_char != None && self.current_char.unwrap().is_whitespace(){
            self.advance();
        }
    }

    //Return a (multidigit) integer consumed from the input.
    fn integer(&mut self) -> i64 {
        let mut result = String::new();
        while self.current_char != None && self.current_char.unwrap().is_ascii_digit(){
            result.push(self.current_char.unwrap());
            self.advance();
        }
        return result.parse().unwrap();
    }

    //Lexical analyzer (also known as scanner or tokenizer)
    //
    //        This method is responsible for breaking a sentence
    //        apart into tokens. One token at a time.
    fn get_next_token(&mut self) -> Result<Token, ErrorKind>
    {
        while self.current_char != None
        {
            if self.current_char.unwrap().is_whitespace(){
                self.skip_whitespaces();
                continue;
            }

            if self.current_char.unwrap().is_ascii_digit(){
                return Ok(Token::Integer(self.integer()));
            }

            match self.current_char
            {
                Some('+') => {self.advance(); return Ok(Token::Plus)},
                Some('-') => {self.advance(); return Ok(Token::Minus)},
                Some('*') => {self.advance(); return Ok(Token::Mul)},
                Some('/') => {self.advance(); return Ok(Token::Div)},
                Some('(') => {self.advance(); return Ok(Token::Lparen)},
                Some(')') => {self.advance(); return Ok(Token::Rparen)},
                _ => return Err(ErrorKind::InvalidData),
            }
        }

        return Ok(Token::Eof);
    }

    fn create_lexer(text: String) -> Lexer{
        let init_char = text.chars().nth(0);
        Lexer{
            text,
            pos: 0,
            current_char: init_char
        }
    }
}

struct Interpreter<'a>
{
    lexer: &'a mut Lexer,
    current_token: Token
}

impl<'a> Interpreter<'a>
{
    fn eat(&mut self)
    {
        let res = self.lexer.get_next_token();

        match res
        {
            Ok(token) => self.current_token = token,
            Err(_e) => panic!("Error parsing the string (probably an invalid character)"),
        }
    }

    //factor : INTEGER | LPAREN expr RPAREN
    fn factor(&mut self) -> i64
    {
        match self.current_token
        {
            Token::Integer(integer) => {self.eat(); return integer},
            Token::Lparen => {self.eat(); let result = self.expr(); self.eat(); return result}
            _ => panic!("Error in the factor rule")
        }
    }

    //term : factor ((MUL | DIV) factor)*
    fn term(&mut self) -> i64
    {
        let mut result = self.factor();

        while self.current_token == Token::Mul || self.current_token == Token::Div{
            match self.current_token
            {
                Token::Mul => {self.eat(); result *= self.factor();},
                Token::Div => {self.eat(); result /= self.factor();},
                _ => panic!("Error in term rule")
            }
        }

        return result;
    }

    //Arithmetic expression parser / interpreter.
    //
    //        calc> 7 + 3 * (10 / (12 / (3 + 1) - 1))
    //        22
    //
    //        expr   : term ((PLUS | MINUS) term)*
    //        term   : factor ((MUL | DIV) factor)*
    //        factor : INTEGER | LPAREN expr RPAREN
    fn expr(&mut self) -> i64
    {
        let mut result = self.term();

        while self.current_token == Token::Plus || self.current_token == Token::Minus{
            match self.current_token
            {
                Token::Plus => {self.eat(); result += self.term();},
                Token::Minus => {self.eat(); result -= self.term();},
                _ => panic!("Error in expr rule")
            }
        }

        return result;
    }

    fn create_interpreter(lexer: &mut Lexer) -> Interpreter
    {
        let res_token = lexer.get_next_token();
        let cur_token: Token;

        match res_token{
            Ok(token ) => cur_token = token,
            Err(_e) => panic!("Error in creating the interpreter"),
        }

        Interpreter{
            lexer,
            current_token: cur_token,
        }
    }
}

fn main() {
    loop {
        println!("Please insert the formula you want to calculate. Press CTRL-c to exit");
        let mut formula = String::new();
        io::stdin().read_line(&mut formula).expect("Failed to read line");

        let mut lexer = Lexer::create_lexer(formula);

        let mut interpreter = Interpreter::create_interpreter(&mut lexer);

        let result = interpreter.expr();

        println!("{}", result);
    }
    //println!("Hello, world!");
}
