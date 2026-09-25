#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    // création d'instances de chaque variante de IpAddrKind
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);
}

fn route(ip_kind: IpAddrKind) {
    dbg!(ip_kind);
}
