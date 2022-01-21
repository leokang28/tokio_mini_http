use tokio::{net::TcpStream, io::{AsyncReadExt, AsyncWriteExt}};
use std::{
    str::from_utf8,
};

const CRLF: &str = "\n\r";

#[derive(Debug)]
pub enum FileIoStat {
    FileNotExist,
    IsDir,
    Ok,
}

pub async fn handle_tcp_to_http(mut stream: TcpStream) -> () {
    let mut buffer: [u8; 1024] = [0; 1024];
    stream.read(&mut buffer).await.unwrap();
    println!("{}", String::from_utf8_lossy(&buffer[..]));
    let mut response_body: String = "".to_string();

    // TODO: 解析http报文，取出path拼装文件路径
    if _is_index(from_utf8(&buffer).unwrap()) {
        response_body = _read_file("./static/index.html").await.unwrap();
    } else {
        response_body = _handle_404();
    }
    let res = _gen_response(response_body, 200);
    stream.write(&res.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}

fn _gen_response(body: String, status: u16) -> String {
    let status= format!("{}{}",status, CRLF);
    let content_length = format!("Content-Length: {}{}", body.len(), CRLF);
    let content_type = format!("Conent-Type: text/html;charset=utf8{}", CRLF);
    format!("HTTP/1.1 {}{}{}{}{}", status, content_length, content_type, CRLF, body)
}

fn _is_index(path: &str) -> bool {
    path.starts_with("GET /index ")
}

fn _handle_404() -> String {
    "<h1>404 Not Found</h1>".to_string()
}

async fn _read_file(path: &str) -> Option<String> {
    match _valid_file(path).await {
        FileIoStat::Ok => {
            let mut fd = tokio::fs::File::open(path).await.unwrap();
            let mut file = format!("");
            fd.read_to_string(&mut file).await.unwrap();
            Some(file)
        },
        FileIoStat::FileNotExist => {
            Some(_handle_404())
        },
        _ => None
    }
}

async fn _valid_file(path: &str) -> FileIoStat {
    match tokio::fs::metadata(path).await {
        Ok(metadata) => {
            if metadata.is_dir() {
                return FileIoStat::IsDir;
            }
        },
        Err(e) => {
            println!("{:?}", e);
            return FileIoStat::FileNotExist;
        }
    };
    FileIoStat::Ok
}