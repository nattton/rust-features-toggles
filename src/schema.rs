// @generated automatically by Diesel CLI.

diesel::table! {
    features (id) {
        id -> Integer,
        feature_id -> Text,
        category_id -> Integer,
        product_id -> Text,
        code -> Text,
        name_th -> Text,
        name_en -> Text,
        is_active -> Bool,
        updated_date_time -> Text,
        updated_by_id -> Text,
        updated_by_name -> Text,
        sorting_order -> Integer,
        feature_type -> Text,
    }
}
