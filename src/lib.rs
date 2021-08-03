/// Generalization of an exercise that can have one single expected result or several.
pub trait Exercise {
    /// Returns the description of the exercise. This should help the user understand what should
    /// be done.
    fn get_description(&self) -> String;
    /// Returns a String that should test the input provided by the user.
    fn get_expected_result(&self) -> String {
        "".to_string()
    }
    /// Returns a Vector of Strings that one of it should test the input provided by the user.
    fn get_expected_results(&self) -> Vec<String> {
        let expected_result = self.get_expected_result();
        vec![expected_result]
    }
    /// Returns the property to sort the exercises.
    /// This is useful when you want to control the order in which the exercises appear.
    /// If this function is not implemented by a concrete Struct, the order of the exercises can
    /// be completely random.
    fn get_sort_property(&self) -> String {
        "".to_string()
    }
}
