use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct AddTaskCommand {
    #[arg(short, long)]
    pub(crate) name: String
}

#[derive(Parser)]
pub struct ChangeTaskNameCommand {
    #[arg(short, long)]
    pub(crate) id: i32,
    #[arg(short, long)]
    pub(crate) name: String
}

#[derive(Parser)]
pub struct SetTaskAsDoneCommand {
    #[arg(short, long)]
    pub(crate) id: i32
}

#[derive(Parser)]
pub struct DeleteTaskCommand{
    #[arg(short, long)]
    pub(crate) id: i32
}

#[derive(Parser)]
pub struct PrintTasks {}

#[derive(Parser)]
pub struct TaskCommand {
    #[clap(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[clap(about = "add")]
    Add(AddTaskCommand),
    #[clap(about = "change task name")]
    ChangeName(ChangeTaskNameCommand),
    #[clap(about = "set task as done")]
    SetAsDone(SetTaskAsDoneCommand),
    #[clap(about = "delete task")]
    Delete(DeleteTaskCommand),
    #[clap(about = "print tasks")]
    Print(PrintTasks),
}