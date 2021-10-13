use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use log::{error, info};
use std::str::FromStr;
use std::sync::Arc;
use std::{convert::Infallible, net::SocketAddr};

type HandlerFunc = fn(c: Context); // 处理器

#[derive(Debug, Default)]
pub struct Context {
    req: Request<Body>,
}

impl Context {
    pub fn new() -> Self {
        Context::default()
    }
    /// 调用下一个handler
    pub fn next(&self) {}
}

/// cookie相关的操作
impl Context {
    /// 获取cookie
    pub fn cookie(&self, name: &str) -> Option<String> {
        None
    }

    /// 设置cookie
    pub fn set_cookie(&self, name: &str, value: &str) {}
}

/// 获取客户端数据
impl Context {
    /// 从path中获取参数
    pub fn param<T>(&self, name: &str) -> Option<T>
    where
        T: FromStr,
    {
        name.parse().ok()
    }

    pub fn query<T>(&self, name: &str) -> Option<T>
    where
        T: FromStr,
    {
        name.parse().ok()
    }

    pub fn form<T>(&self, name: &str) -> Option<T>
    where
        T: FromStr,
    {
        name.parse().ok()
    }

    /// 将提交的数据解析成json对象
    pub fn bind<T>(&self) -> Result<T, String>
    where
        T: Default,
    {
        let data: T = T::default();
        Ok(data)
    }

    /// 获取上传文件
    pub fn file(&self) -> Result<String, String> {
        Err("test".into())
    }

    /// 保存上传的文件到指定路径
    pub fn save_uploaded_file(file: &str, des: &str) -> Result<bool, String> {
        Ok(true)
    }
}

/// 返回数据
impl Context {
    /// 返回字符串到客户端
    pub fn string(&self, code: u32, data: &str) {}

    /// 返回json格式数据
    pub fn json(&self, code: u32, data: &str) {}

    /// 返回jsonp格式数据
    pub fn jsonp(&self, code: u32, data: &str) {}

    /// 重定向
    pub fn redirect(&self, code: u32, url: &str) {}
}

#[derive(Debug, Default)]
pub struct Router {}

impl Router {
    pub fn new() -> Self {
        Router::default()
    }

    async fn shutdown_signal() {
        // Wait for the CTRL+C signal
        tokio::signal::ctrl_c()
            .await
            .expect("安装 CTRL+C 处理器失败");
    }

    /// 静态资源目录
    pub fn static_dir(path: &str, dir: &str) {}

    /// 静态文件
    pub fn static_file(uri: &str, filepath: &str) {}

    /// 创建一个路由组
    pub fn group(&self, path: &str) -> RouterGroup {
        RouterGroup {}
    }

    async fn handle(
        context: Arc<Router>,
        addr: SocketAddr,
        req: Request<Body>,
    ) -> Result<Response<Body>, Infallible> {
        Ok(Response::new(Body::from("Hello World")))
    }

    /// 启动服务器
    pub fn run(self, host: &str) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let addr = match SocketAddr::from_str(host) {
                Ok(v) => v,
                Err(_) => {
                    error!("解析地址失败，请确认格式为(ip:port),你的地址是:{}", host);
                    return;
                }
            };

            // 路由表信息
            let router: Arc<Router> = Arc::new(self);

            // 创建service处理每个请求
            let make_service = make_service_fn(move |conn: &AddrStream| {
                // 客户端地址信息
                let addr = conn.remote_addr();

                // 每个请求都克隆一个路由信息，传入给处理函数
                let router = router.clone();
                let service = service_fn(move |req| Self::handle(router.clone(), addr, req));
                async move { Ok::<_, Infallible>(service) }
            });
            let server = Server::bind(&addr).serve(make_service);

            let graceful = server.with_graceful_shutdown(Self::shutdown_signal());

            info!("启动成功: {}", host);

            if let Err(e) = graceful.await {
                eprintln!("server error: {}", e);
            }
        });
    }

    /// 启动tls服务器
    pub fn run_tls(host: &str, pem: &str, key: &str) {}

    /// 添加中间件
    pub fn middleware(&self, handler: HandlerFunc) {}
}

pub struct RouterGroup {}

impl RouterGroup {
    pub fn middleware(&self, handler: HandlerFunc) {}

    pub fn post(&self, path: &str, handler: HandlerFunc) {}

    pub fn get(&self, path: &str, handler: HandlerFunc) {}

    pub fn put(&self, path: &str, handler: HandlerFunc) {}

    pub fn delete(&self, path: &str, handler: HandlerFunc) {}

    pub fn options(&self, path: &str, handler: HandlerFunc) {}
}

#[cfg(test)]
mod tests {
    use crate::Router;

    #[test]
    fn test_run() {
        let r = Router::new();

        r.run("127.0.0.1:5555");
    }
}
