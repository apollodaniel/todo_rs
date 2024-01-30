use todoapp::todo::*;

fn main() {

    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);

    if args.len() >= 2{
        let todos_input: Vec<&String> = args[1..].iter().collect();
        process(args[0].as_str(), todos_input);
    }else if args.len() == 1 {
        if args[0].eq("list"){
            print_list();
        }else{
            println!("{}", MISSING_NAME_MSG);
        }
    }else{
        println!("{}", MISSING_COMMAND_MSG);
    }
}

