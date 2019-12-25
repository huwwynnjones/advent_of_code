use std::{
    collections::HashMap,
    fmt,
    fs::File,
    io,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
pub struct Planet {
    name: String,
    orbiting_planets: HashMap<String, Planet>,
}

impl fmt::Display for Planet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> ", self.name)?;
        for planet in self.orbiting_planets.values() {
            write!(f, "{} ", planet.name())?
        }
        writeln!(f)?;
        for planet in self.orbiting_planets.values() {
            write!(f, "{}", planet)?
        }
        Ok(())
    }
}

impl Planet {
    fn new(name: &str) -> Planet {
        let name = name.to_string();
        let orbiting_planets = HashMap::new();
        Planet {
            name,
            orbiting_planets,
        }
    }

    fn add_planet_to_orbit_of(&mut self, planet_name: &str, orbit_name: &str) -> bool {
        let planet_key = planet_name.to_string();
        let mut inserted = false;
        if orbit_name == self.name() {
            inserted = self.add_planet(planet_name);
        } else if self.orbiting_planets.contains_key(&planet_key) {
            self.orbiting_planets
                .entry(planet_key)
                .and_modify(|p| inserted = p.add_planet(planet_name));
        } else {
            for p in self.orbiting_planets.values_mut() {
                if p.add_planet_to_orbit_of(planet_name, orbit_name) {
                    inserted = true;
                    break;
                }
            }
        }
        inserted
    }

    fn add_planet(&mut self, planet_name: &str) -> bool {
        let planet = Planet::new(planet_name);
        let planet_key = planet.name().to_string();
        self.orbiting_planets.insert(planet_key, planet);
        true
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn count_orbits_to(&self, name: &str) -> u32 {
        let mut count = 0;
        for planet in self.orbiting_planets.values() {
            if planet.name() == name {
                count += 1
            }
            let sub_count = planet.count_orbits_to(name);
            if sub_count != 0 {
                count += sub_count + 1
            }
        }
        count
    }

    pub fn total_orbits(&self) -> u32 {
        self.all_planet_names()
            .iter()
            .map(|n| self.count_orbits_to(n))
            .sum()
    }

    fn all_planet_names(&self) -> Vec<String> {
        let mut names = Vec::new();
        for planet in self.orbiting_planets.values() {
            names.push(planet.name().to_string());
            names.append(&mut planet.all_planet_names())
        }
        names
    }
}

pub fn process_orbit_map(map: &[&str]) -> Planet {
    const COM: &str = "COM";
    let mut com = Planet::new(COM);
    let mut planet_pairs: Vec<(String, String)> = map.iter().map(|e| split_orbital_relationship(e)).collect();
    while !(planet_pairs.is_empty()) {
        planet_pairs.retain(|p| !(com.add_planet_to_orbit_of(&p.1, &p.0)));
    }
    com
}

fn split_orbital_relationship(text: &str) -> (String, String) {
    let mut planet_names = Vec::new();
    for item in text.split(')') {
        planet_names.push(item)
    }
    assert_eq!(
        planet_names.len(),
        2,
        "Orbital relationship has more that two planets {}",
        &text
    );
    (
        (*planet_names.get(0).unwrap()).to_string(),
        (*planet_names.get(1).unwrap()).to_string(),
    )
}

pub fn load_orbit_input(file_name: &str) -> io::Result<Vec<String>> {
    let orbit_input = File::open(file_name)?;
    let reader = BufReader::new(orbit_input);

    let orbit_map = reader.lines().map(|s| s.unwrap()).collect();

    Ok(orbit_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_orbital_relationship() {
        assert_eq!(
            split_orbital_relationship("COM)B"),
            ("COM".to_string(), "B".to_string())
        );
        assert_eq!(
            split_orbital_relationship("YQ2)CYG"),
            ("YQ2".to_string(), "CYG".to_string())
        )
    }

    #[test]
    fn test_add_planet_to_orbit_of() {
        let mut com = Planet::new("COM");
        assert_eq!(com.add_planet_to_orbit_of("B", "COM"), true);
        assert_eq!(com.add_planet_to_orbit_of("C", "B"), true);
        assert_eq!(com.add_planet_to_orbit_of("D", "COM"), true);
        assert_eq!(com.add_planet_to_orbit_of("F", "E"), false);
    }

    #[test]
    fn test_process_orbit_map() {
        let map = [
            "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L",
        ];
        let com = process_orbit_map(&map);
        assert_eq!(com.count_orbits_to("COM"), 0, "Distance to COM");
        assert_eq!(com.count_orbits_to("D"), 3, "Distance to D");
        assert_eq!(com.count_orbits_to("L"), 7, "Distance to L");
    }

    #[test]
    fn test_all_planets() {
        let map = [
            "COM)B", "G)H", "B)C", "C)D", "D)E", "E)F", "B)G", "D)I", "E)J", "J)K", "K)L",
        ];
        let com = process_orbit_map(&map);
        let all_planets = com.all_planet_names();
        assert!(all_planets.contains(&"B".to_string()));
        assert!(all_planets.contains(&"C".to_string()));
        assert!(all_planets.contains(&"D".to_string()));
        assert!(all_planets.contains(&"E".to_string()));
        assert!(all_planets.contains(&"F".to_string()));
        assert!(all_planets.contains(&"G".to_string()));
        assert!(all_planets.contains(&"H".to_string()));
        assert!(all_planets.contains(&"I".to_string()));
        assert!(all_planets.contains(&"J".to_string()));
        assert!(all_planets.contains(&"K".to_string()));
        assert!(all_planets.contains(&"L".to_string()));
    }

    #[test]
    fn test_total_orbits() {
        let map = [
            "COM)B", "G)H", "B)C", "C)D", "D)E", "E)F", "B)G", "D)I", "E)J", "J)K", "K)L",
        ];
        let com = process_orbit_map(&map);
        assert_eq!(com.total_orbits(), 42)
    }
}
