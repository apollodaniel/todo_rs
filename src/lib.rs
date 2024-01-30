pub mod todo{
    use rusqlite::Connection;

    const INVALID_COMMAND_MSG: &str = include_str!("invalid_command.txt");

    pub fn process(command: &str, todo: &str){
        match command {
            "add" => execute_command(TodoCommand::Add(todo)),
            "remove" => execute_command(TodoCommand::Remove(todo)),
            "mark" => execute_command(TodoCommand::Mark(todo)),
            "unmark" => execute_command(TodoCommand::Unmark(todo)),
            _=>{
                println!("{}", INVALID_COMMAND_MSG.replace("<command>", format!("'{}'",command).as_str()));
            }
        }
    }

    pub fn connect_db() -> Option<Connection>{
        let app_folder = simple_home_dir::home_dir().unwrap().join("/.todo_rs");
        let db_location = app_folder.join("/todo.db");
        
        let result = std::fs::create_dir(app_folder);
        match result {
            Ok(_) =>{
                match rusqlite::Connection::open(db_location) {
                    Ok(con)=>{
                        match con.execute("CREATE TABLE IF NOT EXISTS todo(id INTEGER PRIMARY KEY AUTOINCREMENT, content TEXT, marked BOOL)", []) {
                            Ok(_) => Some(con),
                            Err(_) => None
                        }                        
                    }
                    Err(_) => None
                }
            },
            Err(_) => {
                println!("There was an error when trying to create the database directory");
                None
            }
        }
    }

    fn execute_command(command: TodoCommand){
        let connection = connect_db();
        match connection {
            Some(con)=>{
                match command {
                    TodoCommand::Add(todo)=>{
                        let result = con.execute(format!("INSERT INTO todo (content, marked) values ('{}', false)", todo).as_str(), []);
                        if let Err(e) = result{
                            println!("Error adding new todo.\n{}",e);
                        } 
                    },
                    TodoCommand::Remove(todo)=>{
                        
                    },
                    TodoCommand::Mark(todo)=>{
        
                    },
                    TodoCommand::Unmark(todo)=>{
        
                    },
                } 
            },
            None=>{
                println!("Error getting db connection")
            }
        }
    }

    enum TodoCommand<'a>{
        Add(&'a str),
        Remove(&'a str),
        Mark(&'a str),
        Unmark(&'a str)
    }

    struct Todo{
        id: u32,
        content: String,
        marked: bool
    }
}