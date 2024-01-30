use todoapp::todo::*;

fn main() {

    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);

    if args.len() >= 2{
        
    }else if args.len() == 1 {
        println!("{}", MISSING_NAME_MSG);
    }else{
        println!("{}", MISSING_COMMAND_MSG);
    }

}
