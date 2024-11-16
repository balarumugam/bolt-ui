#[derive(Copy, Clone)]
pub enum Operation {
    Increment,
    Decrement,
    Reset,
}

#[derive(Copy, Clone)]
pub enum TodoOperation {
    Toggle(usize),
    Remove(usize),
}

#[derive(Copy, Clone)]
pub enum Action {
    Counter(Operation),
    Todo(TodoOperation),
    ToggleTheme,
    ToggleVisibility,
}
