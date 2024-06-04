use std::collections::HashMap;

use chrono::{DateTime, Datelike, Duration, Utc};
use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};

mod list;

#[derive(Debug, Clone)]
pub struct Person {
    date_of_birth: DateTime<Utc>,
    first_name: String,
    middle_name: Option<String>,
    last_name: String,
}
impl Person {
    /// Creates a new `Person` and allows you to specify whether the `Person` should have a middle name.
    pub fn new(have_middle_name: bool) -> Self {
        let now = Utc::now();
        Self::with_dob_range(now - Duration::days(366 * 100), now, have_middle_name)
    }

    /// Creates a completely random `Person`.
    /// There is a 50% chance the `Person` will have a middle name.
    /// The `Person` will be between 0 and 100 years old.
    pub fn random() -> Self {
        let now = Utc::now();
        Self::with_dob_range(
            now - Duration::days(366 * 100),
            now,
            rand::thread_rng().gen_bool(0.5),
        )
    }

    /// Creates a new `Person` and allows you to specify the date of birth range.
    pub fn random_with_dob_range(min: DateTime<Utc>, max: DateTime<Utc>) -> Self {
        let mut rng = rand::thread_rng();
        let have_middle_name = rng.gen_bool(0.5);
        Self::with_dob_range_custom_rng(&mut rng, min, max, have_middle_name)
    }

    /// Creates a new `Person` and allows you to specify the date of birth range and whether the `Person` should have a middle name.
    /// ## Example
    /// ```rust
    /// use chrono::{Duration, Utc};
    /// use person::Person;
    /// let now = Utc::now();
    /// let person = Person::random_with_dob_range(
    ///     now - Duration::days(366 * 40),
    ///     now - Duration::days(366 * 21),
    /// );
    /// assert_eq!(person.get_age() >= 21, true);
    /// ```
    pub fn with_dob_range(min: DateTime<Utc>, max: DateTime<Utc>, have_middle_name: bool) -> Self {
        Self::with_dob_range_custom_rng(&mut rand::thread_rng(), min, max, have_middle_name)
    }

    /// Creates a new `Person` and allows you to specify the range of years
    pub fn with_dob_range_custom_rng(
        rng: &mut ThreadRng,
        min: DateTime<Utc>,
        max: DateTime<Utc>,
        have_middle_name: bool,
    ) -> Self {
        let range_millis = (max - min).num_milliseconds();
        let random_millis = rng.gen_range(0..range_millis);
        Self {
            date_of_birth: min + Duration::milliseconds(random_millis),
            first_name: list::NAMES.choose(rng).unwrap().to_string(),
            middle_name: if have_middle_name {
                Some(list::NAMES.choose(rng).unwrap().to_string())
            } else {
                None
            },
            last_name: list::SURNAMES.choose(rng).unwrap().to_string(),
        }
    }

    pub fn get_first_name(&self) -> String {
        self.first_name.clone()
    }

    pub fn get_middle_name(&self) -> Option<String> {
        self.middle_name.clone()
    }

    pub fn get_last_name(&self) -> String {
        self.last_name.clone()
    }

    pub fn get_date_of_birth(&self) -> DateTime<Utc> {
        self.date_of_birth.clone()
    }

    /// Returns the elapsed years since the `Person`'s date of birth
    pub fn get_age(&self) -> u32 {
        Utc::now().years_since(self.date_of_birth).unwrap()
    }

    /// Returns the person's full name, including the middle name.
    pub fn get_full_name(&self) -> String {
        format!(
            "{}{}{}",
            self.first_name,
            match self.middle_name.as_ref() {
                Some(mn) => format!(" {mn} "),
                _ => " ".to_string(),
            },
            self.last_name,
        )
    }

    /// Returns the person's full name with a shortened middle name.
    pub fn get_short_full_name(&self) -> String {
        format!(
            "{}{}{}",
            self.first_name,
            match self.middle_name.as_ref() {
                Some(mn) => format!(" {}. ", mn.chars().next().unwrap()),
                _ => " ".to_string(),
            },
            self.last_name,
        )
    }

    /// Generates a random username by using random separators, numbers and the person's identity.
    pub fn get_random_username(&self) -> String {
        let mut rng = rand::thread_rng();
        let number = [
            rng.gen_range(0..9999).to_string(),
            "".into(),
            self.get_age().to_string(),
            self.date_of_birth.year().to_string(),
        ]
        .choose(&mut rng)
        .unwrap()
        .clone();
        let middle_name_initial = self
            .get_middle_name()
            .unwrap_or(".".into())
            .chars()
            .next()
            .unwrap()
            .to_string();
        let divisor = [
            "".into(),
            "-".into(),
            "_".into(),
            ".".into(),
            middle_name_initial,
        ]
        .choose(&mut rng)
        .unwrap()
        .clone();

        let mut parts = vec![];
        if rng.gen_bool(0.70) {
            parts.push(repeat_last_char(&self.first_name, rng.gen_range(0..2)));
            parts.push(divisor.to_string());
            parts.push(repeat_last_char(&self.last_name, rng.gen_range(0..2)));
        } else {
            parts.push(repeat_last_char(&self.last_name, rng.gen_range(0..2)));
            parts.push(divisor.to_string());
            parts.push(repeat_last_char(&self.first_name, rng.gen_range(0..2)));
        }
        parts.push(number);

        let leet_map: HashMap<char, char> = [
            ('a', '4'),
            ('b', '8'),
            ('c', 'C'),
            ('d', 'd'),
            ('e', '3'),
            ('f', 'F'),
            ('g', '6'),
            ('h', 'h'),
            ('j', 'J'),
            ('k', 'k'),
            ('l', '1'),
            ('m', 'm'),
            ('n', 'n'),
            ('o', '0'),
            ('p', 'p'),
            ('q', 'Q'),
            ('r', 'r'),
            ('s', '5'),
            ('t', '7'),
            ('u', 'u'),
            ('v', 'v'),
            ('w', 'w'),
            ('x', 'x'),
            ('y', 'Y'),
            ('z', '2'),
        ]
        .iter()
        .cloned()
        .collect();

        leetify_string(&parts.join(""), &leet_map)
    }
}
impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.get_short_full_name(), self.get_age())
    }
}

fn repeat_last_char(s: &str, times: usize) -> String {
    let mut result = s.to_string();
    if let Some(last_char) = s.chars().last() {
        for _ in 0..times {
            result.push(last_char);
        }
    }
    result
}

fn leetify_string(input: &str, leet_map: &HashMap<char, char>) -> String {
    let mut rng = rand::thread_rng();
    let mut result = String::new();

    for (i, c) in input.chars().enumerate() {
        if i == 0 || !rng.gen_bool(0.25) {
            result.push(c);
        } else {
            result.push(leetify_char(c, leet_map));
        }
    }

    result
}

fn leetify_char(c: char, leet_map: &HashMap<char, char>) -> char {
    *leet_map.get(&c).unwrap_or(&c)
}
