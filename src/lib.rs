pub mod metal;

#[cfg(test)]
mod tests {

    use metal::Metal;

    #[test]
    fn bind_port() {
        let server = Metal;
        server.listen(String::from("127.0.0.1:8080"));
    }
}