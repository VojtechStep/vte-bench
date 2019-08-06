#![feature(test)]
extern crate test;
use vte::Perform;

fn main() {}

struct DummyDispatcher;

impl Perform for DummyDispatcher {
    fn print(&mut self, _: char) {}
    fn execute(&mut self, _: u8) {}
    fn hook(&mut self, _: &[i64], _: &[u8], _: bool) {}
    fn put(&mut self, _: u8) {}
    fn unhook(&mut self) {}
    fn osc_dispatch(&mut self, _: &[&[u8]]) {}
    fn csi_dispatch(&mut self, _: &[i64], _: &[u8], _: bool, _: char) {}
    fn esc_dispatch(&mut self, _: &[i64], _: &[u8], _: bool, _: u8) {}
}

#[cfg(test)]
mod bench {
    use crate::DummyDispatcher;
    use test::{black_box, Bencher};
    use vte::Parser;

    #[bench]
    fn excess_parameters(b: &mut Bencher) {
        let mut parser = Parser::new();
        let mut dispatcher = DummyDispatcher;

        b.iter(|| {
            for _ in 0..100 {
                parser.advance(&mut dispatcher, b'\x1b');
                parser.advance(&mut dispatcher, b'[');
                for _ in 0..10000 {
                    black_box(parser.advance(&mut dispatcher, b';'));
                }
                parser.advance(&mut dispatcher, b'p');
            }
        })
    }

    #[bench]
    fn normal_parameters(b: &mut Bencher) {
        let mut parser = Parser::new();
        let mut dispatcher = DummyDispatcher;

        b.iter(|| {
            for _ in 0..100 {
                parser.advance(&mut dispatcher, b'\x1b');
                parser.advance(&mut dispatcher, b'[');
                for _ in 0..10 {
                    black_box(parser.advance(&mut dispatcher, b';'));
                }
                parser.advance(&mut dispatcher, b'p');
            }
        })
    }
}
