use std::str;
use nom::digit;

use super::Robot;
use super::Position;
use super::Movement;

// parsers to parse the input
named!(sign <i32>, alt!(tag!("-") => { |_| -1 } | tag!("+") => { |_| 1 }));
named!(numeric_string <&str>, map_res!(digit, str::from_utf8));
named!(unsigned_nr<i32>, map_res!(numeric_string, str::FromStr::from_str));
named!(signed_nr<i32>, do_parse!(s: opt!(sign) >> n: unsigned_nr >> (s.unwrap_or(1) * n) ));
named!(robot<Robot>,
       do_parse!(
           char!('[') >>
           x: signed_nr >>
           char!(',') >>
           y: signed_nr >>
           char!(']') >>
           ( Robot { position: Position {x, y} } )
           )
      );
named!(movement<Movement>,
       do_parse!(
           char!('(') >>
           x: signed_nr >>
           char!(',') >>
           y: signed_nr >>
           char!(')') >>
           ( Movement {x, y} )
           )
      );
named!(robots< Vec<Robot> >, many1!( robot ) );
named!(movements< Vec<Movement> >, many1!( movement ) );
named!(robots_and_movements< (Vec<Robot>, Vec<Movement>) >, tuple!(robots, movements) );

pub(crate) fn parse_input(input: &[u8]) -> (Vec<Robot>, Vec<Movement>) {
    let (_, (rs, ms)) = robots_and_movements(input).unwrap();
    (rs, ms)
}
