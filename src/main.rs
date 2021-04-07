use redis::Commands;
use redis::cluster::ClusterClient;
fn main(){
    let nodes = vec!["redis://localhost:7001/", "redis://localhost:7002/", "redis://localhost:7003/", "redis://localhost:7004/", "redis://localhost:7005/", "redis://localhost:7006/"];
    let client = ClusterClient::open(nodes).unwrap();
    let mut connection = client.get_connection().unwrap();

    let _: () = connection.set("test", "test_data").unwrap();
    let rv: String = connection.get("test").unwrap();
    println!("___{}___", rv);
}
