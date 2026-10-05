pub const GET: &str = "GET";
pub const PUT: &str = "PUT";
pub const HEAD: &str = "HEAD";
pub const DELETE: &str = "DELETE";
pub const POST: &str = "POST";

pub fn get() -> String {
    format!("{}\n", GET)
}

pub fn put() -> String {
    format!("{}\n", PUT)
}

pub fn head() -> String {
    format!("{}\n", HEAD)
}

pub fn delete() -> String {
    format!("{}\n", DELETE)
}

pub fn post() -> String {
    format!("{}\n", POST)
}