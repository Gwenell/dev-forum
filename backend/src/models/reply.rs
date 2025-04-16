#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            id: Set(Uuid::new_v4()),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            ..ActiveModelTrait::default()
        }
    }

    async fn before_save<C>(&mut self, _db: &C, _insert: bool) -> Result<(), DbErr>
    where
        C: ConnectionTrait,
    {
        self.updated_at = Set(Utc::now());
        
        Ok(())
    }
} 