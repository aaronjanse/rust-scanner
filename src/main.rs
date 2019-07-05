fn main() {
    let text = "Hello, world!";
    parse(text);
}

fn parse(text: &str) {
  let mut iter = TextIter::init(text);
  loop {
    match iter.next() {
      Some(c) => print!("{}", c),
      None => break,
    }
  }
}

struct TextIter<'a> {
  iter: core::iter::Peekable<core::str::Chars<'a>>,
  next: Option<char>,
}

impl<'a> TextIter<'a> {
  fn init(s: &'a str) -> TextIter<'a> {
    let mut iter = s.chars().peekable();
    let next = iter.next();
    TextIter {
      iter, next,
    }
  }
  fn next (&mut self) -> Option<char> {
    let cur = self.next;
    self.next = self.iter.next();
    cur
  }

  fn peek(&self) -> Option<char> {
    self.next
  }

  fn peek2(&mut self) -> std::option::Option<char> {
    match self.iter.peek() {
      Some(i) => Some(*i),
      None => None,
    }
  }
}

// struct Token {
//   kind: TokenType,
//   line: u32,
// }

// enum TokenType {
//   // Single-character tokens.
//   LeftParen, RightParen,
//   LeftBrace, RightBrace,
//   Colon, Pipe, Backslash,
//   Plus, Minus, Star, Slash,
//   Dollar,

//   // One or two character tokens.
//   Equal, NotEqual,
//   And, Or,
//   Greater, GreaterEq,
//   Less, LessEq,
//   Arrow, // ->
//   PipeForwards,  // |>

//   // Literals.
//   Identifier, Str, Number,

//   // Keywords.
//   True, False,
//   If, Then, Else,
//   Let, In,
//   Yield,

//   Eof,
// }