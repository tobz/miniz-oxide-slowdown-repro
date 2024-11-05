use std::{io::Write as _, time::Instant};

use flate2::{write::ZlibEncoder, Compression};
use rand::{rngs::StdRng, Rng as _, SeedableRng as _};

fn main() {
    // Generate 32MB of random data.
    let start_gen = Instant::now();
    let mut rng = StdRng::seed_from_u64(0xdeadbeefcafebabe);
    let mut rand_buf = [0u8; 8192];
    let mut random_data = Vec::with_capacity(32 * 1024 * 1024);

    while random_data.len() < random_data.capacity() {
        rng.fill(&mut rand_buf);
        random_data.extend_from_slice(&rand_buf);
    }
    println!("Took {:?} to generate 32MB of random data.", start_gen.elapsed());

    // Compress the random data, using zlib, a bunch of times... 32 times, to be precise.
    let start_compress = Instant::now();
    let mut compress_buf = Vec::with_capacity(32 * 1024 * 1024);

    for _ in 0..32 {
        let start_compress_round = Instant::now();
        compress_buf.clear();
        let mut encoder = ZlibEncoder::new(&mut compress_buf, Compression::default());
        encoder.write_all(&random_data).unwrap();
        let compressed_data = encoder.finish().unwrap();
        println!("Compressed 32MB of random data to {} bytes in {:?}.", compressed_data.len(), start_compress_round.elapsed());
    }

    println!("Took {:?} to compress 32MB of random data 32 times.", start_compress.elapsed());
}
