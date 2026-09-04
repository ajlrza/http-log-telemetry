pub fn logs_parser(log: &str) {
    
    let keywords_map: [&str; 6] = [""; 6];
    
    keywords_map[0] = "GET";
    keywords_map[1] = "POST";
    keywords_map[2] = "DELETE";
    
    keywords_map[3] = "Host";
    keywords_map[4] = "Authorization";
    keywords_map[5] = "---";
    
    let current_str_state: &str = "";
    let whitespace: bool = false;
    
    for word in log {
        if (word == keywords_map.next()) {
            current_str_state = current_str_state + word;
            continue;
        }
        else if (word == "") {
            
        }
    }
    
}
