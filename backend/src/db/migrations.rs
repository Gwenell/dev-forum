use sea_orm_migration::prelude::*;
use uuid::Uuid;
use chrono::Utc;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240410_000001_create_users_table::Migration),
            Box::new(m20240410_000002_create_categories_table::Migration),
            Box::new(m20240410_000003_create_subcategories_table::Migration),
            Box::new(m20240410_000004_create_threads_table::Migration),
            Box::new(m20240410_000005_create_posts_table::Migration),
            Box::new(m20240410_000006_create_files_table::Migration),
            Box::new(m20240410_000007_create_chat_messages_table::Migration),
            Box::new(m20240410_000008_create_streams_table::Migration),
            Box::new(m20240410_000009_create_initial_data::Migration),
        ]
    }
}

mod m20240410_000001_create_users_table {
    use super::*;

    #[derive(DeriveMigrationName)]
    pub struct Migration;

    #[async_trait::async_trait]
    impl MigrationTrait for Migration {
        async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
            manager
                .create_table(
                    Table::create()
                        .table(Users::Table)
                        .if_not_exists()
                        .col(
                            ColumnDef::new(Users::Id)
                                .uuid()
                                .not_null()
                                .primary_key(),
                        )
                        .col(ColumnDef::new(Users::Username).string().not_null().unique_key())
                        .col(ColumnDef::new(Users::Email).string().not_null().unique_key())
                        .col(ColumnDef::new(Users::PasswordHash).string().not_null())
                        .col(ColumnDef::new(Users::DisplayName).string())
                        .col(ColumnDef::new(Users::Bio).text())
                        .col(ColumnDef::new(Users::AvatarUrl).string())
                        .col(ColumnDef::new(Users::ThemePreference).string())
                        .col(ColumnDef::new(Users::IsAdmin).boolean().not_null().default(false))
                        .col(ColumnDef::new(Users::IsActive).boolean().not_null().default(true))
                        .col(ColumnDef::new(Users::CreatedAt).timestamp().not_null())
                        .col(ColumnDef::new(Users::UpdatedAt).timestamp().not_null())
                        .col(ColumnDef::new(Users::LastLogin).timestamp())
                        .to_owned(),
                )
                .await
        }

        async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
            manager
                .drop_table(Table::drop().table(Users::Table).to_owned())
                .await
        }
    }

    #[derive(DeriveIden)]
    enum Users {
        Table,
        Id,
        Username,
        Email,
        PasswordHash,
        DisplayName,
        Bio,
        AvatarUrl,
        ThemePreference,
        IsAdmin,
        IsActive,
        CreatedAt,
        UpdatedAt,
        LastLogin,
    }
}

mod m20240410_000002_create_categories_table {
    use super::*;

    #[derive(DeriveMigrationName)]
    pub struct Migration;

    #[async_trait::async_trait]
    impl MigrationTrait for Migration {
        async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
            manager
                .create_table(
                    Table::create()
                        .table(Categories::Table)
                        .if_not_exists()
                        .col(
                            ColumnDef::new(Categories::Id)
                                .uuid()
                                .not_null()
                                .primary_key(),
                        )
                        .col(ColumnDef::new(Categories::Name).string().not_null())
                        .col(ColumnDef::new(Categories::Slug).string().not_null().unique_key())
                        .col(ColumnDef::new(Categories::Description).text().not_null())
                        .col(ColumnDef::new(Categories::Icon).string())
                        .col(ColumnDef::new(Categories::DisplayOrder).integer().not_null().default(0))
                        .col(ColumnDef::new(Categories::CreatedAt).timestamp().not_null())
                        .col(ColumnDef::new(Categories::UpdatedAt).timestamp().not_null())
                        .to_owned(),
                )
                .await
        }

        async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
            manager
                .drop_table(Table::drop().table(Categories::Table).to_owned())
                .await
        }
    }

    #[derive(DeriveIden)]
    enum Categories {
        Table,
        Id,
        Name,
        Slug,
        Description,
        Icon,
        DisplayOrder,
        CreatedAt,
        UpdatedAt,
    }
}

mod m20240410_000003_create_subcategories_table {
    use super::*;

    #[derive(DeriveMigrationName)]
    pub struct Migration;

    #[async_trait::async_trait]
    impl MigrationTrait for Migration {
        async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
            manager
                .create_table(
                    Table::create()
                        .table(Subcategories::Table)
                        .if_not_exists()
                        .col(
                            ColumnDef::new(Subcategories::Id)
                                .uuid()
                                .not_null()
                                .primary_key(),
                        )
                        .col(ColumnDef::new(Subcategories::CategoryId).uuid().not_null())
                        .col(ColumnDef::new(Subcategories::Name).string().not_null())
                        .col(ColumnDef::new(Subcategories::Slug).string().not_null().unique_key())
                        .col(ColumnDef::new(Subcategories::Description).text().not_null())
                        .col(ColumnDef::new(Subcategories::Icon).string())
                        .col(ColumnDef::new(Subcategories::DisplayOrder).integer().not_null().default(0))
                        .col(ColumnDef::new(Subcategories::CreatedAt).timestamp().not_null())
                        .col(ColumnDef::new(Subcategories::UpdatedAt).timestamp().not_null())
                        .foreign_key(
                            ForeignKey::create()
                                .name("fk_subcategory_category")
                                .from(Subcategories::Table, Subcategories::CategoryId)
                                .to(Categories::Table, Categories::Id)
                                .on_delete(ForeignKeyAction::Cascade)
                                .on_update(ForeignKeyAction::Cascade),
                        )
                        .to_owned(),
                )
                .await
        }

        async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
            manager
                .drop_table(Table::drop().table(Subcategories::Table).to_owned())
                .await
        }
    }

    #[derive(DeriveIden)]
    enum Subcategories {
        Table,
        Id,
        CategoryId,
        Name,
        Slug,
        Description,
        Icon,
        DisplayOrder,
        CreatedAt,
        UpdatedAt,
    }

    #[derive(DeriveIden)]
    enum Categories {
        Table,
        Id,
    }
}

mod m20240410_000004_create_threads_table {
    use super::*;

    #[derive(DeriveMigrationName)]
    pub struct Migration;

    #[async_trait::async_trait]
    impl MigrationTrait for Migration {
        async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
            manager
                .create_table(
                    Table::create()
                        .table(Threads::Table)
                        .if_not_exists()
                        .col(
                            ColumnDef::new(Threads::Id)
                                .uuid()
                                .not_null()
                                .primary_key(),
                        )
                        .col(ColumnDef::new(Threads::SubcategoryId).uuid().not_null())
                        .col(ColumnDef::new(Threads::UserId).uuid().not_null())
                        .col(ColumnDef::new(Threads::Title).string().not_null())
                        .col(ColumnDef::new(Threads::Slug).string().not_null().unique_key())
                        .col(ColumnDef::new(Threads::IsPinned).boolean().not_null().default(false))
                        .col(ColumnDef::new(Threads::IsLocked).boolean().not_null().default(false))
                        .col(ColumnDef::new(Threads::Views).integer().not_null().default(0))
                        .col(ColumnDef::new(Threads::CreatedAt).timestamp().not_null())
                        .col(ColumnDef::new(Threads::UpdatedAt).timestamp().not_null())
                        .col(ColumnDef::new(Threads::LastPostAt).timestamp().not_null())
                        .foreign_key(
                            ForeignKey::create()
                                .name("fk_thread_subcategory")
                                .from(Threads::Table, Threads::SubcategoryId)
                                .to(Subcategories::Table, Subcategories::Id)
                                .on_delete(ForeignKeyAction::Cascade)
                                .on_update(ForeignKeyAction::Cascade),
                        )
                        .foreign_key(
                            ForeignKey::create()
                                .name("fk_thread_user")
                                .from(Threads::Table, Threads::UserId)
                                .to(Users::Table, Users::Id)
                                .on_delete(ForeignKeyAction::Cascade)
                                .on_update(ForeignKeyAction::Cascade),
                        )
                        .to_owned(),
                )
                .await
        }

        async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
            manager
                .drop_table(Table::drop().table(Threads::Table).to_owned())
                .await
        }
    }

    #[derive(DeriveIden)]
    enum Threads {
        Table,
        Id,
        SubcategoryId,
        UserId,
        Title,
        Slug,
        IsPinned,
        IsLocked,
        Views,
        CreatedAt,
        UpdatedAt,
        LastPostAt,
    }

    #[derive(DeriveIden)]
    enum Subcategories {
        Table,
        Id,
    }

    #[derive(DeriveIden)]
    enum Users {
        Table,
        Id,
    }
}

mod m20240410_000005_create_posts_table {
    use super::*;

    #[derive(DeriveMigrationName)]
    pub struct Migration;

    #[async_trait::async_trait]
    impl MigrationTrait for Migration {
        async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
            manager
                .create_table(
                    Table::create()
                        .table(Posts::Table)
                        .if_not_exists()
                        .col(
                            ColumnDef::new(Posts::Id)
                                .uuid()
                                .not_null()
                                .primary_key(),
                        )
                        .col(ColumnDef::new(Posts::ThreadId).uuid().not_null())
                        .col(ColumnDef::new(Posts::UserId).uuid().not_null())
                        .col(ColumnDef::new(Posts::Content).text().not_null())
                        .col(ColumnDef::new(Posts::CodeContent).text())
                        .col(ColumnDef::new(Posts::CodeLanguage).string())
                        .col(ColumnDef::new(Posts::IsEdited).boolean().not_null().default(false))
                        .col(ColumnDef::new(Posts::CreatedAt).timestamp().not_null())
                        .col(ColumnDef::new(Posts::UpdatedAt).timestamp().not_null())
                        .foreign_key(
                            ForeignKey::create()
                                .name("fk_post_thread")
                                .from(Posts::Table, Posts::ThreadId)
                                .to(Threads::Table, Threads::Id)
                                .on_delete(ForeignKeyAction::Cascade)
                                .on_update(ForeignKeyAction::Cascade),
                        )
                        .foreign_key(
                            ForeignKey::create()
                                .name("fk_post_user")
                                .from(Posts::Table, Posts::UserId)
                                .to(Users::Table, Users::Id)
                                .on_delete(ForeignKeyAction::Cascade)
                                .on_update(ForeignKeyAction::Cascade),
                        )
                        .to_owned(),
                )
                .await
        }

        async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
            manager
                .drop_table(Table::drop().table(Posts::Table).to_owned())
                .await
        }
    }

    #[derive(DeriveIden)]
    enum Posts {
        Table,
        Id,
        ThreadId,
        UserId,
        Content,
        CodeContent,
        CodeLanguage,
        IsEdited,
        CreatedAt,
        UpdatedAt,
    }

    #[derive(DeriveIden)]
    enum Threads {
        Table,
        Id,
    }

    #[derive(DeriveIden)]
    enum Users {
        Table,
        Id,
    }
}

mod m20240410_000006_create_files_table {
    use super::*;

    #[derive(DeriveMigrationName)]
    pub struct Migration;

    #[async_trait::async_trait]
    impl MigrationTrait for Migration {
        async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
            manager
                .create_table(
                    Table::create()
                        .table(Files::Table)
                        .if_not_exists()
                        .col(
                            ColumnDef::new(Files::Id)
                                .uuid()
                                .not_null()
                                .primary_key(),
                        )
                        .col(ColumnDef::new(Files::UserId).uuid().not_null())
                        .col(ColumnDef::new(Files::FileName).string().not_null())
                        .col(ColumnDef::new(Files::FilePath).string().not_null())
                        .col(ColumnDef::new(Files::FileSize).big_integer().not_null())
                        .col(ColumnDef::new(Files::MimeType).string().not_null())
                        .col(ColumnDef::new(Files::Description).text())
                        .col(ColumnDef::new(Files::Platform).string())
                        .col(ColumnDef::new(Files::DownloadCount).integer().not_null().default(0))
                        .col(ColumnDef::new(Files::IsMalwareScanned).boolean().not_null().default(false))
                        .col(ColumnDef::new(Files::IsSafe).boolean().not_null().default(false))
                        .col(ColumnDef::new(Files::CreatedAt).timestamp().not_null())
                        .col(ColumnDef::new(Files::UpdatedAt).timestamp().not_null())
                        .foreign_key(
                            ForeignKey::create()
                                .name("fk_file_user")
                                .from(Files::Table, Files::UserId)
                                .to(Users::Table, Users::Id)
                                .on_delete(ForeignKeyAction::Cascade)
                                .on_update(ForeignKeyAction::Cascade),
                        )
                        .to_owned(),
                )
                .await
        }

        async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
            manager
                .drop_table(Table::drop().table(Files::Table).to_owned())
                .await
        }
    }

    #[derive(DeriveIden)]
    enum Files {
        Table,
        Id,
        UserId,
        FileName,
        FilePath,
        FileSize,
        MimeType,
        Description,
        Platform,
        DownloadCount,
        IsMalwareScanned,
        IsSafe,
        CreatedAt,
        UpdatedAt,
    }

    #[derive(DeriveIden)]
    enum Users {
        Table,
        Id,
    }
}

mod m20240410_000007_create_chat_messages_table {
    use super::*;

    #[derive(DeriveMigrationName)]
    pub struct Migration;

    #[async_trait::async_trait]
    impl MigrationTrait for Migration {
        async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
            manager
                .create_table(
                    Table::create()
                        .table(ChatMessages::Table)
                        .if_not_exists()
                        .col(
                            ColumnDef::new(ChatMessages::Id)
                                .uuid()
                                .not_null()
                                .primary_key(),
                        )
                        .col(ColumnDef::new(ChatMessages::UserId).uuid().not_null())
                        .col(ColumnDef::new(ChatMessages::RoomId).string().not_null())
                        .col(ColumnDef::new(ChatMessages::Message).text().not_null())
                        .col(ColumnDef::new(ChatMessages::IsCode).boolean().not_null().default(false))
                        .col(ColumnDef::new(ChatMessages::CodeLanguage).string())
                        .col(ColumnDef::new(ChatMessages::CreatedAt).timestamp().not_null())
                        .foreign_key(
                            ForeignKey::create()
                                .name("fk_chat_message_user")
                                .from(ChatMessages::Table, ChatMessages::UserId)
                                .to(Users::Table, Users::Id)
                                .on_delete(ForeignKeyAction::Cascade)
                                .on_update(ForeignKeyAction::Cascade),
                        )
                        .to_owned(),
                )
                .await?;

            // Create an index on room_id to speed up message retrieval
            manager
                .create_index(
                    Index::create()
                        .name("idx_chat_messages_room_id")
                        .table(ChatMessages::Table)
                        .col(ChatMessages::RoomId)
                        .to_owned(),
                )
                .await
        }

        async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
            manager
                .drop_table(Table::drop().table(ChatMessages::Table).to_owned())
                .await
        }
    }

    #[derive(DeriveIden)]
    enum ChatMessages {
        Table,
        Id,
        UserId,
        RoomId,
        Message,
        IsCode,
        CodeLanguage,
        CreatedAt,
    }

    #[derive(DeriveIden)]
    enum Users {
        Table,
        Id,
    }
}

mod m20240410_000008_create_streams_table {
    use super::*;

    #[derive(DeriveMigrationName)]
    pub struct Migration;

    #[async_trait::async_trait]
    impl MigrationTrait for Migration {
        async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
            manager
                .create_table(
                    Table::create()
                        .table(Streams::Table)
                        .if_not_exists()
                        .col(
                            ColumnDef::new(Streams::Id)
                                .uuid()
                                .not_null()
                                .primary_key(),
                        )
                        .col(ColumnDef::new(Streams::UserId).uuid().not_null())
                        .col(ColumnDef::new(Streams::Title).string().not_null())
                        .col(ColumnDef::new(Streams::Description).text())
                        .col(ColumnDef::new(Streams::StreamKey).string().not_null().unique_key())
                        .col(ColumnDef::new(Streams::IsActive).boolean().not_null().default(false))
                        .col(ColumnDef::new(Streams::Resolution).string())
                        .col(ColumnDef::new(Streams::RefreshRate).integer())
                        .col(ColumnDef::new(Streams::ViewerCount).integer().not_null().default(0))
                        .col(ColumnDef::new(Streams::ScheduledStart).timestamp())
                        .col(ColumnDef::new(Streams::StartedAt).timestamp())
                        .col(ColumnDef::new(Streams::EndedAt).timestamp())
                        .col(ColumnDef::new(Streams::CreatedAt).timestamp().not_null())
                        .col(ColumnDef::new(Streams::UpdatedAt).timestamp().not_null())
                        .foreign_key(
                            ForeignKey::create()
                                .name("fk_stream_user")
                                .from(Streams::Table, Streams::UserId)
                                .to(Users::Table, Users::Id)
                                .on_delete(ForeignKeyAction::Cascade)
                                .on_update(ForeignKeyAction::Cascade),
                        )
                        .to_owned(),
                )
                .await
        }

        async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
            manager
                .drop_table(Table::drop().table(Streams::Table).to_owned())
                .await
        }
    }

    #[derive(DeriveIden)]
    enum Streams {
        Table,
        Id,
        UserId,
        Title,
        Description,
        StreamKey,
        IsActive,
        Resolution,
        RefreshRate,
        ViewerCount,
        ScheduledStart,
        StartedAt,
        EndedAt,
        CreatedAt,
        UpdatedAt,
    }

    #[derive(DeriveIden)]
    enum Users {
        Table,
        Id,
    }
}

mod m20240410_000009_create_initial_data {
    use super::*;
    use sea_orm::{ConnectionTrait, Statement, DbBackend};
    use argon2::{
        password_hash::{rand_core::OsRng, SaltString},
        Argon2, PasswordHasher
    };

    #[derive(DeriveMigrationName)]
    pub struct Migration;

    #[async_trait::async_trait]
    impl MigrationTrait for Migration {
        async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
            // Create admin user
            let admin_id = Uuid::new_v4();
            let password = "adminpassword"; // This should be changed in production
            let salt = SaltString::generate(&mut OsRng);
            let argon2 = Argon2::default();
            let password_hash = argon2.hash_password(password.as_bytes(), &salt)
                .map_err(|_| DbErr::Custom("Failed to hash password".to_string()))?
                .to_string();
            let now = Utc::now();

            let insert_admin = format!(
                "INSERT INTO users (id, username, email, password_hash, is_admin, created_at, updated_at) 
                 VALUES ('{}', 'admin', 'admin@example.com', '{}', true, '{}', '{}')",
                admin_id, password_hash, now, now
            );

            let db = manager.get_connection();
            db.execute(Statement::from_string(
                DbBackend::MySql,
                insert_admin,
            ))
            .await?;

            // Create categories and subcategories
            self.create_category_with_subcategories(
                db, 
                "Programming Languages",
                "programming-languages",
                "Discussions about various programming languages",
                1,
                vec![
                    ("Python", "python", "Discussions about Python programming language"),
                    ("Rust", "rust", "Discussions about Rust programming language"),
                    ("JavaScript", "javascript", "Discussions about JavaScript programming language"),
                    ("Go", "go", "Discussions about Go programming language"),
                    ("Others", "others", "Discussions about other programming languages"),
                ]
            ).await?;

            self.create_category_with_subcategories(
                db, 
                "Development Tools",
                "development-tools",
                "Topics related to IDEs, text editors, and other development tools",
                2,
                vec![
                    ("VS Code", "vs-code", "Discussions about Visual Studio Code"),
                    ("JetBrains Suite", "jetbrains-suite", "Discussions about JetBrains tools"),
                    ("Vim/Neovim", "vim-neovim", "Discussions about Vim and Neovim"),
                    ("Emacs", "emacs", "Discussions about Emacs"),
                    ("Others", "others-tools", "Discussions about other development tools"),
                ]
            ).await?;

            self.create_category_with_subcategories(
                db, 
                "Frameworks & Libraries",
                "frameworks-libraries",
                "Discussions on popular frameworks and libraries",
                3,
                vec![
                    ("React", "react", "Discussions about React"),
                    ("Vue.js", "vue-js", "Discussions about Vue.js"),
                    ("Django", "django", "Discussions about Django"),
                    ("Flask", "flask", "Discussions about Flask"),
                    ("Others", "others-frameworks", "Discussions about other frameworks and libraries"),
                ]
            ).await?;

            self.create_category_with_subcategories(
                db, 
                "Code Sharing",
                "code-sharing",
                "A space for users to share and review code snippets",
                4,
                vec![
                    ("Frontend", "frontend", "Frontend code sharing"),
                    ("Backend", "backend", "Backend code sharing"),
                    ("Full Stack", "full-stack", "Full stack code sharing"),
                    ("Algorithms", "algorithms", "Algorithm implementations"),
                    ("Others", "others-code", "Other code sharing"),
                ]
            ).await?;

            self.create_category_with_subcategories(
                db, 
                "Tips & Tutorials",
                "tips-tutorials",
                "Guides, tutorials, and best practices",
                5,
                vec![
                    ("Beginner", "beginner", "Tips and tutorials for beginners"),
                    ("Intermediate", "intermediate", "Tips and tutorials for intermediate users"),
                    ("Advanced", "advanced", "Tips and tutorials for advanced users"),
                    ("Best Practices", "best-practices", "Software development best practices"),
                    ("Others", "others-tips", "Other tips and tutorials"),
                ]
            ).await?;

            self.create_category_with_subcategories(
                db, 
                "Live Streams",
                "live-streams",
                "For users to host and join live coding sessions",
                6,
                vec![
                    ("Upcoming Streams", "upcoming-streams", "Upcoming live coding streams"),
                    ("Past Streams", "past-streams", "Past live coding streams"),
                    ("Stream Requests", "stream-requests", "Requests for specific live streams"),
                    ("Others", "others-streams", "Other live stream related discussions"),
                ]
            ).await?;

            self.create_category_with_subcategories(
                db, 
                "App Sharing",
                "app-sharing",
                "A category for sharing and downloading applications",
                7,
                vec![
                    ("Windows", "windows", "Applications for Windows"),
                    ("macOS", "macos", "Applications for macOS"),
                    ("Linux", "linux", "Applications for Linux"),
                    ("Cross-Platform", "cross-platform", "Cross-platform applications"),
                    ("Others", "others-apps", "Other applications"),
                ]
            ).await?;

            self.create_category_with_subcategories(
                db, 
                "General Discussion",
                "general-discussion",
                "Off-topic discussions and community building",
                8,
                vec![
                    ("Introductions", "introductions", "Introduce yourself to the community"),
                    ("Off-Topic", "off-topic", "Off-topic discussions"),
                    ("Community Events", "community-events", "Community events and announcements"),
                    ("Others", "others-general", "Other general discussions"),
                ]
            ).await?;

            Ok(())
        }

        async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
            // This will be handled by dropping the tables
            Ok(())
        }
    }

    impl Migration {
        async fn create_category_with_subcategories(
            &self,
            db: &impl ConnectionTrait,
            category_name: &str,
            category_slug: &str,
            category_description: &str,
            display_order: i32,
            subcategories: Vec<(&str, &str, &str)>,
        ) -> Result<(), DbErr> {
            let category_id = Uuid::new_v4();
            let now = Utc::now();

            let insert_category = format!(
                "INSERT INTO categories (id, name, slug, description, display_order, created_at, updated_at) 
                 VALUES ('{}', '{}', '{}', '{}', {}, '{}', '{}')",
                category_id, category_name, category_slug, category_description, display_order, now, now
            );

            db.execute(Statement::from_string(
                DbBackend::MySql,
                insert_category,
            ))
            .await?;

            for (i, (name, slug, description)) in subcategories.iter().enumerate() {
                let subcategory_id = Uuid::new_v4();
                let insert_subcategory = format!(
                    "INSERT INTO subcategories (id, category_id, name, slug, description, display_order, created_at, updated_at) 
                     VALUES ('{}', '{}', '{}', '{}', '{}', {}, '{}', '{}')",
                    subcategory_id, category_id, name, slug, description, i as i32, now, now
                );

                db.execute(Statement::from_string(
                    DbBackend::MySql,
                    insert_subcategory,
                ))
                .await?;
            }

            Ok(())
        }
    }
} 