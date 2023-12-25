use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(People::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(People::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(People::Firstname).string().not_null())
                    .col(ColumnDef::new(People::Lastname).string())
                    .col(ColumnDef::new(People::Secondname).string())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(People::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum People {
    Table,
    Id,
    Firstname,
    Secondname,
    Lastname,
}
