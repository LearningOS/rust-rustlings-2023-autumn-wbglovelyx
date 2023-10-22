mod delicious_snacks {
    // Fix these use statements
    use self::fruits::PEAR;
    use self::veggies::CUCUMBER;

    pub mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    pub mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruits::PEAR,
        delicious_snacks::veggies::CUCUMBER
    );
}
