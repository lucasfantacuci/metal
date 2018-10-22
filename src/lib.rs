pub mod metal;

#[cfg(test)]
mod tests {

    use metal::Metal;

    #[test]
    fn bind_port() {
        Metal::listen(String::from("127.0.0.1:8080"));
    }
}