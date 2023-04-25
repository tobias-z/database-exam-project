fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    fs_watcher::run();
}
