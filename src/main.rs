use std::str::Chars;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;


#[derive(Serialize, Deserialize)]
struct Data {
    id: usize,
    key: Box<str>,
    title: Box<str>,
    latest: usize,
    timestamp: Box<str>,
    source: String
}

struct Link {
    name: String,
    link: String
}

struct Header {
    name: String,
    depth: usize
}

struct Comment {
    name: String
}

enum Section {
    Text(String),
    Header(Header),
    Link(Link),
    Table,
    Graphic,
    Comment(Comment)
}

struct Lexer<'a> {
    //pub raw_data: String,
    pub cursor: usize,
    pub len: usize,
    pub table: Vec<Section>,
    pub chars: Chars<'a>,
}
// raw_data: raw_data.clone(),
impl Lexer<'_> {
    pub fn new(raw_data: String) -> Self {
        Self {cursor: 0, len: raw_data.len(), table: vec![], chars: Chars::from(raw_data.chars())}
    }

    fn lex(&mut self) -> &Vec<Section> {
        //self.len = self.chars.len();
        let mut current_text = String::new();
        let mut caught: bool = false;
        let mut block: Option<Section> = None;

        for cursor in self.cursor..self.len {
            if self.chars.nth(cursor).unwrap() == '=' && self.chars.nth(cursor + 1).unwrap() == '=' {
                caught = true;
                block = Some(self.header_block());
            }

            current_text += self.chars.nth(cursor).unwrap().to_string().as_str();

            if caught {
                caught = false;
                current_text = "".to_string();
                self.table.push(Section::Text( current_text.clone() ));

                if Some(&block.unwrap()).is_some() {
                    self.table.push(block.unwrap());
                }
            }
        }

        return &self.table;
    }

    fn header_block(&mut self) -> Section {
        let mut title= String::new();
        let mut b: usize = self.cursor + 2;
        let mut depth: usize = 0;

        while self.chars.nth(b).unwrap() == '=' {
            depth+=1;
            b+=1;
        }

        b+=1;

        for b in b..self.len {
            title += self.chars.nth(b).unwrap().to_string().as_str();
            if self.chars.nth(b).unwrap() == ' ' && self.chars.nth(b + 1).unwrap() == '=' {
                break;
            }
        }

        self.cursor = b + depth + 2;

        return Section::Header(Header{name: title, depth });
    }
}

fn main() {
    let mut file = File::open("sample.json").unwrap();
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();
    let data: Data = serde_json::from_str(&buff).unwrap();

    let mut lexer = Lexer::new(data.source);
    let _output = lexer.lex();
}
