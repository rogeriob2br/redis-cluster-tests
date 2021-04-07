use redis::Commands;
use redis::cluster::ClusterClient;
fn main(){
    let nodes = vec!["redis://10.128.0.64:6379/", "redis://10.129.0.53:6379/", "redis://10.130.0.81:6379/", "redis://10.128.0.65:6379/", "redis://10.129.0.54:6379/", "redis://10.130.0.82:6379/"];
    let client = ClusterClient::open(nodes).unwrap();
    let mut connection = client.get_connection().unwrap();

    let _: () = connection.set("test", "test_data").unwrap();
    let rv: String = connection.get("test").unwrap();
    println!("___{}___", rv);
}
