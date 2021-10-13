use env_logger::{self, Env};
use log::{error, info, warn};
use rin::Router;



fn main(){
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    
    let r = Router::new();
    let admin = r.group("admin");
    {
        admin.get("index", |c|{
            
        })
    }
    r.run("127.0.0.1:80");
}

