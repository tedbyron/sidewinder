use std::io::{self, Write as _};

#[allow(dead_code, clippy::cast_possible_truncation)]
pub fn write(width: u32, height: u32) -> io::Result<()> {
    let stdout = io::stdout();
    let lock = stdout.lock();
    // TODO: make faster with_capacity?
    let mut buf = io::BufWriter::new(lock);

    // Write header information.
    writeln!(&mut buf, "P3\n{} {}\n255", width, height)?;

    // Write pixel information.
    for j in (0..height).rev() {
        for i in 0..width {
            let r = f64::from(i) / f64::from(width - 1);
            let g = f64::from(j) / f64::from(height - 1);
            let b = 0.25;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            writeln!(&mut buf, "{} {} {}", ir, ig, ib)?;
        }
    }

    buf.flush()?;

    Ok(())
}
