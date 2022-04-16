pub(crate) struct Import<'a> {
    pub module: String,
    pub name: String,
    pub ty: &'a str,
}
