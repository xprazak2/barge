use barge::store::{store_client, StoreMsg, store_server};

use tokio::sync::mpsc;

#[tokio::test]
async fn store_should_work() {
  let (tx, rx) = mpsc::channel::<StoreMsg>(10);

  let _store_actor = store_server::start_store(rx);

  let empty_res = store_client::list_store(tx.clone()).await.expect("Should show that store is empty");
  assert!(empty_res.is_empty());

  store_client::add_peer(tx.clone(), 42).await.expect("Should add peer to store");
  let store_res = store_client::list_store(tx.clone()).await.expect("Should show that store has one peer");
  assert_eq!(store_res.len(), 1);
  assert_eq!(store_res[0], 42);

  store_client::remove_peer(tx.clone(), 42).await.expect("Should remove peer from store");
  let empty_res = store_client::list_store(tx).await.expect("Should show that store is empty again");
  assert!(empty_res.is_empty());
}
