#[test]
fn req() {
    let body = ureq::get("https://storage.googleapis.com/golang/?prefix=go&marker=")
        .call()
        .unwrap()
        .into_string()
        .unwrap();

    println!("{body}")
}

#[test]
fn req2() {
    let body = ureq::get("https://storage.googleapis.com/golang/?prefix=go&marker=")
        .call()
        .unwrap();

    let headers: Vec<_> = body
        .headers_names()
        .into_iter()
        .flat_map(|x| body.header(&x).map(|v| (x, v)))
        .collect();

    for x in headers {
        println!("name {} value {}", x.0, x.1);
    }
}
