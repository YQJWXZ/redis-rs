# Redis-rs


Using Tokio and Rust's futures to create an asynchronous Redis client.


- RespFrame
    - 使用RespEncoder/RespDecoder trait统一行为
    - unit test
- Command
    - 使用CommandExecutor trait统一行为
    - 使用TryFromn从RespFrame获得Command
    - unit test
- TcpListener / TcpStream
    - accept() -> tokio::spawn 开启新任务处理TcpStream
    - Framed: 把TcpStream按帧处理(桥接网络层和应用层)
    - CodeC: 实现Encoder/Decoder
