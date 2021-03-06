use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub fn generate_function_name(name: impl AsRef<str>) -> String {
    format!(
        "f{}_{}",
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(15)
            .map(char::from)
            .collect::<String>(),
        name.as_ref()
    )
}
