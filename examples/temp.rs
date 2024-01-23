use xt_oss::oss::Request;

#[tokio::main]
async fn main() {
	let request = Request::new();
	println!("{:#?}", request);
}