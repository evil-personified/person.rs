# person.rs
This crate allows you to create random and unique identities.
This crate is also not fully optimized for speed, PRs are welcome.
## Usage
Simply create a random identity by calling the `Person::random` function:
```rust
use person::Person;

let person = Person::random();
```
Create a random person that's legally allowed to drink:
```rust
use chrono::{Duration, Utc};
use person::Person;

// We're using 366 days as a year here to account for leap years and
// to make sure the person is at least 21 years old.
let person = Person::random_with_dob_range(
    Utc::now() - Duration::days(366 * 40),
    Utc::now() - Duration::days(366 * 21),
);
```
Or check out what this person might pick as their social media username:
```rust
use person::Person;

println!("{}", Person::random().get_random_username());
```