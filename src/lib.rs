pub mod todo{

    use rusqlite::Connection;

    pub const INVALID_COMMAND_MSG: &str = include_str!("invalid_command.txt");
    pub const MISSING_COMMAND_MSG: &str = include_str!("missing_command.txt");
    pub const MISSING_NAME_MSG: &str = include_str!("missing_name.txt");

    pub fn print_list(){
        let result = list();
        if let Some(todos) = result{
            for (index,todo) in todos.iter().enumerate() {
                println!("{} - {}", index+1, todo);
            }
        }else{
            println!("There's no todo saved yet, try adding something using:");
            println!("todo_rs add [name]");
        }
    }
    pub fn list()->Option<Vec<Todo>>{
        let connection = connect_db();

        match connection {
            Some(con) => {
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
                            let todo_list: Vec<Todo> = todos
                                .filter(|f| f.is_ok())
                                .map(|f|f.unwrap()).collect();

                            if todo_list.len() > 0{
                                Some(todo_list)
                            }else{
                                None
                            }                          
                        }else{
                            None
                        }
                    },
                    Err(_)=>{
                        None
                    }
                }
            },
            None=>{
                None
            }
        }
        
    }
    pub fn process(command: &str, todo: Vec<&String>){
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

        
        fn get_where_query(todo: &str) -> String{
            let todo_list = list().unwrap_or(vec![]);
            let id = todo.parse::<usize>();
                            
            if id.is_ok() {
                // numeric - Remove by index
                format!("rowid = {}", todo_list[id.unwrap()-1].id)
            }else{
                // content - remove by
                format!("content IS '{}'",todo)
            }
        }

        match connection {
            Some(con)=>{
                match command {
                    TodoCommand::Add(todos)=>{
                        for todo in todos{
                            let result = con.execute(format!("INSERT INTO todo (content, marked) values ('{}', false)", todo).as_str(), []);
                            if let Err(e) = result{
                                println!("Error adding new todo.\n{}",e);
                            }
                        }
                    },
                    TodoCommand::Remove(mut todos)=>{
                        todos.sort_by(|a,b|{
                            b.cmp(a)
                        });
                        for todo in todos{
                            let where_query = get_where_query(todo);

                            let result = con.execute(format!("DELETE from todo WHERE {}", where_query).as_str(), []);
    
                            if let Err(e) = result{
                                println!("Error removing {} todo.\n{}",todo,e);
                            }
                        }
                    },
                    TodoCommand::Mark(todos)=>{
                        for todo in todos{
                            let where_query = get_where_query(todo);
    
                            let result = con.execute(format!("UPDATE todo SET marked=true WHERE {}", where_query).as_str(), []);
                            if let Err(e) = result{
                                println!("Error marking {} as done.\n{}",todo,e);
                            }
                        }

                    },
                    TodoCommand::Unmark(todos)=>{
                        for todo in todos{
                        
                            let where_query = get_where_query(todo);

                            let result = con.execute(format!("UPDATE todo SET marked=false WHERE {}",where_query).as_str(), []);
                            if let Err(e) = result{
                                println!("Error unmarking {}.\n{}",todo,e);
                            }
                        }
                    },
                } 
            },
            None=>{
                println!("Error getting db connection")
            }
        }

        print_list();

    }

    enum TodoCommand<'a>{
        Add(Vec<&'a String>),
        Remove(Vec<&'a String>),
        Mark(Vec<&'a String>),
        Unmark(Vec<&'a String>),
    }

    pub struct Todo{
        id: u32,
        content: String,
        marked: bool
    }

    impl std::fmt::Display for Todo {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if self.marked{
                write!(f, "\x1B[9m{}\x1B[0m", self.content)
            }else{
                write!(f, "{}", self.content)
            }
        }
    }

}