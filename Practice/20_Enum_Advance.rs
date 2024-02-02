
enum PromoDiscount{
    NewUser,
    Holiday(String)
}

enum Discount{
    Percent(i32),
    Flat(i32),
    Promo(PromoDiscount),
    Custom(String),
}