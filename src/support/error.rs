pub fn refusal(code: &str) -> String {
    match code {
        "vpnBlocked" => "the connection looks like a virtual private network".to_string(),
        "badTask" => "the task is not available any more".to_string(),
        "HUMAN_CHECK_REQUIRED" => "the server asks for a human check on this account".to_string(),
        other => format!("the server refused the request with {other}"),
    }
}
