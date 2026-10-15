use std::hint::black_box;
use std::time::Instant;
struct Sink(u64);
impl vte::Perform for Sink {
    fn print(&mut self, c: char) {
        self.0 = self.0.wrapping_add(c as u64);
    }
}
fn main() {
    for (name, row) in [
        ("ascii", b"README text 0123456789 abcdefghijklmnopqrstuvwxyz\r\n".as_slice()),
        ("ansi", b"\x1b[32mREADME text\x1b[0m 0123456789\r\n"),
        ("status", b"\x1b]7501;state=working:progress=42\x1b\\"),
    ] {
        let bytes = row.repeat(2048);
        let mut parser = vte::Parser::new();
        let mut sink = Sink(0);
        for _ in 0..50 {
            parser.advance(&mut sink, &bytes);
        }
        let start = Instant::now();
        for _ in 0..1000 {
            parser.advance(black_box(&mut sink), black_box(&bytes));
        }
        println!(
            "{name} {:.2} MB/s checksum={}",
            bytes.len() as f64 * 1000.0 / start.elapsed().as_secs_f64() / 1e6,
            black_box(sink.0)
        );
    }
}
