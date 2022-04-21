pub enum FilterValue<T> {
    Equal(T),
    NotEqual(T),
    Like(T),
    NotLike(T),
}

pub struct PokeFilter<T> {
    pub stat: String,
    pub value: FilterValue<T>,
}
