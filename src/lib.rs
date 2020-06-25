use reqwest;
use std::net::Ipv4Addr;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

pub async fn get_ip() -> Result<Ipv4Addr> {
    let ip: Vec<u8> = reqwest::get("https://api.ipify.org")
        .await?
        .text()
        .await?
        .split(".")
        .map(|x| x.parse::<u8>().unwrap())
        .collect();
    Ok(Ipv4Addr::new(ip[0], ip[1], ip[2], ip[3]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn ensure_public() {
        // Simple test to determine the IP returned by IPify is not private.
        let ip = get_ip().await;
        assert_eq!(ip.unwrap().is_private(), false);
    }
}
