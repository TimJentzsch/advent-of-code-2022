use std::fs;

pub(crate) trait Day {
    /// The identifier of the day, e.g. "01" or "21".
    fn identifier(&self) -> &'static str;

    /// Run the program for the given day.
    fn run(&self);

    /// Get the input file for this day.
    fn get_input(&self) -> String {
        let file_name = format!("day_{}.txt", self.identifier());

        fs::read_to_string(format!("../inputs/{file_name}")).unwrap_or_else(|_| {
            panic!("Failed to read input file, make sure to add it at /inputs/{file_name}",)
        })
    }
}
