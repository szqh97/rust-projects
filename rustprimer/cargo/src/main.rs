extern crate url;

use url::{Url, ParseError, Host};

fn main() {
    let tmpurl = "mysql://192.168.1.100:3306";
    let issue_list_url = Url::parse(
        //"https://github.com/rust-lang/rust/issue?lables=E-easy&state=open"
        "mysql://192.168.1.100:3306"
    ).unwrap();

    println!("{:?}", issue_list_url.scheme());
    println!("{:?}", issue_list_url.username());
    println!("{:?}", issue_list_url.password());
    println!("{:?}", issue_list_url.host_str());
    println!("{:?}", issue_list_url.host().unwrap());
    println!("{:?}", issue_list_url.port());
    println!("{:?}", issue_list_url.path());
}
