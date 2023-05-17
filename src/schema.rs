// @generated automatically by Diesel CLI.

diesel::table! {
    laptops (id) {
        id -> Int4,
        brand -> Text,
        model -> Text,
        cpu -> Text,
        gpu -> Text,
        ram_gb -> Int4,
        price -> Numeric,
    }
}
