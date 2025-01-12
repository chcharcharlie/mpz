//! Mocked DEAP VMs for testing

use mpz_ot::mock::{mock_ot_shared_pair, MockSharedOTReceiver, MockSharedOTSender};
use utils_aio::mux::{mock::MockMuxChannelFactory, MuxChannel};

use crate::config::Role;

use super::{vm::DEAPVm, DEAPThread};

/// Mock DEAP Leader VM.
pub type MockLeader = DEAPVm<MockSharedOTSender, MockSharedOTReceiver>;
/// Mock DEAP Leader thread.
pub type MockLeaderThread = DEAPThread<MockSharedOTSender, MockSharedOTReceiver>;
/// Mock DEAP Follower VM.
pub type MockFollower = DEAPVm<MockSharedOTSender, MockSharedOTReceiver>;
/// Mock DEAP Follower thread.
pub type MockFollowerThread = DEAPThread<MockSharedOTSender, MockSharedOTReceiver>;

/// Create a pair of mocked DEAP VMs
pub async fn create_mock_deap_vm(
    id: &str,
) -> (
    DEAPVm<MockSharedOTSender, MockSharedOTReceiver>,
    DEAPVm<MockSharedOTSender, MockSharedOTReceiver>,
) {
    let mut mux_factory = MockMuxChannelFactory::new();
    let (leader_ot_send, follower_ot_recv) = mock_ot_shared_pair();
    let (follower_ot_send, leader_ot_recv) = mock_ot_shared_pair();

    let leader_channel = mux_factory.get_channel(id).await.unwrap();
    let follower_channel = mux_factory.get_channel(id).await.unwrap();

    let leader = DEAPVm::new(
        id,
        Role::Leader,
        [42u8; 32],
        leader_channel,
        Box::new(mux_factory.clone()),
        leader_ot_send,
        leader_ot_recv,
    );

    let follower = DEAPVm::new(
        id,
        Role::Follower,
        [69u8; 32],
        follower_channel,
        Box::new(mux_factory),
        follower_ot_send,
        follower_ot_recv,
    );

    (leader, follower)
}
