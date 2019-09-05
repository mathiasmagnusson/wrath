#[cfg(feature = "big-floats")]
pub type Float = f64;
#[cfg(not(feature = "big-floats"))]
pub type Float = f32;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
