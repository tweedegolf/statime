use std::sync::mpsc;

use nix::sys::socket::{InetAddr, SockAddr};
use statime::{
    datastructures::common::ClockIdentity,
    network::linux::{get_clock_id, LinuxRuntime},
    ptp_instance::{Config, PtpInstance},
};

fn main() {
    let (tx, rx) = mpsc::channel();
    let network_runtime = LinuxRuntime::new(tx);

    let config = Config {
        identity: ClockIdentity(get_clock_id().expect("Could not get clock identity")),
        sdo: 0,
        domain: 0,
        interface: SockAddr::Inet(InetAddr::new(
            nix::sys::socket::IpAddr::new_v4(0, 0, 0, 0),
            0,
        )),
    };

    let mut instance = PtpInstance::new(config, network_runtime);

    loop {
        let packet = rx.recv().expect("Could not get further network packets");
        instance.handle_network(packet);
    }
}
