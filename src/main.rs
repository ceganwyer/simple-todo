use eyre::Result;

mod todo;

fn main() -> Result<()> {
    todo::app::start()
}
