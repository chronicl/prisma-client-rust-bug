use prisma_client_rust_bug::db;

#[tokio::main]
async fn main() {
    let client = db::new_client().await.unwrap();
    #[cfg(debug)]
    client._db_push(false);

    client
        .user()
        .create(
            "username".to_string(),
            "email@email.com".to_string(),
            vec![],
        )
        .exec()
        .await
        .unwrap();
}
