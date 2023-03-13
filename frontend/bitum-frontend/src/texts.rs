pub enum Texts {
    InternalServerError,
}

impl Texts {
    pub fn as_str(&self) -> &'static str {
        match self {
            Texts::InternalServerError => "Внутренняя ошибка сервера",
        }
    }
}
