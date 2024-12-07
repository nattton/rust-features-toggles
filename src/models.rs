use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::features)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Feature {
  pub id:  i32,
  pub feature_id: String,
  pub category_id: i32,
  pub product_id: String,
  pub code: String,
  pub name_th: String,
  pub name_en: String,
  pub is_active: bool,
  pub updated_date_time: String,
  pub updated_by_id: String,
  pub updated_by_name: String,
  pub sorting_order:i32,
  pub feature_type: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::features)]
pub struct NewFeature {
  pub feature_id: String,
  pub category_id: i32,
  pub product_id: String,
  pub code: String,
  pub name_th: String,
  pub name_en: String,
  pub is_active: bool,
  pub updated_date_time: String,
  pub updated_by_id: String,
  pub updated_by_name: String,
  pub sorting_order:i32,
  pub feature_type: String,
}