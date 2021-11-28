use std::io::{self, Write as _};
use std::time::Instant;

use indicatif::{HumanDuration, ProgressIterator as _};

use crate::vec3::Rgb;

/// Write a PPM file to the standard output stream.
///
/// # Errors
///
/// If there is an error writing to stdout.
#[allow(clippy::cast_possible_truncation)]
pub fn write(width: u32, height: u32) -> io::Result<()> {
    let timer = Instant::now();

    let stdout = io::stdout();
    let lock = stdout.lock();
    // TODO: make faster with_capacity?
    let mut buf = io::BufWriter::new(lock);

    // Write header information.
    writeln!(&mut buf, "P3\n{} {}\n255", width, height)?;

    // Write pixel information.
    for j in (0..height).rev().progress() {
        for i in 0..width {
            let color = Rgb::new(
                f64::from(i) / f64::from(width - 1),
                f64::from(j) / f64::from(height - 1),
                0.25,
            );

            color.write(&mut buf)?;
        }
    }

    buf.flush()?;
    eprintln!("PPM written in {}", HumanDuration(timer.elapsed()));

    Ok(())
}
