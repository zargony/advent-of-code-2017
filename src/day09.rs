#[macro_use]
extern crate nom;


/// Tokenized content of a stream
#[derive(Debug, PartialEq)]
enum Token<'a> {
    GroupStart,
    GroupEnd,
    Garbage(Vec<&'a str>),
    Data(&'a str),
}

impl<'a> Token<'a> {
    /// Returns the garbage size (without cancelled characters)
    fn garbage_size(&self) -> usize {
        match *self {
            Token::Garbage(ref v) => v.iter().map(|s| s.len()).sum(),
            _ => 0,
        }
    }
}

// The stream of characters
#[derive(Debug, Clone)]
struct Stream<'a> {
    input: &'a str,
}

impl<'a> Iterator for Stream<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        named!(garbage<&str, Vec<&str>>,
            delimited!(
                tag!("<"),
                many1!(
                    terminated!(
                        take_while!(|ch| ch!='!' && ch!='>'),
                        opt!(preceded!(tag!("!"), take!(1)))
                    )
                ),
                tag!(">")
            )
        );
        named!(token<&str, Token>, alt!(
            tag!("{") => { |_| Token::GroupStart } |
            tag!("}") => { |_| Token::GroupEnd } |
            garbage => { |s| Token::Garbage(s) } |
            take_until_either!("{}<") => { |s| Token::Data(s) }
        ));
        match token(self.input) {
            nom::IResult::Done(rest, token) => {
                self.input = rest;
                Some(token)
            },
            nom::IResult::Incomplete(_) => None,
            nom::IResult::Error(e) => panic!("Parser error: {:?}", e),
        }
    }
}

impl<'a> Stream<'a> {
    /// Create a new stream to tokenize using the given input
    fn new(input: &'a str) -> Stream<'a> {
        Stream { input: input }
    }

    /// Consumes the stream and returns the number of groups
    fn groups(self) -> usize {
        self.filter(|t| *t == Token::GroupEnd).count()
    }

    /// Consumes the stream and returns the score of the stream
    fn score(self) -> usize {
        self.fold((0, 0), |(score, depth), token| {
            match token {
                Token::GroupStart => (score, depth + 1),
                Token::GroupEnd => (score + depth, depth - 1),
                _ => (score, depth),
            }
        }).0
    }

    /// Consumes the stream and returns total size of garbage
    fn garbage_size(self) -> usize {
        self.map(|t| t.garbage_size()).sum()
    }
}


fn main() {
    let stream = Stream::new(include_str!("day09.txt"));
    println!("Total stream score of {} groups: {}", stream.clone().groups(), stream.clone().score());
    println!("Total size of garbage: {}", stream.garbage_size());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let mut stream = Stream::new("{{hello}<a}b<c{d!>e>}");
        assert_eq!(stream.next(), Some(Token::GroupStart));
        assert_eq!(stream.next(), Some(Token::GroupStart));
        assert_eq!(stream.next(), Some(Token::Data("hello")));
        assert_eq!(stream.next(), Some(Token::GroupEnd));
        assert_eq!(stream.next(), Some(Token::Garbage(vec!["a}b<c{d", "e"])));
        assert_eq!(stream.next(), Some(Token::GroupEnd));
        assert_eq!(stream.next(), None);
    }

    #[test]
    fn samples1() {
        assert_eq!(Stream::new("{}").groups(), 1);
        assert_eq!(Stream::new("{{{}}}").groups(), 3);
        assert_eq!(Stream::new("{{},{}}").groups(), 3);
        assert_eq!(Stream::new("{{{},{},{{}}}}").groups(), 6);
        assert_eq!(Stream::new("{<{},{},{{}}>}").groups(), 1);
        assert_eq!(Stream::new("{<a>,<a>,<a>,<a>}").groups(), 1);
        assert_eq!(Stream::new("{{<a>},{<a>},{<a>},{<a>}}").groups(), 5);
        assert_eq!(Stream::new("{{<!>},{<!>},{<!>},{<a>}}").groups(), 2);

        assert_eq!(Stream::new("{}").score(), 1);
        assert_eq!(Stream::new("{{{}}}").score(), 6);
        assert_eq!(Stream::new("{{},{}}").score(), 5);
        assert_eq!(Stream::new("{{{},{},{{}}}}").score(), 16);
        assert_eq!(Stream::new("{<a>,<a>,<a>,<a>}").score(), 1);
        assert_eq!(Stream::new("{{<ab>},{<ab>},{<ab>},{<ab>}}").score(), 9);
        assert_eq!(Stream::new("{{<!!>},{<!!>},{<!!>},{<!!>}}").score(), 9);
        assert_eq!(Stream::new("{{<a!>},{<a!>},{<a!>},{<ab>}}").score(), 3);
    }

    #[test]
    fn samples2() {
        assert_eq!(Stream::new("<>").garbage_size(), 0);
        assert_eq!(Stream::new("<random characters>").garbage_size(), 17);
        assert_eq!(Stream::new("<<<<>").garbage_size(), 3);
        assert_eq!(Stream::new("<{!>}>").garbage_size(), 2);
        assert_eq!(Stream::new("<!!>").garbage_size(), 0);
        assert_eq!(Stream::new("<!!!>>").garbage_size(), 0);
        assert_eq!(Stream::new("<{o\"i!a,<{i<a>").garbage_size(), 10);
    }
}
