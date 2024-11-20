
// Implement the 'Animal' trait for 'Cow'
impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "moooooo!"
    }
}