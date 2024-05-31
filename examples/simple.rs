use futures::stream::{self, StreamExt};
use stream_guard::GuardStreamExt;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut s = stream::iter(0..3).guard(|| println!("Dropped!"));
    while let Some(i) = s.next().await {
        println!("{}", i);
    }
}
