pub mod paste;
pub mod token;
pub mod user;

pub use token::Token;

lazy_static! {
    pub static ref DOMAIN: String = "http://name1e5s.fun:8888".to_string();
    pub static ref PASTE_DIR: String = "/paste/".to_string();
}
