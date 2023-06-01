#![cfg(feature = "integration-tests")]
#[cfg(test)]
mod tests {
    use serial_test::serial;
    use std::sync::Once;

    use shared::query::{CompoundQueryBuilder, Query, QueryType, SingleQueryBuilder};
    use tracing::{error, info, Level};
    use tracing_subscriber::FmtSubscriber;

    use client::{AuthenticatedClient, UnconnectedClient};
    use server::BINDED_URL_PORT;

    pub const USERNAME: &str = "Bob";
    pub const PASSWORD: &str = "Pomme";

    pub trait ToStringVec {
        fn to_string_vec(&self) -> Vec<String>;
    }

    impl<T: std::string::ToString> ToStringVec for [T] {
        fn to_string_vec(&self) -> Vec<String> {
            self.iter().map(|item| item.to_string()).collect()
        }
    }

    pub async fn connect_and_auth_client(
        client: UnconnectedClient,
    ) -> AuthenticatedClient {
        let client = client.connect(BINDED_URL_PORT).await.unwrap();
        client
            .authenticate(USERNAME.to_string(), PASSWORD.to_string())
            .await
            .unwrap()
    }

    pub async fn insert_some_data(client: &mut AuthenticatedClient) {
        let _ = client
            .insert(
                "users".to_string(),
                [12, 112, 29, 176].to_vec(),
                ["read", "write"].to_string_vec(),
                ["authentification", "authorization"].to_string_vec(),
            )
            .await
            .unwrap();

        let _ = client
            .insert(
                "users".to_string(),
                [12, 1, 2, 178, 76, 23, 145].to_vec(),
                ["read"].to_string_vec(),
                ["search"].to_string_vec(),
            )
            .await
            .unwrap();

        let _ = client
            .insert(
                "".to_string(),
                [12, 122, 221, 234, 178, 76, 23, 178, 97, 23, 18, 7, 6, 23, 145].to_vec(),
                ["read"].to_string_vec(),
                ["logging"].to_string_vec(),
            )
            .await
            .unwrap();

        let _ = client
            .insert(
                "posts".to_string(),
                [76, 231, 15, 13, 42, 54, 78].to_vec(),
                [].to_vec(),
                [].to_vec(),
            )
            .await
            .unwrap();

        let _ = client
            .insert(
                "documents".to_string(),
                [1, 2, 3, 4, 65, 68, 67].to_vec(),
                ["read", "write", "delete"].to_string_vec(),
                ["storage", "search"].to_string_vec(),
            )
            .await
            .unwrap();

        let user_data = vec![1, 2, 3, 4]; // Some binary data for a user
        let product_data = vec![5, 6, 7, 8]; // Some binary data for a product
        let order_data = vec![9, 10, 11, 12]; // Some binary data for an order

        let acl = ["read:all", "write:all"].to_string_vec(); // Access control list for the data
        let user_usecases = ["filter", "another_usecase"].to_string_vec();
        let product_usecases = ["filter", "yet_another_usecase"].to_string_vec();
        let order_usecases = ["filter", "different_usecase"].to_string_vec();

        // Insert user data
        client
            .insert("users".to_string(), user_data, acl.clone(), user_usecases)
            .await
            .unwrap();

        // Insert product data
        client
            .insert("products".to_string(), product_data, acl.clone(), product_usecases)
            .await
            .unwrap();

        // Insert order data
        client
            .insert("orders".to_string(), order_data, acl.clone(), order_usecases)
            .await
            .unwrap();
    }

    static INIT: Once = Once::new();

    pub fn initialize() {
        INIT.call_once(|| {
            setup_logger();
        });
    }

    fn setup_logger() {
        let subscriber = FmtSubscriber::builder().with_max_level(Level::TRACE).finish();
        tracing::subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");
    }

    #[tokio::test]
    #[serial]
    async fn test_authentification() {
        initialize();

        let client = UnconnectedClient::default();
        let client = client.connect(BINDED_URL_PORT).await.unwrap();
        let mut client = client
            .authenticate(USERNAME.to_string(), PASSWORD.to_string())
            .await
            .unwrap();
        assert!(client.is_alive());
        if let Err(err) = client.terminate_connection().await {
            error!("{:?}", err);
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_insert() {
        initialize();

        let client = UnconnectedClient::default();
        let mut client = connect_and_auth_client(client).await;

        let x = client
            .insert(
                "table".to_string(),
                [
                    12, 1, 2, 178, 4, 4, 12, 47, 31, 24, 1, 243, 12, 4, 124, 76, 234, 1,
                    76, 23, 145,
                ]
                .to_vec(),
                [].to_vec(),
                ["Tomate"].to_string_vec(),
            )
            .await
            .unwrap();

        info!("{:?}", x);

        if let Err(err) = client.terminate_connection().await {
            error!("{:?}", err);
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_simple_query() {
        initialize();

        let client = UnconnectedClient::default();
        let mut client = connect_and_auth_client(client).await;
        insert_some_data(&mut client).await;

        let user_query = SingleQueryBuilder::default()
            .with_collection("users".to_owned())
            .with_usecase("filter".to_owned())
            .build();

        let x = client.query(Query::Single(user_query)).await;
        info!("query result {:?}", x);

        if let Err(err) = client.terminate_connection().await {
            error!("{:?}", err);
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_insert_and_query() {
        initialize();

        let client = UnconnectedClient::default();
        let mut client = connect_and_auth_client(client).await;
        insert_some_data(&mut client).await;

        let user_filter = SingleQueryBuilder::default()
            .with_collection("users".to_owned())
            .with_usecase("filter".to_owned())
            .build();

        let product_filter = SingleQueryBuilder::default()
            .with_collection("products".to_owned())
            .with_usecase("filter".to_owned())
            .build();

        let sub_query = CompoundQueryBuilder::default()
            .with_query_type(QueryType::Or)
            .with_query(Query::Single(user_filter))
            .with_query(Query::Single(product_filter))
            .build();

        let order_filter = SingleQueryBuilder::default()
            .with_collection("orders".to_owned())
            .with_usecase("filter".to_owned())
            .build();

        let main_query = CompoundQueryBuilder::default()
            .with_query_type(QueryType::And)
            .with_query(Query::Single(order_filter))
            .with_query(Query::Compound(sub_query))
            .build();

        let x = client.query(Query::Compound(main_query)).await;
        info!("query result {:?}", x);

        if let Err(err) = client.terminate_connection().await {
            error!("{:?}", err);
        }
    }
}
