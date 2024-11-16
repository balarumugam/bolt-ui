#[macro_export]
macro_rules! append_child {
    ($parent:expr, $child:expr) => {
        $parent.append_child(&$child).expect("Failed to append child")
    };
}