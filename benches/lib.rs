#![feature(test)]

extern crate test;
extern crate hex;


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_encode_512bits(b: &mut Bencher) {
        b.iter(|| {
            hex::encode("5d07719b61b0abb6f1c3b17b1d69c838278f87f9b5e75077026e5fedf96c2eb25d07719b61b0abb6f1c3b17b1d69c838278f87f9b5e75077026e5fedf96c2eb2")
        });
    }

    #[bench]
    fn bench_encode_256bits(b: &mut Bencher) {
        b.iter(|| {
            hex::encode("5d07719b61b0abb6f1c3b17b1d69c838278f87f9b5e75077026e5fedf96c2eb2")
        });
    }

    #[bench]
    fn bench_encode_128bits(b: &mut Bencher) {
        b.iter(|| {
            hex::encode("5d07719b61b0abb6f1c3b17b1d69c838")
        });
    }

    #[bench]
    fn bench_encode_64bits(b: &mut Bencher) {
        b.iter(|| {
            hex::encode("5d07719b61b0abb6")
        });
    }

    #[bench]
    fn bench_encode_32bits(b: &mut Bencher) {
        b.iter(|| {
            hex::encode("5d07719b")
        });
    }

    #[bench]
    fn bench_encode_16bits(b: &mut Bencher) {
        b.iter(|| {
            hex::encode("5d07")
        });
    }
}
