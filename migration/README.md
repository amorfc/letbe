# README.md

## Welcome to the Jungle of Migrations!

Alright folks, buckle up! We're about to dive into the wild world of database migrations using the Rust programming language and the sea-orm-cli tool.

### Getting Started

First things first, you need to install sea-orm-cli. It's as easy as pie, just run the following command:

```bash
cargo install sea-orm-cli
```

### Setting up the Database

Next, you need to set your DATABASE_URL. You can do this either in your environment or directly in the command line. Here's an example:

```bash
DATABASE_URL="postgres://<postgres_username>:<postgres_pwd>@<hostname>:<port>/<db_name>" sea-orm-cli migrate up
```

Just replace `<postgres_username>`, `<postgres_pwd>`, `<hostname>`, `<port>`, and `<db_name>` with your actual PostgreSQL credentials and you're good to go!

**Note:** If you're running the application with docker-compose, make sure to replace `<hostname>` with localhost. Docker can be a bit picky sometimes.

### The Workspace

Our application is structured as a workspace with different members. We have `entity` and `migration` as members of our workspace.

The `migrator` is also a member of our workspace and is accessed from the `migration` directory (crate).

And guess what? We run the `migrator` at the entry point of our application in `main.rs`.

### The Migration Structure

In our migration files, we do two things:

1. We migrate enum types of PostgreSQL. These enum types are created from our entities. All enum types defined in every entity are coming from entities defined enums.

2. We create tables from entities.

Here's an example of an entity and enum types:

```rust
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
    // Represents a db column using `Gender` active enum
    pub gender: Gender,
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
```

And here's an example of a migration file that creates a table:

```rust
// m20220101_000003_create_user_table.rs

use sea_orm_migration::{
    prelude::*,
    sea_orm::{DbBackend, Schema},
};

use entity::user::{self};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_user_table" // Make sure this matches with the file name
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Create the User table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db_postgres = DbBackend::Postgres;
        let connection = manager.get_connection();
        let schema = Schema::new(db_postgres);
        let table_create_stm = db_postgres.build(&schema.create_table_from_entity(user::Entity));
        connection.execute(table_create_stm).await?;

        Ok(())
    }

    // Define how to rollback this migration: Drop the User table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(user::Entity).to_owned())
            .await
    }
}

impl Migration {}
```

And here's an example of a migration file that creates enum types:

```rust
// m20220101_000002_create_enum.rs

use std::vec;

use sea_orm_migration::{
    prelude::*,
    sea_orm::{DbBackend, Schema},
};

use entity::user::{self, Gender, UserType};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000002_create_enum" // Make sure this matches with the file name
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Create the User table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db_postgres = DbBackend::Postgres;
        let connection = manager.get_connection();
        let schema = Schema::new(db_postgres);
        let create_stms = vec![
            schema.create_enum_from_active_enum::<UserType>(),
            schema.create_enum_from_active_enum::<Gender>(),
        ];

        let stms = create_stms
            .iter()
            .map(|stm| db_postgres.build(stm))
            .collect::<Vec<_>>();

        for stm in stms {
            connection.execute(stm).await?;
        }

        Ok(())
    }

    // Define how to rollback this migration: Drop the User table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(user::Entity).to_owned())
            .await
    }
}
```

And finally, here's where we combine the migration files:

```rust
pub use entity::prelude::*;
pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_user_table;
mod m20220101_000002_create_enum;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000002_create_enum::Migration),
            Box::new(m20220101_000001_create_user_table::Migration),
        ]
    }
}
```

## The Magic of Migrations

Alright, so you've seen a bunch of code and you're probably wondering, "How does all this come together?" Well, let me introduce you to the magic command that makes it all happen:

```rust
Migrator::up(&db, None).await?;
```

This little line of code is like the conductor of an orchestra. It takes all the individual pieces we've defined (the migrations) and runs them in the correct order.

The `up` method is responsible for applying the migrations. It takes two arguments: a reference to the database (`&db`) and an optional target version (in our case, `None` which means all migrations will be run).

When you call `Migrator::up(&db, None).await?;`, it goes through each migration, starting from the first one, and applies them to the database. It's like saying, "Alright, let's build this thing!"

And just like that, your database structure is set up according to your defined migrations. It's like magic, but better... because it's code!

And that's it! You've just navigated the jungle of migrations. Happy coding!
