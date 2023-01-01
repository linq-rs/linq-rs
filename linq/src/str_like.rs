pub trait StringLike<'a> {
    fn from_str<'b>(s: &'b str) -> Self
    where
        'b: 'a;

    fn as_str(&self) -> &str;
}

impl<'a> StringLike<'a> for &'a str {
    fn as_str(&self) -> &str {
        self
    }

    fn from_str<'b>(s: &'b str) -> Self
    where
        'b: 'a,
    {
        s
    }
}

impl<'a> StringLike<'a> for String {
    fn as_str(&self) -> &str {
        self.as_str()
    }

    fn from_str<'b>(s: &'b str) -> Self
    where
        'b: 'a,
    {
        s.to_string()
    }
}
