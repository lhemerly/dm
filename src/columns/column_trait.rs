pub trait ColumnTrait {
    fn name(&self) -> &str;
    fn len(&self) -> usize;
    fn get_as_string(&self, idx: usize) -> String;
    fn push_str(&mut self, val: &str);
    fn clone_box(&self) -> Box<dyn ColumnTrait>;
}
