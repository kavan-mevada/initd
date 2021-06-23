#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Service<'a> {
    pub Name: &'a str,
    pub CapBnd: Option<u64>,
}

impl<'a> Service<'a> {
    fn run(&self) {}
}
