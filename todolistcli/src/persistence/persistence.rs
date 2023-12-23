use crate::models::task::Task;
use rusqlite::{params, Connection, Result};
use crate::commands::commands::{AddTaskCommand, ChangeTaskNameCommand, DeleteTaskCommand, SetTaskAsDoneCommand};

pub fn save_task(task_command: &AddTaskCommand) -> Result<()> {
    let conn = Connection::open("identifier.sqlite")?;
    let completed = 0;
    conn.execute(
        "INSERT INTO tasks (name, is_done) VALUES (?1, ?2)",
        params![task_command.name, completed],
    )?;
    Ok(())
}

pub fn change_task_name(task: &ChangeTaskNameCommand) -> Result<()> {
    let conn = Connection::open("identifier.sqlite")?;
    conn.execute(
        "UPDATE tasks SET name = ?1 WHERE id = ?2",
        params![task.name, task.id],
    )?;
    Ok(())
}

pub fn set_task_as_done(task: &SetTaskAsDoneCommand) -> Result<()> {
    let conn = Connection::open("identifier.sqlite")?;
    conn.execute(
        "UPDATE tasks SET is_done = 1 WHERE id = ?1",
        params![task.id],
    )?;
    Ok(())
}

pub fn delete_task(task: &DeleteTaskCommand) -> Result<()> {
    let conn = Connection::open("identifier.sqlite")?;
    conn.execute(
        "DELETE FROM tasks WHERE id = ?1",
        params![task.id],
    )?;
    Ok(())
}


pub fn print_tasks() -> Result<()>{
    let conn = Connection::open("identifier.sqlite")?;

    let mut stmt = conn.prepare("SELECT id, name, is_done FROM tasks")?;
    let task_iter = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            name: row.get(1)?,
            is_done: row.get::<_, i32>(2)?,
        })
    }).unwrap();

    for task_result in task_iter {
        match task_result {
            Ok(task) => println!("Found task {:?}", task),
            Err(e) => println!("Error getting task: {}", e),
        }
    }
    Ok(())
}