use rand::RngCore;
use tokio::task::yield_now;

#[tokio::main]
async fn main() {
    let block = Box::pin(async { // 0x7fff239d3760
        let arc = rand::thread_rng().next_u32(); // 0x564d5467ad20
        let _rc = rand::thread_rng().next_u32(); // 0x564d5467ad24

        {
            let mut _irc = 64u32; // 0x7fff239d3504
            _irc = _rc;
        }

        println!("Rc: {}", _rc);

        yield_now().await;

        {
            let mut _irc = 32u32; // 0x7fff239d3554
            _irc = 32;
        }
        println!("Arc: {}", arc);
    });

    block.await;
}
