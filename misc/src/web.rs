use afire::Request;

pub fn real_ip(req: &Request) -> &str {
    let real = req.address.split_once(':').unwrap().0;
    if real != "127.0.0.1" {
        return real;
    }

    req.headers
        .iter()
        .find(|x| x.name == "X-Forwarded-For")
        .map(|x| x.value.split_once(',').unwrap().0)
        .unwrap_or(real)
}
