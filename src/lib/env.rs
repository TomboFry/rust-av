use dotenv;

pub fn init () {
	dotenv::dotenv().ok();
}

pub fn env (key : &str) -> String {
	std::env::var(key).expect(&format!("Could not load `{}` from .env file", key))
}
