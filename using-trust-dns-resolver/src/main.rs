use std::env;

use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts},
    Resolver,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Please provide a name to query!");
        std::process::exit(1);
    }

    let query = format!("{}.", args[1]);

    println!(" --- default(google) ---");
    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();
    let response = resolver.lookup_ip(&query).unwrap();
    for r in response.iter() {
        println!("{:?}", r);
    }

    println!(" --- system( /etc/resolv.conf ) ---");
    let resolver = Resolver::from_system_conf().unwrap();
    let response = resolver.lookup_ip(&query).unwrap();
    for r in response.iter() {
        println!("{:?}", r);
    }

    println!(" --- NS --");
    let response = resolver
        .lookup(&query, trust_dns_resolver::proto::rr::RecordType::NS)
        .unwrap();
    for r in response.iter() {
        println!("{:?}", r);
    }
}
