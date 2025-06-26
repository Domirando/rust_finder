use std::io;

fn main(){
    enum Request {
        Query,
        File,
        CaseSensitivity,
    }
    fn request_messages(req: Request) -> String {
        match req {
            Request::Query => "Please enter a word/phrase you are looing for".to_string(),
            Request::File => "Please provide a file location".to_string(),
            Request::CaseSensitivity => "Do you want the search be case sensitive? (yes/no)".to_string()
        }
    }
    let mut search = input(request_messages(Request::File));
    let mut query = input(request_messages(Request::Query));
    let mut case_sensitivity = input(request_messages(Request::CaseSensitivity));
    println!("search: {}, query: {}, case sensitivity: {}", search, query, case_sensitivity);
    fn input(request: String) -> String{
        println!("{}", request); 
        let mut response = String::new();
        io::stdin().read_line(&mut response).expect("failed to read line");
        let response = response.trim();
        response = (request.contains("file") && response.is_empty()) => "poem.txt"; 
        response.to_string()
    }
    
    let config = Config::new(search, file.to_string(), case_sensitivity);
    if let Err(e) = func::run(config) {
        println!("application error: {}", e);
        process::exit(1);
    } 
}
