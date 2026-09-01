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
