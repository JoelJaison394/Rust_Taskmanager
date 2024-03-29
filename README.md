# Rust Task Manager
![Rust Task Manager](https://dev-to-uploads.s3.amazonaws.com/uploads/articles/j80rz0b0c27jwznokg6f.png)

Welcome to the Rust Task Manager project! 🚀 This project is a simple yet effective task manager implemented in Rust, designed for command-line use.

## Table of Contents
- [Features](#features)
- [Usage](#usage)
- [How to Run](#how-to-run)
- [Contributing](#contributing)

## Features

### Task Struct
The `Task` struct represents a task with the following attributes:
- `id: u32` - unique identifier for each task.
- `title: String` - the title of the task.
- `description: String` - a detailed description of the task.
- `completed: bool` - indicates whether the task is completed or not.

### Task Methods
The `Task` struct includes methods for:
- Adding a new task.
- Listing all tasks.
- Editing a task.
- Deleting a task.
- Marking a task as completed.

### Main Function
The `main` function provides a command-line interface to interact with the task manager. It includes options to add, list, edit, delete, mark as completed, and exit the program.

## Usage

1. **Add a Task**
    - Choose option `1` to add a new task.
    - Enter the title and description when prompted.

2. **List All Tasks**
    - Choose option `2` to display a list of all tasks.

3. **Edit a Task**
    - Choose option `3` and enter the task ID to edit the corresponding task.
    - Follow the prompts to update the title and description.

4. **Delete a Task**
    - Choose option `4` and enter the task ID to delete the corresponding task.

5. **Mark a Task as Completed**
    - Choose option `5` and enter the task ID to mark the corresponding task as completed.

6. **Exit Program**
    - Choose option `6` to exit the program.

## How to Run

1. Ensure you have Rust installed on your system.
2. Clone this repository:
    ```bash
    git clone https://github.com/JoelJaison394/Rust_Taskmanager.git
    cd rust-task-manager
    ```
3. Run the project:
    ```bash
    cargo run
    ```

## Contributing

Contributions are welcome! If you have any improvements or features to add, feel free to open an issue or submit a pull request. 

## Live Coding Session

Watch the live coding session where this project comes to life: [Rust Task Manager Live Coding](https://youtu.be/FfmfWk_kOA4)

