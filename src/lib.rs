pub mod todo{

    use rusqlite::Connection;

    pub const INVALID_COMMAND_MSG: &str = include_str!("invalid_command.txt");
    pub const MISSING_COMMAND_MSG: &str = include_str!("missing_command.txt");
    pub const MISSING_NAME_MSG: &str = include_str!("missing_name.txt");


    pub fn list(){
        execute_command(TodoCommand::List)
    }
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
        let app_folder = simple_home_dir::home_dir().unwrap().join(".todo_rs");
        let db_location = app_folder.join("todo.db");

        if !app_folder.exists(){
            if let Err(e) = std::fs::create_dir(app_folder){
                println!("There was an error when trying to create the database directory\n{}",e.to_string());  
            }
        }
        if !db_location.exists(){
            if let Err(e) =  std::fs::File::create(&db_location){
                println!("There was an error when trying to create the database file\n{}",e.to_string());  
            }
        }
        
        match rusqlite::Connection::open(db_location) {
            Ok(con)=>{
                match con.execute("CREATE TABLE IF NOT EXISTS todo(id INTEGER PRIMARY KEY AUTOINCREMENT, content TEXT, marked BOOL)", []) {
                    Ok(_) => Some(con),
                    Err(_) => None
                }                        
            }
            Err(_) => None
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
                        list();
                    },
                    TodoCommand::Remove(todo)=>{
                        let result = con.execute(format!("DELETE from todo WHERE content IS '{}'", todo).as_str(), []);
                        if let Err(e) = result{
                            println!("Error removing {} todo.\n{}",todo,e);
                        }
                        list();
                    },
                    TodoCommand::Mark(todo)=>{
                        let result = con.execute(format!("UPDATE todo SET marked=true WHERE content is '{}'", todo).as_str(), []);
                        if let Err(e) = result{
                            println!("Error marking {} as done.\n{}",todo,e);
                        }
                        list();
                    },
                    TodoCommand::Unmark(todo)=>{
                        let result = con.execute(format!("UPDATE todo SET marked=false WHERE content is '{}'", todo).as_str(), []);
                        if let Err(e) = result{
                            println!("Error unmarking {}.\n{}",todo,e);
                        }
                        list();
                    },
                    TodoCommand::List=>{
                        let result = con.prepare("SELECT * from todo");
                        match result {
                            Ok(mut stmt) => {
                                let result =  stmt.query_map([], |f|{
                                    Ok(Todo{
                                        id: f.get(0)?,
                                        content: f.get(1)?,
                                        marked: f.get(2)?                                       
                                    })
                                });
                                if let Ok(todos) = result {
                                    let todo_list: Vec<Result<Todo, rusqlite::Error>> = todos.collect();
                                    for todo in todo_list {
                                        if todo.is_ok(){
                                            println!("{}", todo.unwrap());
                                        }
                                    }
                                }
                            },
                            Err(e)=>{
                                println!("Error when getting todo list.\n{}",e.to_string())
                            }
                        }
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
        Unmark(&'a str),
        List
    }

    struct Todo{
        id: u32,
        content: String,
        marked: bool
    }

    impl std::fmt::Display for Todo {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} - {} - {}",self.id, self.content, self.marked)
        }
    }

}