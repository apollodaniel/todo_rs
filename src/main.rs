use todoapp::todo::*;

fn main() {

    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);

    if args.len() >= 2{
        process(args[0].as_str(), args[1..].join(" ").as_str());
    }else if args.len() == 1 {
        if args[0].eq("list"){
            list();
        }else{
            println!("{}", MISSING_NAME_MSG);
        }
    }else{
        println!("{}", MISSING_COMMAND_MSG);
    }

}
