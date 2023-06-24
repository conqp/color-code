use ansi_color_codec::{ColorCodec, RESET};
use clap::Parser;
use ctrlc::set_handler;
use std::io::{stdin, stdout, BufReader, BufWriter, Read, Write};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

const STDOUT_WRITE_ERR: &str = "Could not write bytes to STDOUT";

#[derive(Parser)]
#[clap(about, author, version)]
struct Args {
    #[clap(short, long, name = "decode")]
    pub decode: bool,

    #[clap(short, long, name = "no-clear")]
    pub no_clear: bool,
}

fn main() {
    let args = Args::parse();
    let running = Arc::new(AtomicBool::new(true));
    let bytes = stream_stdin(running.clone());
    let mut stdout = BufWriter::new(stdout().lock());

    set_handler(move || {
        running.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    if args.decode {
        decode(&mut stdout, bytes);
    } else {
        encode(&mut stdout, bytes, !args.no_clear);
    }
}

fn decode(f: &mut BufWriter<impl Write>, bytes: impl Iterator<Item = u8>) {
    bytes
        .ansi_color_decode()
        .map_while(|result| match result {
            Ok(byte) => Some(byte),
            Err(error) => {
                eprintln!("{error}");
                None
            }
        })
        .for_each(|byte| {
            f.write_all(&[byte]).expect(STDOUT_WRITE_ERR);
        });

    f.flush().expect("Could not flush STDOUT");
}

fn encode(f: &mut BufWriter<impl Write>, bytes: impl Iterator<Item = u8>, clear: bool) {
    bytes.ansi_color_encode().for_each(|code| {
        write!(f, "{code}").expect(STDOUT_WRITE_ERR);
    });

    if clear {
        write!(f, "{RESET}").expect(STDOUT_WRITE_ERR);
    }
}

fn stream_stdin(running: Arc<AtomicBool>) -> impl Iterator<Item = u8> {
    BufReader::new(stdin().lock())
        .bytes()
        .take_while(move |_| running.load(Ordering::SeqCst))
        .map_while(Result::ok)
}
