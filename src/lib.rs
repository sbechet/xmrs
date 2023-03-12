pub mod module;
pub mod patternslot;
pub mod instrument;
pub mod envelope;
pub mod vibrato;
pub mod sample;

// load and save xm
pub mod xm;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(42, 42);
    }
}
