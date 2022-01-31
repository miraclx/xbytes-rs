use std::io;
use std::io::Write;
use std::thread;
use std::time;

use xbytes::{sizes::binary::bytes::*, ByteSize};

fn main() {
    let mut stdout = io::stdout();

    let total = ByteSize::of(5, GIBI_BYTE);
    let mut current = ByteSize::ZERO;

    println!("Simulating a download...");
    while current < total {
        current += ByteSize::of(92.4, MEBI_BYTE).min(total - current);
        print!(
            "\x1b[2K  [{:>3}%] ({:.2} / {:.2})\x1b[0G",
            (current * 100) / total,
            current,
            total
        );
        stdout.flush().expect("failed flushing stdout");
        thread::sleep(time::Duration::from_millis(500));
    }
    println!("\nDownload Complete!");
}
