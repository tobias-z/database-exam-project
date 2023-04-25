use std::io;

fn main() -> io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();
    storage::run()
}
