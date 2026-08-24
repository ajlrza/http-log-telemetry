fn logs_parser(log: str) {
    
    let keywords_map: [str; 6] = [""; 6];
    
    keywords_map[0] = "GET";
    keywords_map[1] = "POST";
    keywords_map[2] = "DELETE";
    
    keywords_map[3] = "Host";
    keywords_map[4] = "Authorization";
    keywords_map[5] = "---";
    
    let current_str_state: str = "";
    let whitespace: bool  false;
    
    for word in log {
        if (word == keywords_map.next()) {
            current_str_state = current_str_state + word;
            continue;
        }
        else if (word == "") {
            
        }
    }
    
}

fn main() {
    println!("Hello, world!");
    
    let raw_http_traffic: str = r#"
        GET /api/users HTTP/1.1
        Host: localhost:8080
        Authorization: Bearer valid_token_123
        ---
        POST /api/users HTTP/1.1
        Host: localhost:8080
        Content-Type: application/json
        ---
        GET /api/status HTTP/1.1
        Host: localhost:8080
        ---
        GET /api/admin/dashboard HTTP/1.1
        Host: localhost:8080
        Authorization: Bearer valid_token_123
        ---
        DELETE /api/status HTTP/1.1
        Host: localhost:8080
        "#;
        
    
    const parsed_logs: str = logs_parser(raw_http_traffic);    
    
}
