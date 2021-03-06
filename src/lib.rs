extern crate svg;
#[macro_use]
extern crate nom;

mod parsers;
use parsers::parse_input;

#[derive(Debug)]
struct Movement {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Robot {
    position: Position,
}

pub fn puzzle(input: &str) -> u32 {
    // parse the robots and movements
    let (mut robots, movements) = parse_input(input.as_bytes());

    // assert the number of movements is a multiple of the number of robots
    assert!((movements.len() / robots.len()) * robots.len() == movements.len());

    // apply the movements to the robots
    let mut collisions = vec![];
    for round_moves in movements.chunks(robots.len()) {
        // update positions
        for (r, m) in robots.iter_mut().zip(round_moves) {
            r.position.x += m.x;
            r.position.y += m.y;
        }

        // check for collisions
        for (i, r1) in robots.iter().enumerate() {
            for r2 in robots[(i + 1)..].iter() {
                if r1.position.x == r2.position.x && r1.position.y == r2.position.y {
                    collisions.push(r1.position.clone());
                }
            }
        }
    }

    // create a SVG with dots on all collision positions
    let mut document = svg::Document::new();
    for col in &collisions {
        let circle = svg::node::element::Circle::new()
            .set("cx", col.x)
            .set("cy", col.y)
            .set("r", 0.5)
            .set("fill", "black")
            .set("stroke", "none");
        document = document.add(circle);
    }
    svg::save("image.svg", &document).unwrap();

    collisions.len() as u32
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        // [0,0][1,1](1,0)(0,-1)(0,1)(-1,0)(-1,0)(0,1)(0,-1)(1,0)
        //
        // Robot 1 begint op 0,0 en Robot 2 begint op 1,1
        // Robot 1 gaat naar 1,0 (0,0 + 1,0)
        // Robot 2 gaat naar 1,0 (1,1 + 0,-1)
        // OEPS! Dit is dus een knelpunt.
        // Robot 1 gaat naar 1,1 (1,0 + 0,1)
        // Robot 2 gaat naar 0,0 (1,0 + -1,0)
        // Robot 1 gaat naar 0,1 (1,1 + -1,0)
        // Robot 2 gaat naar 0,1 (0,0 + 0,1)
        // AI, Dit is ook een knelpunt.
        // Robot 1 gaat naar 0,0 (0,1 + 0,-1)
        // Robot 2 gaat naar 1,1 (0,1 + 1,0)
        //
        // Het komt in dit voorbeeld dus 2 keer voor dat de robots elkaar tegen komen.
        let input = "[0,0][1,1](1,0)(0,-1)(0,1)(-1,0)(-1,0)(0,1)(0,-1)(1,0)";
        assert_eq!(puzzle(input), 2);
    }
}
