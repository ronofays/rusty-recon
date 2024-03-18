use subdomain_lib;

fn main() {
    let out = subdomain_lib::crtsh::query_crtsh("indeed.com").unwrap();
    println!("{:?}", out);
}