pub enum FilterValue<T> {
    NumEqual(T),
    StrEqual(T),
    Like(T),
    GreaterThan(T),
    LesserThan(T),
    OneType(T),
    TwoType(T),
}

pub struct PokeFilter<T> {
    pub stat: String,
    pub value: FilterValue<T>,
}
