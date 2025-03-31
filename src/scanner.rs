use std::{
    fs::File,
    io::{self, BufRead},
};

// importamos el modulo de token que contiene la definicion de los tokens y sus tipos
use crate::token::{Token, TokenType};

// El escaner es el encargado de contener toda la informacion que permita
// el proceso de identificacion de tokens
// tenemos una lista de tokens, que son los tokens identificados
// y una linea y columna que nos permite saber en que parte del archivo estamos
// el escaner es el encargado de escanearlo pero el archivo se lee en la funcion run() en main.rs
pub struct Scanner {
    // vector que contiene objetos de tipo Token, que son los tokens identificados, junto a su lexema (valor crudo o source code, no se como explicarlo), junto a su posicion (linea y columna)
    tokens: Vec<Token>,
    line: usize,
    column: usize,
}

impl Scanner {
    pub fn new() -> Self {
        Scanner {
            tokens: Vec::new(),
            line: 1,
            column: 1,
        }
    }

    // funcion para escanear un archivo
    // toma como entrada un std::fs::File
    pub fn scan_file(&mut self, file: File) -> io::Result<()> {
        // creamos un buffer para leer linea por linea
        let reader = io::BufReader::new(file);
        // hacemos el loop para cada linea
        for line in reader.lines() {
            let raw_line = line?;
            // scaneamos la linea
            self.scan_line(&raw_line);
            // pasamos a la siguiente linea
            self.line += 1;
            // dentro del scan_line se hace el loop por columna
            self.column = 1;
        }
        Ok(())
    }

    // funcion para escanear una linea
    // toma como entrada una &str
    // y escanea cada caracter
    // y lo categoriza como un token y lo agrega a la lista de tokens identificados
    // si no es un token valido, lo ignora
    // y pasa al siguiente caracter de la linea, y si no hay mas caracteres, termina el ciclo (para esta linea)
    fn scan_line(&mut self, line: &str) {
        // se crea una pila iterable con los caracteres de la linea
        let mut chars = line.chars().peekable();
        while let Some(&char) = chars.peek() {
            // ahora caterorizamos
            match char {
                // no hacemos nada con espacios en blancos, pasamos a la siguiente columna
                ' ' | '\t' => {
                    self.column += 1;
                    chars.next();
                }

                'a'..='z' | 'A'..'Z' | '_' => {
                    // dicho que se me ocurrio mientras programaba esto: todo es un identificador hasta que se demuestre lo contrario

                    // si es una letra, comenzamos a leer el lexema
                    let start_column = self.column;
                    let mut lexeme = String::new();
                    // digit = [0-9]
                    // letter = [a-zA-Z]
                    // En el texto aparece ID = [letter | _]+[digit|letter|]*
                    // Pero supondre que es un error gramatico y que falto un _ despues de letter en la definicion que tiene [...]*
                    // tal que ID = [letter | _]+[digit|letter|_]*
                    while let Some(&char) = chars.peek() {
                        if char.is_alphanumeric() || char == '_' {
                            lexeme.push(char);
                            self.column += 1;
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    // verificamos si es una palabra reservada y si no entonces es un identificador por defecto
                    let token_type = TokenType::from_string(lexeme.clone());
                    self.tokens
                        .push(Token::new(token_type, lexeme, self.line, start_column));
                }
                '0'..='9' => {
                    // si es un numero, comenzamos a leer el lexema
                    let start_column = self.column;
                    let mut lexeme = String::new();
                    // digit = [0-9]
                    // INT_NUM = [digit]+
                    while let Some(&char) = chars.peek() {
                        if char.is_digit(10) {
                            lexeme.push(char);
                            self.column += 1;
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    self.tokens.push(Token::new(
                        TokenType::INT_NUM,
                        lexeme,
                        self.line,
                        start_column,
                    ));
                }
                // Extra por parte de nosotros, para poder identificar comentarios, encontramos esto relevante aunque no este en los requisitos
                '/' => {
                    // si es un simbolo de comentario, comenzamos a leer el comentario
                    let start_column = self.column;
                    let mut lexeme = String::new();
                    lexeme.push(char);
                    self.column += 1;
                    chars.next();

                    // verificamos si es un comentario de una sola linea o de varias lineas
                    if let Some(&next_char) = chars.peek() {
                        if next_char == '/' {
                            // ignoramos
                            break;
                        } else {
                            // no es un comentario, lo agregamos como un simbolo
                            let token_type = TokenType::from_string(lexeme.clone());
                            self.tokens
                                .push(Token::new(token_type, lexeme, self.line, start_column));
                        }
                    }
                },
                // manejamos exactamente estos simbolos y no otros, para poder manejar dobles simbolos como lo son &&, ||, ==, !=, <=, >=, <<, >>, etc
                '&' | '|' | '=' | '!' | '<' | '>' => {
                    let start_column = self.column;
                    let mut lexeme = String::new();
                    lexeme.push(char);
                    self.column += 1;
                    chars.next();

                    // verificamos si necesitamos combinarlo con otro simbolo
                    if let Some(&next_char) = chars.peek() {
                        // esto es lo que hace, es tomar el siguiente caracter y lo combina con el actual
                        let combined = format!("{}{}", char, next_char);
                        // y verificamos si la combinacion es valida
                        match combined.as_str() {
                            "&&" | "||" | "==" | "!=" | "<=" | ">=" | "<<" | ">>" => {
                                lexeme.push(next_char);
                                self.column += 1;
                                chars.next();
                            }
                            // si no, no hacemos nada, y seria solamente un simbolo en el lexema
                            _ => {}
                        }
                    }

                    let token_type = TokenType::from_string(lexeme.clone());
                    self.tokens
                        .push(Token::new(token_type, lexeme, self.line, start_column));
                }
                _ => {
                    // si es otra cosa, lo identificamos (no aplica para tokens compuestos (dobles o mas caracteres))
                    let token_type = TokenType::from_string(char.to_string());
                    self.tokens.push(Token::new(
                        token_type,
                        char.to_string(),
                        self.line,
                        self.column,
                    ));
                    chars.next();
                }
            }
        }
    }

    // funcion para obtener los tokens, para poder imprimirlos
    pub fn get_tokens(&self) -> &Vec<Token> {
        &self.tokens
    }
}
