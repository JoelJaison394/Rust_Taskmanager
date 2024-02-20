struct Task {
    id: u32,
    title: String,
    description: String,
    completed:bool
}

impl Task {
    fn add_task(tasks: &mut Vec<Task>){
        println!("Enter the title of the task:");
        let title: String = getinput("Title:");
        println!("Enter the description for the task:");
        let description: String = getinput("Description: ");

        let id: u32 = tasks.len() as u32 + 1;

        let new_task = Task {
            id,
            title,
            description,
            completed: false
        };

        tasks.push(new_task);

        println!("Task added successfully");

    }

    fn list_tasks( tasks: &Vec<Task>){
        if tasks.is_empty(){
            println!("No tasks available");
        }else{
            println!("All tasks are:");
            for task in tasks {
                println!("ID: {}\nTitle: {}\nDescription: {}\nCompleted: {}\n",
                        task.id, task.title, task.description , task.completed);
            }
        }
    }

    fn edit_task( tasks: &mut Vec<Task>, task_id: u32){
        if let Some(task) = tasks.iter_mut().find(|t| t.id == task_id){
            println!("Editing the task with ID{}",task_id);
            let new_title: String=getinput("Enter the new title");
            task.title = new_title;
            let new_description: String = getinput("Enter the new description");
            task.description = new_description;
            println!("The task is edited sucessfully");
        }else{
            println!("The task with ID{} not found",task_id);
        }
    }

    fn delete_task(tasks: &mut Vec<Task>, task_id: u32 ){
        if let Some(index) = tasks.iter().position(|task| task.id == task_id) {
            tasks.remove(index);
            println!("The task deleted successfully")
        }else{
            println!("The task with ID{} not found",task_id)
        }
    }
    fn mark_task(tasks: &mut Vec<Task>, task_id: u32){
        if let Some(task) = tasks.iter_mut().find(|t| t.id == task_id) {
            task.completed = true;
            println!("Task is marked as completed")
        }else{
            println!("The task with ID{} not found",task_id)
        }
    }
}



fn main(){
    let mut tasks: Vec<Task> = Vec::new();

    loop {
        println!("1. Add a task");
        println!("2. List all tasks");
        println!("3. Edit the task");
        println!("4. Delete the task");
        println!("5. Mark the task complete");
        println!("6. Exit from the program");

        let choice: u32  = getinput("Enter the choice").parse().expect("Please enter a valid number");

        match choice  {
            1 => {
                Task::add_task( &mut tasks);
            }
            2 =>{
                Task::list_tasks(&tasks);
            }
            3=>{
                let task_id_input: String = getinput("Enter the task ID");
                let task_id: u32 = task_id_input.trim().parse().unwrap_or(0);

                Task::edit_task( &mut tasks , task_id);
            }
            4=>{
                let task_id_input: String = getinput("Enter the task ID");
                let task_id: u32 = task_id_input.trim().parse().unwrap_or(0);

                Task::delete_task(&mut tasks , task_id);

            }
            5=>{
                let task_id_input: String = getinput("Enter the task ID");
                let task_id: u32 = task_id_input.trim().parse().unwrap_or(0);

                Task::mark_task(&mut tasks , task_id);
            }
            6=>{
                println!("Exiting the program");
                break;
            }

            _ => {
                println!("Invalid choice ,  please enter a valid option")
            }
        }
    }

}

fn getinput(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}
