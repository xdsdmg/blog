mod config;
mod handler;
mod http;
mod thread_pool;

fn main() {
    let mut http = http::Http::new(&config::CONFIG.server.host, config::CONFIG.server.port, 4);

    http.register("/api/blog", "GET", handler::file);
    http.register("/api/ping", "GET", handler::ping);

    http.spin();
}
