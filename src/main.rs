extern crate tokio_core;
extern crate tokio_uds;

fn main() {
    let core = tokio_core::reactor::Core::new().unwrap();
    let handle = core.handle();
    
    let (a, b) = tokio_uds::UnixStream::pair(&handle).unwrap();
    let cred_a = a.peer_cred().unwrap();
    let cred_b = b.peer_cred().unwrap();
    assert_eq!(cred_a, cred_b);

    println!("Credentials: {:?}", cred_a);
}
