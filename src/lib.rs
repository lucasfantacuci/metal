pub mod metal;

#[cfg(test)]
mod tests {

    use metal::Server;

    #[test]
    fn bind_port() {
        Server::listen(String::from("127.0.0.1:8080"));
    }
}