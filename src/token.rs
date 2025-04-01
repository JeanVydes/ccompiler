// los tipos de tokens que nos piden en la especificacion del taller
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // palabras reservas (keywords)
    INT,
    MAIN,
    VOID,
    BREAK,
    DO,
    ELSE,
    IF,
    WHILE,
    RETURN,
    READ,
    WRITE,
    // simbolos especiales
    LBRACE,
    RBRACE,
    LSQUARE,
    RQUARE,
    LPAR,
    RPAR,
    SEMI,
    PLUS,
    MINUS,
    MUL_OP,
    DIV_OP,
    AND_OP,
    OR_OP,
    NOT_OP,
    ASSIGN,
    LT,
    GT,
    SHL_OP,
    SHR_OP,
    EQ,
    NOTEQ,
    LTEQ,
    GTEQ,
    COMMA,

    // numeros e identificadores
    INT_NUM,
    ID,

    // directivas de pre procesamiento
    PP_DEFINE,
    PP_ELIF,
    PP_ELSE,
    PP_ENDIF,
    PP_IF,
    PP_INCLUDE,
    PP_UNDEF,
    PP_MESSAGE,
    PP_IFDEF,
    PP_IFNDEF,

    // tokens
    ERR,
}

// implementamos algunos metodos
impl TokenType {
    // esta funcion identifica el token
    pub fn from_string(lexeme: String) -> TokenType {
        // hacemos .to_lowercase() para identificar tanto tokens en mayusculas o minisculas
        match lexeme.to_lowercase().as_str() {
            "int" => TokenType::INT,
            "main" => TokenType::MAIN,
            "void" => TokenType::VOID,
            "break" => TokenType::BREAK,
            "do" => TokenType::DO,
            "else" => TokenType::ELSE,
            "if" => TokenType::IF,
            "while" => TokenType::WHILE,
            "return" => TokenType::RETURN,
            "read" => TokenType::READ,
            "write" => TokenType::WRITE,

            "#define" => TokenType::PP_DEFINE,
            "#elif" => TokenType::PP_ELIF,
            "#else" => TokenType::PP_ELSE,
            "#endif" => TokenType::PP_ENDIF,
            "#ifdef" => TokenType::PP_IFDEF,
            "#ifndef" => TokenType::PP_IFNDEF,
            "#include" => TokenType::PP_INCLUDE,
            "#undef" => TokenType::PP_UNDEF,
            "#message" => TokenType::PP_MESSAGE,
            "#if" => TokenType::PP_IF,

            "{" => TokenType::LBRACE,
            "}" => TokenType::RBRACE,
            "[" => TokenType::LSQUARE,
            "]" => TokenType::RQUARE,
            "(" => TokenType::LPAR,
            ")" => TokenType::RPAR,
            ";" => TokenType::SEMI,
            "+" => TokenType::PLUS,
            "-" => TokenType::MINUS,
            "*" => TokenType::MUL_OP,
            "/" => TokenType::DIV_OP,

            "&&" => TokenType::AND_OP,
            "||" => TokenType::OR_OP,
            "!" => TokenType::NOT_OP,
            "=" => TokenType::ASSIGN,
            "<" => TokenType::LT,
            ">" => TokenType::GT,
            "<<" => TokenType::SHL_OP,
            ">>" => TokenType::SHR_OP,
            "==" => TokenType::EQ,
            "!=" => TokenType::NOTEQ,
            "<=" => TokenType::LTEQ,
            ">=" => TokenType::GTEQ,

            "," => TokenType::COMMA,

            _ => {
                if lexeme.chars().all(|c| c.is_digit(10)) {
                    TokenType::INT_NUM
                } else if lexeme.chars().all(|c| c.is_alphanumeric() || c == '_') {
                    TokenType::ID
                } else {
                    TokenType::ERR
                }
            }
        }
    }
}

// un Token se utiliza para identificar tokens, conteniendo informacion importante para utilizarla despues
#[derive(Debug, Clone)]
pub struct Token {
    // el tipo de tokens
    pub r#type: TokenType,
    // el source code/valor en bruto
    pub lexeme: String,
    // la linea donde se encuentra el token
    pub line: usize,
    // la columna donde se encuentra el token
    pub column: usize,
}

// implementamos algunos metodos, el constructor y el to_string para utilizarlo para debugging
impl Token {
    // esta funcion se utiliza para crear un objeto de tipo Token
    pub fn new(token_type: TokenType, lexeme: String, line: usize, column: usize) -> Token {
        Token {
            r#type: token_type,
            lexeme,
            line,
            column,
        }
    }

    // esta funcion permita mostrar en forma de string informacion acerca del token, la utilizamos unicamente para hacer el print, luego de haber escaneado el archivo e identificado tokens
    pub fn to_string(&self) -> String {
        format!("{:?}", self.r#type)
    }
}
