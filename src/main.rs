// mod args;
mod chunk;
mod chunk_type;
// mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
        let v = vec![10, 20, 40];
        dbg!(v.get(..));
        dbg!(v.get(1..));
        dbg!(v.get(2..));
        dbg!(v.get(3..));
        dbg!(v.get(4..));
        Ok(())
}