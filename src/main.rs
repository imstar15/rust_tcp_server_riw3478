use std::{io::{BufRead, BufReader, BufWriter, Write}, net::{TcpListener, TcpStream}};

fn main() {
    // 侦听IP和端口
    let listener = TcpListener::bind("127.0.0.1:9123").unwrap();
    // 取回接入的client stream
    for stream in listener.incoming() {
        // 匹配stream
        match stream {
            // 错误则抛异常
            Err(e) => print!("Accept err {}", e),
            // 正常则进入处理参数
            Ok(stream) => {
                handle_client(stream);
            }
        }
    }
}

fn handle_client(stream: TcpStream) {
    // 读取输入并打印
    let mut reader = BufReader::new(&stream);
    let mut response = String::new();
    reader.read_line(&mut response).expect("Could not read");
    println!("{}", response);

    // 打印输出
    let mut writer = BufWriter::new(&stream);
    writer.write_all("Red\n".as_bytes()).expect("could not write");
    writer.flush().expect("could not flush");
}