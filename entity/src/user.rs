use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increments)]
    pub id: i32,
    pub email: String,
    pub password: String,
    // Represents a db column using `UserType` active enum
    pub user_type: UserType,
    pub name: String,
    pub surname: String,
}

// Define the `UserType` active enum
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "user_type_enum")]
pub enum UserType {
    #[sea_orm(string_value = "Individual")]
    Individual,
    #[sea_orm(string_value = "Corporation")]
    Corporation,
}

impl From<String> for UserType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Individual" => UserType::Individual,
            "Corporation" => UserType::Corporation,
            _ => panic!("Invalid user type"),
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
