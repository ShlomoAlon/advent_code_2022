use std::iter::Peekable;
use std::ops::Deref;
use std::str::Chars;
use std::vec::IntoIter;

pub struct Parser{
    chars: Peekable<IntoIter<char>>
}
impl Deref for Parser{
    type Target = Peekable<IntoIter<char>>;

    fn deref(&self) -> &Self::Target {
        &self.chars
    }
}

impl Parser {
    pub fn new(s: &str) -> Self{
        Parser{chars: s.to_string().chars().collect::<Vec<char>>().into_iter().peekable()}
    }

    pub fn parse_whitespace(& mut self) -> bool {
        if !self.chars.peek().is_none() || self.chars.peek().unwrap().is_whitespace(){
            false
        } else {
            while let Some(cur) = self.chars.peek() {
                if !cur.is_whitespace(){
                    break
                } else {
                    self.chars.next();
                }
            }
            true
        }
    }

    pub fn parse_int(& mut self) -> Option<i64>{
        let mut result = String::new();
        if !self.chars.peek()?.is_digit(10){
            return None
        }
        while self.chars.peek().is_some() && self.chars.peek().unwrap().is_digit(10) {
            result.push(self.chars.next().unwrap());
        }
        result.parse().ok()
    }
    pub fn parse_next_digit(& mut self) -> Option<i64>{
        while !self.chars.peek()?.is_digit(10) {
            self.chars.next();
        }
        self.parse_int()
    }

}


// pub fn parse_specific_word(parser: Parser, word: &str) -> Option<bool>{
//     let mut word = word.chars().peekable();
//     if word.peek() != parser.peek(){
//         Some(false)
//     } else {
//         for letter in word{
//             if parser.next()? != letter{
//                 return None
//             }
//         }
//         Some(true)
//     }
// }

// pub fn parse_word(parser: Parser) -> String{
//     let mut result = String::new();
//     while parser.peek().is_some() && !parser.peek().unwrap().is_whitespace() {
//         result.push(parser.next().unwrap())
//     }
//     result
// }
//






