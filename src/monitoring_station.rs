use std::{
    f32, fmt,
    fs::File,
    io,
    io::{BufReader, Read},
};

#[derive(Debug, Eq, PartialEq)]
pub struct AsteroidMap {
    width: u32,
    height: u32,
    asteroids: Vec<(u32, u32)>,
}

impl AsteroidMap {
    pub fn new(map_data: &str) -> AsteroidMap {
        let lines: Vec<&str> = map_data.split('\n').collect();
        let mut asteroids = Vec::new();
        let height = lines.len() as u32;
        let width = lines[0].len() as u32;
        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch == '#' {
                    asteroids.push((x as u32, y as u32))
                }
            }
        }
        AsteroidMap {
            width,
            height,
            asteroids,
        }
    }

    pub fn find_best_location(&self) -> ((u32, u32), u32) {
        let mut best_count = 0;
        let mut best_point = (0, 0);
        for potential_base in &self.asteroids {
            let mut lines_of_sight = Vec::new();
            for asteroid in &self.asteroids {
                if asteroid != potential_base {
                    lines_of_sight.push(Line::new(*potential_base, *asteroid))
                }
            }
            let mut clear_lines_of_sight = 0;
            for line in &lines_of_sight {
                let mut asteroid_count = Vec::new();
                for asteroid in &self.asteroids {
                    if asteroid != potential_base {
                        if point_on_line(&line, *asteroid) {
                            asteroid_count.push(*asteroid);
                        } else {
                        }
                    }
                }
                if asteroid_count.len() == 1 {
                    clear_lines_of_sight += 1
                }
            }
            if clear_lines_of_sight > best_count {
                best_count = clear_lines_of_sight;
                best_point = *potential_base;
            }
        }
        (best_point, best_count as u32)
    }

    pub fn shoot_asteroids(&self, base_location: (u32, u32)) -> Vec<(u32, u32)> {
        let mut target_asteroids: Vec<(u32, u32)> = self
            .asteroids
            .iter()
            .copied()
            .filter(|x| x != &base_location)
            .collect();
        let mut shot_asteroids: Vec<(u32, u32)> = Vec::new();

        while !(&target_asteroids.is_empty()) {
            let mut asteroid_angles = Vec::new();
            for asteroid in &target_asteroids {
                let mut angle = calculate_angle(base_location, *asteroid);
                let quadrant = quadrant(base_location, *asteroid);
                let distance = distance(base_location, *asteroid);
                match quadrant {
                    Quadrant::NE => angle += 90.0,
                    Quadrant::SE => angle = angle.abs() + 90.0,
                    Quadrant::SW => angle = (90.0 - angle.abs()) + 180.0,
                    Quadrant::NW => angle = angle.abs() + 270.0,
                }

                asteroid_angles.push((angle, *asteroid, distance));
                asteroid_angles.sort_by(|a, b| {
                    a.0.partial_cmp(&b.0)
                        .unwrap_or_else(|| panic!("cannot sort {} & {}", a.0, b.0))
                });
            }
            let mut iter = asteroid_angles.iter_mut().peekable();
            let mut multiple_asteroids = Vec::new();
            while let Some(current) = iter.next() {
                multiple_asteroids.push(*current);
                if let Some(next) = iter.peek() {
                    if approx_equal(current.0, next.0) {
                        continue;
                    }
                }
                multiple_asteroids.sort_by(|a, b| {
                    a.2.partial_cmp(&b.2)
                        .unwrap_or_else(|| panic!("cannot sort {} & {}", a.2, b.2))
                });
                let closest_asteroid = multiple_asteroids[0].1;
                shot_asteroids.push(closest_asteroid);
                target_asteroids.retain(|x| x != &closest_asteroid);
                multiple_asteroids.clear();
            }
        }
        shot_asteroids
    }
}

#[derive(Debug)]
struct Line {
    start: (f32, f32),
    end: (f32, f32),
}

impl Line {
    fn new(start: (u32, u32), end: (u32, u32)) -> Line {
        let s = (start.0 as f32, start.1 as f32);
        let e = (end.0 as f32, end.1 as f32);
        Line { start: s, end: e }
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}, {}), ({}, {})",
            self.start.0, self.start.1, self.end.0, self.end.1
        )?;
        Ok(())
    }
}

fn distance(start: (u32, u32), end: (u32, u32)) -> f32 {
    let a = (end.0 as f32 - start.0 as f32).powi(2);
    let b = (end.1 as f32 - start.1 as f32).powi(2);
    (a + b).sqrt()
}

fn calculate_slope(start: (f32, f32), end: (f32, f32)) -> f32 {
    ((end.1 - start.1) / (end.0 - start.0))
}

fn calculate_y_intercept(point: (f32, f32), slope: f32) -> f32 {
    //y = mx + b
    //6 = 0.5*8 + b
    //6 - 4 = b
    point.1 - (slope * point.0)
}

fn calculate_angle(start: (u32, u32), end: (u32, u32)) -> f32 {
    let s = (start.0 as f32, start.1 as f32);
    let e = (end.0 as f32, end.1 as f32);
    calculate_slope(s, e).atan().to_degrees()
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Quadrant {
    NE,
    SE,
    SW,
    NW,
}

fn quadrant(start: (u32, u32), end: (u32, u32)) -> Quadrant {
    if (end.0 >= start.0) && (end.1 <= start.1) {
        Quadrant::NE
    } else if (end.0 >= start.0) && (end.1 >= start.1) {
        Quadrant::SE
    } else if (end.0 <= start.0) && (end.1 >= start.1) {
        Quadrant::SW
    } else if (end.0 <= start.0) && (end.1 <= start.1) {
        Quadrant::NW
    } else {
        panic!("Unkown quadrant logic reached for {:?} {:?}", start, end)
    }
}

const ERROR: f32 = 0.01;

fn approx_equal(a: f32, b: f32) -> bool {
    (a - b).abs() < ERROR
}

fn point_on_line(line: &Line, point: (u32, u32)) -> bool {
    if line_is_flat(&line) {
        within_end_points(&line, point)
    } else if within_end_points(&line, point) {
        let p = (point.0 as f32, point.1 as f32);
        if (p == line.start) | (p == line.end) {
            true
        } else {
            let slope = calculate_slope(line.start, line.end);
            let b = calculate_y_intercept(line.start, slope);
            let y = (slope * p.0) + b;
            approx_equal(p.1, y)
        }
    } else {
        false
    }
}

fn line_is_flat(line: &Line) -> bool {
    approx_equal(line.start.0, line.end.0) | approx_equal(line.start.1, line.end.1)
}

fn within_end_points(line: &Line, point: (u32, u32)) -> bool {
    within_x(&line, point) & within_y(&line, point)
}

fn within_x(line: &Line, point: (u32, u32)) -> bool {
    ((point.0 as f32 >= line.start.0) & ((point.0 as f32) <= line.end.0))
        | ((point.0 as f32 >= line.end.0) & ((point.0 as f32) <= line.start.0))
}

fn within_y(line: &Line, point: (u32, u32)) -> bool {
    (point.1 as f32 >= line.start.1) & ((point.1 as f32) <= line.end.1)
        | (point.1 as f32 >= line.end.1) & ((point.1 as f32) <= line.start.1)
}

pub fn load_asteroid_input(file_name: &str) -> io::Result<String> {
    let asteroid_input = File::open(file_name)?;
    let mut reader = BufReader::new(asteroid_input);

    let mut asteroid_map = String::new();
    reader.read_to_string(&mut asteroid_map)?;

    Ok(asteroid_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_angle() {
        assert_eq!(calculate_angle((8, 3), (8, 1)), -90.0);
        assert_eq!(calculate_angle((8, 3), (9, 2)), -45.0);
        assert_eq!(calculate_angle((8, 3), (7, 2)), 45.0);
        assert_eq!(calculate_angle((8, 3), (7, 4)), -45.0);
        assert_eq!(calculate_angle((8, 3), (2, 3)), 0.0);
        assert_eq!(calculate_angle((8, 3), (10, 3)), 0.0);
        assert_eq!(calculate_angle((8, 3), (8, 4)), 90.0);
        assert_eq!(calculate_angle((8, 3), (8, 2)), -90.0);
    }

    #[test]
    fn test_distance() {
        assert_eq!(distance((4, 3), (3, 2)), 1.4142135)
    }

    #[test]
    fn test_line_is_flat() {
        let line = Line::new((0, 3), (0, 8));
        assert!(line_is_flat(&line))
    }

    #[test]
    fn test_within_end_points() {
        let line = Line::new((1, 1), (4, 4));
        assert_eq!(within_end_points(&line, (2, 2)), true);
        assert_eq!(within_end_points(&line, (2, 3)), true);
        assert_eq!(within_end_points(&line, (0, 3)), false);
        assert_eq!(within_end_points(&line, (0, 6)), false);
        assert_eq!(within_end_points(&line, (7, 6)), false);
        assert_eq!(within_end_points(&line, (4, 4)), true);
        let line = Line::new((2, 3), (8, 6));
        assert_eq!(within_end_points(&line, (2, 3)), true);
        let line = Line::new((3, 4), (0, 2));
        assert_eq!(within_end_points(&line, (0, 2)), true);
        let line = Line::new((4, 0), (4, 3));
        assert_eq!(within_end_points(&line, (4, 2)), true);
        let line = Line::new((5, 8), (8, 6));
        assert_eq!(within_end_points(&line, (8, 6)), true);
    }

    #[test]
    fn test_point_on_line() {
        let line = Line::new((2, 3), (8, 6));
        assert_eq!(point_on_line(&line, (2, 3)), true);
        assert_eq!(point_on_line(&line, (0, 0)), false);
        assert_eq!(point_on_line(&line, (8, 1)), false);
        assert_eq!(point_on_line(&line, (5, 6)), false);
        let line = Line::new((5, 8), (6, 9));
        assert_eq!(point_on_line(&line, (6, 9)), true);
        assert_eq!(point_on_line(&line, (1, 4)), false);
        let line = Line::new((3, 4), (0, 2));
        assert_eq!(point_on_line(&line, (0, 2)), true);
        let line = Line::new((5, 8), (8, 6));
        assert_eq!(point_on_line(&line, (8, 6)), true);
    }

    #[test]
    fn test_create_astroid_map() {
        let map = ".#..#\n.....\n#####\n....#\n...##";
        let correct_asteroid_map = AsteroidMap {
            width: 5,
            height: 5,
            asteroids: vec![
                (1, 0),
                (4, 0),
                (0, 2),
                (1, 2),
                (2, 2),
                (3, 2),
                (4, 2),
                (4, 3),
                (3, 4),
                (4, 4),
            ],
        };
        let asteroid_map = AsteroidMap::new(map);
        assert_eq!(asteroid_map, correct_asteroid_map);
        assert_eq!(asteroid_map.asteroids, correct_asteroid_map.asteroids)
    }

    #[test]
    fn test_find_best_location() {
        let map = ".#..#\n.....\n#####\n....#\n...##";
        let asteroid_map = AsteroidMap::new(map);
        assert_eq!(asteroid_map.find_best_location(), ((3, 4), 8));

        let map = ".#..#..###\n####.###.#\n....###.#.\n..###.##.#\n##.##.#.#.\n....###..#\n..#.#..#.#\n#..#.#.###\n.##...##.#\n.....#.#..";
        let asteroid_map = AsteroidMap::new(map);
        assert_eq!(asteroid_map.find_best_location(), ((6, 3), 41));

        let map = "#.#...#.#.\n.###....#.\n.#....#...\n##.#.#.#.#\n....#.#.#.\n.##..###.#\n..#...##..\n..##....##\n......#...\n.####.###.";
        let asteroid_map = AsteroidMap::new(map);
        assert_eq!(asteroid_map.find_best_location(), ((1, 2), 35));

        let map = "......#.#.\n#..#.#....\n..#######.\n.#.#.###..\n.#..#.....\n..#....#.#\n#..#....#.\n.##.#..###\n##...#..#.\n.#....####";
        let asteroid_map = AsteroidMap::new(map);
        assert_eq!(asteroid_map.find_best_location(), ((5, 8), 33));

        let map = ".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##";
        let asteroid_map = AsteroidMap::new(map);
        assert_eq!(asteroid_map.find_best_location(), ((11, 13), 210))
    }

    #[test]
    fn test_shoot_asteroids() {
        let map = ".#....#####...#..\n##...##.#####..##\n##...#...#.#####.\n..#.....X...###..\n..#.#.....#....##";
        let asteroid_map = AsteroidMap::new(map);
        let asteroids = asteroid_map.shoot_asteroids((8, 3));
        assert_eq!(asteroids[17], (4, 4));

        let map = ".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##";
        let asteroid_map = AsteroidMap::new(map);
        let base = asteroid_map.find_best_location();
        let asteroids = asteroid_map.shoot_asteroids(base.0);
        assert_eq!(asteroids[199], (8, 2))
    }
}
