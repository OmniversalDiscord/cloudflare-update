use serde::{Serialize, Deserialize};
use reqwest::header::AUTHORIZATION;
use termion::color;
use std::env;
use std::process::Command;

#[derive(Serialize)]
struct DNSRecord {
    r#type: String,
    name: String,
    content: String,
    ttl: usize, 
    proxied: bool
}
#[derive(Deserialize, Debug)]
struct CloudflareError {
    code: usize,
    message: String
}

#[derive(Deserialize, Debug)]
struct CloudflareResult {
    id: String
}

#[derive(Deserialize, Debug)]
struct CloudflareResponseWithResults {
    success: bool,
    errors: Vec<CloudflareError>,
    result: Vec<CloudflareResult>
}

// Needed 
#[derive(Deserialize, Debug)]
struct CloudflareResponse {
    success: bool,
    errors: Vec<CloudflareError>,
}

fn show_response(res: &CloudflareResponse, on_success: String, on_error: String) {
    if res.success {
        println!("{}{}{}", color::Fg(color::Green), on_success, color::Fg(color::Reset));
    } else {
        println!("{}{}{}", color::Fg(color::Red), on_error, color::Fg(color::Reset));
        for err in &res.errors {
            println!("Code {}: {}", err.code, err.message);
        }
    }
}

fn create_record(subdomain: String, token: &String, ip_addr: &String) {
    let client = reqwest::blocking::Client::new();

    let record = DNSRecord {
        r#type: "A".to_string(),
        name: subdomain.clone(),
        content: ip_addr.to_string(),
        ttl: 1,
        proxied: true
    };

    let res: CloudflareResponse = client.post("https://api.cloudflare.com/client/v4/zones/11866a68234f4f4fe5500a90080bb23d/dns_records")
                                  .header(AUTHORIZATION, token)
                                  .json(&record)
                                  .send().unwrap()
                                  .json::<CloudflareResponse>().unwrap();

    show_response(&res, format!("{}.omniversal.co was succesfully added to the DNS records", subdomain), format!("An error occurred adding {}.omniversal.co", subdomain))    
}

fn delete_record(subdomain: String, token: &String) {
    let client = reqwest::blocking::Client::new();

    let find_res: CloudflareResponseWithResults = client.get("https://api.cloudflare.com/client/v4/zones/11866a68234f4f4fe5500a90080bb23d/dns_records")
                                             .header(AUTHORIZATION, token)
                                             .query(&[("name", format!("{}.omniversal.co", subdomain))])
                                             .send().unwrap()
                                             .json::<CloudflareResponseWithResults>().unwrap();
    
    if find_res.result.len() > 0 {
        let id = find_res.result[0].id.clone();
        let url = format!("https://api.cloudflare.com/client/v4/zones/11866a68234f4f4fe5500a90080bb23d/dns_records/{}", id);

        let delete_res: CloudflareResponse = client.delete(&url)
                                                   .header(AUTHORIZATION, token)
                                                   .send().unwrap()
                                                   .json::<CloudflareResponse>().unwrap();

        show_response(&delete_res, format!("{}.omniversal.co was succesfully removed from the DNS records", subdomain), format!("An error occurred removing {}.omniversal.co", subdomain))    
    } else if find_res.success {
        println!("{}{}.omniversal.co was not found in the DNS records{}", color::Fg(color::Red), subdomain, color::Fg(color::Reset))
    } else {
        println!("{}An error occured whilst searching for {}.omniversal.co{}", color::Fg(color::Red), subdomain, color::Fg(color::Reset));
        for err in find_res.errors {
            println!("Code {}: {}", err.code, err.message);
        }
    }
}

fn main() {
    let token = env::var("CLOUDFLARE_TOKEN").expect("CLOUDFLARE_TOKEN was not set");
    let get_ip = Command::new("dig")
                                      .arg("@resolver1.opendns.com")
                                      .arg("ANY")
                                      .arg("myip.opendns.com")
                                      .arg("+short")
                                      .arg("-4")
                                      .output().expect("Failed to execute public IP search");
    
    let ip_addr = String::from_utf8(get_ip.stdout).unwrap();

    println!("{}", token);
    println!("{}", ip_addr);
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("invalid syntax: cloudflare-update [add|remove] <subdomain>")
    } else {
        match &*args[1] {
            "add" => create_record(args[2].clone(), &token, &ip_addr),
            "remove" => delete_record(args[2].clone(), &token),
            _ => println!("invalid method: cloudflare-update [add|remove] <subdomain>")
        }
    }
}