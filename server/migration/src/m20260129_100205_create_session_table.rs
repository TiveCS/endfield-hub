use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("sessions")
                    .if_not_exists()
                    .col(pk_uuid("id"))
                    .col(string("token").not_null().unique_key())
                    .col(uuid("user_id").not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from("sessions", "user_id")
                            .to("users", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("sessions").to_owned())
            .await
    }
}
