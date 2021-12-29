use prettytable::{format, Table};

use crate::client::Client;
use crate::client::Task;

pub fn list() {
    let client = Client::new();

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    table.set_titles(row!["Task ID", "Content"]);

    for task in client.list() {
        table.add_row(row![task.id, task.content]);
    }

    table.printstd();
}

pub fn show(task_id: isize) {
    let client = Client::new();

    let body = client.get(format!("/rest/v1/tasks/{}", task_id).as_str());

    let task: Task = serde_json::from_str(&body).unwrap();

    let mut table = Table::new();

    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    table.add_row(row!["id", task.id]);
    table.add_row(row!["assigner", task.assigner]);
    table.add_row(row!["project_id", task.project_id]);
    table.add_row(row!["section_id", task.section_id]);
    table.add_row(row!["order", task.order]);
    table.add_row(row!["content", task.content]);
    table.add_row(row!["description", task.description]);
    table.add_row(row!["completed", task.completed]);
    // TODO: table.add_row(row!["label_ids", task.label_ids]);
    table.add_row(row!["priority", task.priority]);
    table.add_row(row!["comment_count", task.comment_count]);
    table.add_row(row!["creator", task.creator]);
    table.add_row(row!["created", task.created]);
    // TODO: table.add_row(row!["due", task.due]);
    table.add_row(row!["url", task.url]);

    table.printstd();
}
