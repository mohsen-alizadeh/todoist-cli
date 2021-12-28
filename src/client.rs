use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Due {
    date: String,
    string: String,
    lang: String,
    recurring: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: isize,
    pub assigner: isize,
    pub project_id: isize,
    pub section_id: isize,
    pub order: isize,
    pub content: String,
    pub description: String,
    pub completed: bool,
    pub label_ids: Vec<isize>,
    pub priority: isize,
    pub comment_count: isize,
    pub creator: isize,
    pub created: String,
    pub due: Option<Due>,
    pub url: String,
}

pub struct Client {
    token: String,
    url: String,
}

impl Default for Client {
    fn default() -> Self {
        Self {
            url: String::from("https://api.todoist.com"),
            token: String::from(""),
        }
    }
}

impl Client {
    pub fn new() -> Client {
        Client {
            token: format!("Bearer {}", String::from(env!("TODOIST_TOKEN"))),
            ..Default::default()
        }
    }

    pub fn get(&self, path: &str) -> String {
        reqwest::blocking::Client::new()
            .get(self.path(path))
            .header(reqwest::header::AUTHORIZATION, self.token.clone())
            .send()
            .unwrap()
            .text()
            .unwrap()
    }

    fn path(&self, path: &str) -> String {
        format!("{}{}", self.url, path)
    }

    pub fn list(&self) -> Vec<Task> {
        let body = self.get("/rest/v1/tasks");
        let tasks: Vec<Task> = serde_json::from_str(&body).unwrap();
        tasks
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use httpmock::prelude::*;

    #[test]
    fn list_test() {
        let server = MockServer::start();

        let body = r#"
            [
              {
                "id": 2,
                "assigner": 0,
                "project_id": 1,
                "section_id": 0,
                "order": 15,
                "content": "git push",
                "description": "",
                "completed": false,
                "label_ids": [],
                "priority": 3,
                "comment_count": 0,
                "creator": 1,
                "created": "2021-09-20T09:00:20.856507Z",
                "due": {
                  "date": "2021-12-20",
                  "string": "every day",
                  "lang": "en",
                  "recurring": true
                },
                "url": "https://todoist.com/showTask?id=2"
              }
            ]
        "#;

        let list_tasks = server.mock(|when, then| {
            when.method(GET).path("/rest/v1/tasks");

            then.status(200)
                .header("content-type", "application/json")
                .body(body);
        });

        let client = Client::new(server.base_url());
        let list = client.list();

        list_tasks.assert();
        assert_eq!(list.len(), 1);

        let task = list.get(0);

        assert!(task.is_some());

        assert_eq!(task.unwrap().content, String::from("git push"));
    }
}
