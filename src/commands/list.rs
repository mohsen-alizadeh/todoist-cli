use crate::client::Client;

pub fn list() {
    let url = String::from("https://api.todoist.com");
    let client = Client::new(url);
    let list = client.list();

    for task in &list {
        println!("task : {}", task.content);
    }
}
