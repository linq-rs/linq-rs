use crate::{orm::Table, Variant};

#[derive(Debug, Clone)]
pub struct Primary<T, const AUTOINC: bool>
where
    T: TryFrom<Variant>,
{
    pub value: Option<T>,
}

impl<T, const AUTOINC: bool> Default for Primary<T, AUTOINC>
where
    T: TryFrom<Variant>,
{
    fn default() -> Self {
        Self { value: None }
    }
}

#[derive(Debug, Clone)]
pub struct Column<T>
where
    T: TryFrom<Variant>,
{
    pub value: Option<T>,
}

impl<T> Default for Column<T>
where
    T: TryFrom<Variant>,
{
    fn default() -> Self {
        Self { value: None }
    }
}

#[derive(Debug, Clone)]
pub struct OneToOne<T>
where
    T: Table,
{
    pub value: Option<T>,
}

impl<T> Default for OneToOne<T>
where
    T: Table,
{
    fn default() -> Self {
        Self { value: None }
    }
}

#[derive(Debug, Clone)]
pub struct OneToMany<T>
where
    T: Table,
{
    pub value: Option<Vec<T>>,
}

impl<T> Default for OneToMany<T>
where
    T: Table,
{
    fn default() -> Self {
        Self { value: None }
    }
}
