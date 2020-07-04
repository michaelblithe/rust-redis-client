pub fn encode_command(cmds: &Vec<&str>) -> String {
    let mut cmd_string = "".to_owned();
    cmd_string.push_str(&format!("*{}\r\n", cmds.len()));
    for cmd in cmds {
        let bytes = cmd.as_bytes();
        cmd_string.push_str(&format!("${}\r\n{}\r\n", bytes.len(), cmd));
    }
    cmd_string
}
