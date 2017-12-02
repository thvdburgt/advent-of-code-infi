extern crate regex;
extern crate svg;

use regex::Regex;

use svg::node::element::Path;
use svg::node::element::path::Data;
use svg::node::element::Circle;

struct Robot {
    x: i32,
    y: i32,
    data: Data
}
#[derive(Debug)]
struct Movement {
    x: i32,
    y: i32
}

pub fn puzzle1(input: &str) -> u32 {
    // create the regexes used
    let robot_re = Regex::new(r"\[([-+]?\d+),([-+]?\d+)\]").unwrap();
    let movement_re = Regex::new(r"\(([-+]?\d+),([-+]?\d+)\)").unwrap();

    // parse the movements
    let movements: Vec<_> = movement_re.captures_iter(input).map(|cap| {
        Movement {
            x: cap[1].parse::<i32>().unwrap(),
            y: cap[2].parse::<i32>().unwrap()
        }
    }).collect();
    // parse the robots
    let mut robots: Vec<_> = robot_re.captures_iter(input).map(|cap| {
        let x = cap[1].parse::<i32>().unwrap();
        let y = cap[2].parse::<i32>().unwrap();
        Robot {x, y, data: Data::new().move_to((x, y))
        }
    }).collect();

    assert!((movements.len() / robots.len()) * robots.len() == movements.len());

    
    // count the collisions
    let mut collisions = vec![];
    for round_moves in movements.chunks(robots.len()) {
        // update positions
        for (r, m) in robots.iter_mut().zip(round_moves) {
            r.x += m.x;
            r.y += m.y;
            r.data = r.data.clone().line_by((m.x, m.y));
        }

        // check for collisions
        for (i, r1) in robots.iter().enumerate() {
            for r2 in robots[(i + 1)..].iter() {
                if r1.x == r2.x && r1.y == r2.y {
                    collisions.push(Movement{x: r1.x, y: r1.y});
                }
            }
        }
    }

    // create svg
    let mut document = svg::Document::new()
        .set("viewBox", (0, 0, 100, 100));
    for col in &collisions {
        let circle = Circle::new()
            .set("cx", col.x)
            .set("cy", col.y)
            .set("r", 0.1)
            .set("fill", "black")
            .set("stroke", "none")
            .set("stroke-width", 0.003);
        document = document.add(circle);
    }
    // for mut r in robots {
    //     r.data = r.data.clone().close();
    //     let path = Path::new()
    //         .set("fill", "none")
    //         .set("stroke", "black")
    //         .set("stroke-width", 0.003)
    //         .set("d", r.data);
    //     document = document.add(path);
    // }
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
