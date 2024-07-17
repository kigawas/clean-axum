# models

No axum or api dependencies should be introduced into this folder.
Only dependencies for modelling are allowed:

- serde (JSON serialization/deserialization)
- SeaORM (domain models and database)
- validator (parameter validation)
- utoipa (openapi)
