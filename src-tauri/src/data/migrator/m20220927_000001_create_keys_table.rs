use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220927_000001_create_keys_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(
        &self,
        manager: &SchemaManager,
    ) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(Keys::Table)
                .col(
                    ColumnDef::new(Keys::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(
                    ColumnDef::new(Keys::Name)
                        .string()
                        .not_null(),
                )
                .col(
                    ColumnDef::new(Keys::Desc)
                        .text()
                        .null(),
                )
                .col(
                    ColumnDef::new(Keys::CNam)
                        .binary()
                        .not_null(),
                )
                .col(
                    ColumnDef::new(Keys::CKey)
                        .binary()
                        .not_null(),
                )
                .col(
                    ColumnDef::new(Keys::Uris)
                        .text()
                        .not_null(),
                )
                .col(
                    ColumnDef::new(Keys::Icon)
                        .text()
                        .not_null(),
                )
                .to_owned()
        ).await
    }

    async fn down(
        &self,
        manager: &SchemaManager,
    ) -> Result<(), DbErr> {
        manager.drop_table(
            Table::drop()
                .table(Keys::Table)
                .to_owned()
        ).await
    }
}

#[derive(Iden)]
pub enum Keys {
    Table,
    Id,
    Name,
    Desc,
    CNam,
    CKey,
    Uris,
    Icon,
}