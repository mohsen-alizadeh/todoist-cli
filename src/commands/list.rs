use crate::client::Client;

pub fn list() {
    let client = Client::new();
    let list = client.list();

    for task in &list {
        println!("task : {}", task.content);
    }
}
