use serde::Deserialize;
use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::thread;
use std::time::{Duration, Instant};

#[derive(Deserialize)]
struct Font {
    height: usize,
    gap: usize,
    glyphs: HashMap<char, Vec<String>>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = std::fs::read_to_string("assets/fonts.json")?;
    let fonts: HashMap<String, Font> = serde_json::from_str(&text)?;
    print!("Time: ");
    io::stdout().flush()?;
    let mut input = String::new();
    let font = fonts
        .into_values()
        .find(|f| f.height == 5)
        .ok_or("no font with height 7")?;
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();

    let n: u64 = match input.parse::<u64>() {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Not a valid number: {e}");
            return Ok(());
        }
    };

    let deadline: Instant = Instant::now() + Duration::from_secs(n);
    let mut first = true;
    loop {
        let remaining = deadline.saturating_duration_since(Instant::now());

        let secs = remaining.as_millis().div_ceil(1000);
        let digits = secs.to_string();
        if !first {
            print!("\x1b[{}A", font.height);
        }
        first = false;

        for row in 0..font.height {
            print!("\x1b[2K");
            for c in digits.chars() {
                if let Some(glyph) = font.glyphs.get(&c) {
                    print!("{}{}", glyph[row], " ".repeat(font.gap));
                }
            }
            println!();
        }
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        if remaining.is_zero() {
            break;
        }
        thread::sleep(remaining.min(Duration::from_millis(250)));
    }

    Ok(())
}
