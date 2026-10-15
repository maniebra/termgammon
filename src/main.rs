mod app;
mod domain;
mod infra;
mod shared;

fn main() -> std::io::Result<()> {
    infra::run()
}
