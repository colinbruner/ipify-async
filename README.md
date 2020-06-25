# ipify-async
A simple asynchronous library for obtaining your public IP address within Rust code.

# Example
The following is a simple example using the library while leveraging the tokio runtime.

```
use ipify_async;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

#[tokio::main]
async fn main() -> Result<()> {
    let ip = ipify_async::get_ip().await.unwrap().to_string();
    println!("{:?}", ip);
    Ok(())
}
```


