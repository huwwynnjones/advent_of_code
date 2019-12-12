use std::{
    fs::File,
    io,
    io::{BufRead, BufReader},
};

fn calculate_fuel(mass: i32) -> i32 {
    let mut fuel = fuel_sub_calc(mass);
    if fuel == 0 {
        fuel
    } else {
        fuel = fuel + calculate_fuel(fuel);
        fuel
    }
}

fn fuel_sub_calc(mass: i32) -> i32 {
    let fuel = (mass as f32 / 3.0).floor() as i32;
    if (fuel - 2) < 0 {
        0
    } else {
        fuel - 2
    }
}

pub fn total_fuel(modules_mass: &[i32]) -> i32 {
    modules_mass.iter().map(|x| calculate_fuel(*x)).sum()
}

pub fn load_mass_input(file_name: &str) -> io::Result<Vec<i32>> {
    let modules_mass_input = File::open(file_name)?;
    let reader = BufReader::new(modules_mass_input);

    let mut modules_mass = Vec::new();

    reader.lines().for_each(|l| match l {
        Ok(mass) => match mass.parse::<i32>() {
            Ok(nmb) => modules_mass.push(nmb),
            Err(err) => panic!("{}", err),
        },
        Err(err) => panic!("{}", err),
    });

    Ok(modules_mass)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_fuel() {
        assert_eq!(calculate_fuel(2), 0);
        assert_eq!(calculate_fuel(12), 2);
        assert_eq!(calculate_fuel(14), 2);
        assert_eq!(calculate_fuel(1969), 966);
        assert_eq!(calculate_fuel(100756), 50346);
    }

    #[test]
    fn test_total_fuel() {
        let modules_mass = [12, 14, 1969, 100756];
        assert_eq!(total_fuel(&modules_mass), (0 + 2 + 2 + 966 + 50346))
    }

    #[test]
    fn test_load_mass_input() {
        let correct_mass = [106947, 129138, 56893, 75116, 96763];
        assert_eq!(
            load_mass_input("modules_mass_test.txt").unwrap(),
            correct_mass
        );
    }
}
