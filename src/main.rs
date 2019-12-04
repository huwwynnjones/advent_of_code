use std::io::{BufReader, BufRead};
use std::io;
use std::fs::File;

fn main() -> io::Result<()>  {
    let modules_mass_input = File::open("modules_mass.txt")?;
    let reader = BufReader::new(modules_mass_input);

    let mut modules_mass = Vec::new();
    
    reader.lines().for_each(|l| match l {
        Ok(mass) => match mass.parse::<i32>() {
            Ok(nmb) => modules_mass.push(nmb),
            Err(err) => panic!("{}", err)    
        },
        Err(err) => panic!("{}", err)
    });

    let total = total_fuel(&modules_mass);
    println!("The total fuel needed is {}",total);
    Ok(())
}

fn calculate_fuel(mass: i32) -> i32 {
    ((mass as f32 / 3.0).floor() - 2.0) as i32
}

fn total_fuel(modules_mass: &[i32]) -> i32 {
    modules_mass.iter().map(|x| calculate_fuel(*x)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_fuel(){
        assert_eq!(calculate_fuel(12), 2);
        assert_eq!(calculate_fuel(14), 2);
        assert_eq!(calculate_fuel(1969), 654);
        assert_eq!(calculate_fuel(100756), 33583);
    }

    #[test]
    fn test_total_fuel() {
        let modules_mass = [12, 14, 1969, 100756];
        assert_eq!(total_fuel(&modules_mass), 34241)
    }
}