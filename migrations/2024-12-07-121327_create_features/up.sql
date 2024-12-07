CREATE TABLE features (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  feature_id VARCHAR NOT NULL,
  category_id INTEGER NOT NULL DEFAULT 0,
  product_id VARCHAR NOT NULL DEFAULT "",
  code VARCHAR NOT NULL,
  name_th VARCHAR NOT NULL,
  name_en VARCHAR NOT NULL,
  is_active BOOLEAN NOT NULL DEFAULT 0,
  updated_date_time TEXT NOT NULL,
  updated_by_id VARCHAR NOT NULL,
  updated_by_name VARCHAR NOT NULL,
  sorting_order INTEGER NOT NULL DEFAULT 0,
  feature_type VARCHAR NOT NULL
)