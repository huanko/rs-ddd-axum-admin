use anyhow::Result;

pub async  fn execute(name: String) -> Result<()> {
    println!("Hello, {}!", name);
    Ok(())
} 