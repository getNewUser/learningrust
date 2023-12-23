use clap::{Parser};
use crate::commands::commands::{Commands, TaskCommand};
use crate::persistence::persistence::{print_tasks, save_task, change_task_name, set_task_as_done, delete_task};

mod commands;
mod models;
mod persistence;

fn main() {
    let task_command = TaskCommand::parse();

    match task_command.command {
        Commands::Add(task) => {
            match save_task(&task) {
                Ok(_) => println!("Task '{}' has been saved successfully", task.name),
                Err(e) => println!("Failed to save task '{}': {}", task.name, e),
            }
        },
        Commands::Print(_) => {
            match print_tasks () {
                Ok(_) => println!("Tasks have been printed successfully"),
                Err(e) => println!("Failed to print tasks: {}", e),
            }
        },
        Commands::ChangeName(task) => {
            match change_task_name(&task) {
                Ok(_) => println!("Task '{}' has been updated successfully", task.name),
                Err(e) => println!("Failed to update task '{}': {}", task.name, e),
            }
        },
        Commands::SetAsDone(task) => {
            match set_task_as_done(&task) {
                Ok(_) => println!("Task '{}' has been updated successfully", task.id),
                Err(e) => println!("Failed to update task '{}': {}", task.id, e),
            }
        },
        Commands::Delete(task) => {
            match delete_task(&task) {
                Ok(_) => println!("Task '{}' has been deleted successfully", task.id),
                Err(e) => println!("Failed to delete task '{}': {}", task.id, e),
            }
        }
    }
}