pub enum Planet {
    Mercury,
    Venus,
    Earth,
    Mars,
    Jupyter,
}

pub const MERCURY_LENGTH_OF_YEAR: f32 = 87.97_f32;
pub const VENUS_LENGTH_OF_YEAR: f32 = 255.00_f32;
pub const EARTH_LENGTH_OF_YEAR: f32 = 365.00_f32;
pub const MARS_LENGTH_OF_YEAR: f32 = 687.00_f32;
pub const JUPYTER_LENGTH_OF_YEAR: f32 = 4343.50_f32;

pub fn convert_space_age(age: u16, p: Planet) -> u32 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn age_in_earth() {
        assert_eq!(10, convert_space_age(10, Planet::Earth));
        assert_eq!(37, convert_space_age(37, Planet::Earth));
        assert_eq!(0, convert_space_age(0, Planet::Earth));
    }

    #[test]
    fn age_in_mercury() {
        assert_eq!(62, convert_space_age(15, Planet::Mercury));
    }

    #[test]
    fn age_in_venus() {
        assert_eq!(37, convert_space_age(23, Planet::Venus));
    }

    #[test]
    fn age_in_mars() {
        assert_eq!(24, convert_space_age(45, Planet::Mars));
    }

    #[test]
    fn age_in_jupyter() {
        assert_eq!(6, convert_space_age(70, Planet::Jupyter));
    }
}
