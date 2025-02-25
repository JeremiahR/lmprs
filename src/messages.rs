use core::sync::atomic::Ordering;
use lightning::bitcoin::constants::ChainHash;
use lightning::bitcoin::secp256k1::PublicKey;
use lightning::bitcoin::Network;
use lightning::events::{MessageSendEvent, MessageSendEventsProvider};
use lightning::ln::msgs::{
    AcceptChannel, AcceptChannelV2, AnnouncementSignatures, ChannelAnnouncement,
    ChannelMessageHandler, ChannelReady, ChannelReestablish, ChannelUpdate, ClosingSigned,
    CommitmentSigned, ErrorMessage, FundingCreated, FundingSigned, Init, LightningError,
    NodeAnnouncement, OpenChannel, OpenChannelV2, QueryChannelRange, QueryShortChannelIds,
    ReplyChannelRange, ReplyShortChannelIdsEnd, RevokeAndACK, RoutingMessageHandler, Shutdown,
    Stfu, TxAbort, TxAckRbf, TxAddInput, TxAddOutput, TxComplete, TxInitRbf, TxRemoveInput,
    TxRemoveOutput, TxSignatures, UpdateAddHTLC, UpdateFailHTLC, UpdateFailMalformedHTLC,
    UpdateFee, UpdateFulfillHTLC,
};
use lightning::routing::gossip::NodeId;
use lightning::types::features::{InitFeatures, NodeFeatures};
use std::ops::Deref;
use std::sync::atomic::AtomicBool;
use std::sync::Mutex;
use tokio::sync::mpsc;

// initial code copied from https://github.com/lightningdevkit/rust-lightning/blob/c9fd3a5a1e28c8ae9036f47b1e36f0b5a2557004/lightning-net-tokio/src/lib.rs#L654
pub struct MsgHandler {
    expected_pubkey: PublicKey,
    pubkey_connected: mpsc::Sender<()>,
    pubkey_disconnected: mpsc::Sender<()>,
    disconnected_flag: AtomicBool,
    msg_events: Mutex<Vec<MessageSendEvent>>,
}
impl Deref for MsgHandler {
    type Target = mpsc::Sender<()>;

    fn deref(&self) -> &Self::Target {
        &self.pubkey_connected
    }
}
impl MsgHandler {
    // new function
    pub fn new(
        expected_pubkey: PublicKey,
        pubkey_connected: mpsc::Sender<()>,
        pubkey_disconnected: mpsc::Sender<()>,
        disconnected_flag: AtomicBool,
        msg_events: Mutex<Vec<MessageSendEvent>>,
    ) -> Self {
        MsgHandler {
            expected_pubkey,
            pubkey_connected,
            pubkey_disconnected,
            disconnected_flag,
            msg_events,
        }
    }
}
impl MessageSendEventsProvider for MsgHandler {
    fn get_and_clear_pending_msg_events(&self) -> Vec<MessageSendEvent> {
        self.msg_events.lock().unwrap().drain(..).collect() // TODO: fix
    }
}
impl RoutingMessageHandler for MsgHandler {
    fn handle_node_announcement(
        &self,
        _their_node_id: Option<PublicKey>,
        _msg: &NodeAnnouncement,
    ) -> Result<bool, LightningError> {
        Ok(false)
    }
    fn handle_channel_announcement(
        &self,
        _their_node_id: Option<PublicKey>,
        _msg: &ChannelAnnouncement,
    ) -> Result<bool, LightningError> {
        Ok(false)
    }
    fn handle_channel_update(
        &self,
        _their_node_id: Option<PublicKey>,
        _msg: &ChannelUpdate,
    ) -> Result<bool, LightningError> {
        Ok(false)
    }
    fn get_next_channel_announcement(
        &self,
        _starting_point: u64,
    ) -> Option<(
        ChannelAnnouncement,
        Option<ChannelUpdate>,
        Option<ChannelUpdate>,
    )> {
        None
    }
    fn get_next_node_announcement(
        &self,
        _starting_point: Option<&NodeId>,
    ) -> Option<NodeAnnouncement> {
        None
    }
    fn peer_connected(
        &self,
        _their_node_id: PublicKey,
        _init_msg: &Init,
        _inbound: bool,
    ) -> Result<(), ()> {
        Ok(())
    }
    fn handle_reply_channel_range(
        &self,
        _their_node_id: PublicKey,
        _msg: ReplyChannelRange,
    ) -> Result<(), LightningError> {
        Ok(())
    }
    fn handle_reply_short_channel_ids_end(
        &self,
        _their_node_id: PublicKey,
        _msg: ReplyShortChannelIdsEnd,
    ) -> Result<(), LightningError> {
        Ok(())
    }
    fn handle_query_channel_range(
        &self,
        _their_node_id: PublicKey,
        _msg: QueryChannelRange,
    ) -> Result<(), LightningError> {
        Ok(())
    }
    fn handle_query_short_channel_ids(
        &self,
        _their_node_id: PublicKey,
        _msg: QueryShortChannelIds,
    ) -> Result<(), LightningError> {
        Ok(())
    }
    fn provided_node_features(&self) -> NodeFeatures {
        NodeFeatures::empty()
    }
    fn provided_init_features(&self, _their_node_id: PublicKey) -> InitFeatures {
        InitFeatures::empty()
    }
    fn processing_queue_high(&self) -> bool {
        false
    }
}
impl ChannelMessageHandler for MsgHandler {
    fn handle_open_channel(&self, _their_node_id: PublicKey, _msg: &OpenChannel) {}
    fn handle_accept_channel(&self, _their_node_id: PublicKey, _msg: &AcceptChannel) {}
    fn handle_funding_created(&self, _their_node_id: PublicKey, _msg: &FundingCreated) {}
    fn handle_funding_signed(&self, _their_node_id: PublicKey, _msg: &FundingSigned) {}
    fn handle_channel_ready(&self, _their_node_id: PublicKey, _msg: &ChannelReady) {}
    fn handle_shutdown(&self, _their_node_id: PublicKey, _msg: &Shutdown) {}
    fn handle_closing_signed(&self, _their_node_id: PublicKey, _msg: &ClosingSigned) {}
    fn handle_update_add_htlc(&self, _their_node_id: PublicKey, _msg: &UpdateAddHTLC) {}
    fn handle_update_fulfill_htlc(&self, _their_node_id: PublicKey, _msg: &UpdateFulfillHTLC) {}
    fn handle_update_fail_htlc(&self, _their_node_id: PublicKey, _msg: &UpdateFailHTLC) {}
    fn handle_update_fail_malformed_htlc(
        &self,
        _their_node_id: PublicKey,
        _msg: &UpdateFailMalformedHTLC,
    ) {
    }
    fn handle_commitment_signed(&self, _their_node_id: PublicKey, _msg: &CommitmentSigned) {}
    fn handle_revoke_and_ack(&self, _their_node_id: PublicKey, _msg: &RevokeAndACK) {}
    fn handle_update_fee(&self, _their_node_id: PublicKey, _msg: &UpdateFee) {}
    fn handle_announcement_signatures(
        &self,
        _their_node_id: PublicKey,
        _msg: &AnnouncementSignatures,
    ) {
    }
    fn handle_channel_update(&self, _their_node_id: PublicKey, _msg: &ChannelUpdate) {}
    fn handle_open_channel_v2(&self, _their_node_id: PublicKey, _msg: &OpenChannelV2) {}
    fn handle_accept_channel_v2(&self, _their_node_id: PublicKey, _msg: &AcceptChannelV2) {}
    fn handle_stfu(&self, _their_node_id: PublicKey, _msg: &Stfu) {}
    #[cfg(splicing)]
    fn handle_splice_init(&self, _their_node_id: PublicKey, _msg: &SpliceInit) {}
    #[cfg(splicing)]
    fn handle_splice_ack(&self, _their_node_id: PublicKey, _msg: &SpliceAck) {}
    #[cfg(splicing)]
    fn handle_splice_locked(&self, _their_node_id: PublicKey, _msg: &SpliceLocked) {}
    fn handle_tx_add_input(&self, _their_node_id: PublicKey, _msg: &TxAddInput) {}
    fn handle_tx_add_output(&self, _their_node_id: PublicKey, _msg: &TxAddOutput) {}
    fn handle_tx_remove_input(&self, _their_node_id: PublicKey, _msg: &TxRemoveInput) {}
    fn handle_tx_remove_output(&self, _their_node_id: PublicKey, _msg: &TxRemoveOutput) {}
    fn handle_tx_complete(&self, _their_node_id: PublicKey, _msg: &TxComplete) {}
    fn handle_tx_signatures(&self, _their_node_id: PublicKey, _msg: &TxSignatures) {}
    fn handle_tx_init_rbf(&self, _their_node_id: PublicKey, _msg: &TxInitRbf) {}
    fn handle_tx_ack_rbf(&self, _their_node_id: PublicKey, _msg: &TxAckRbf) {}
    fn handle_tx_abort(&self, _their_node_id: PublicKey, _msg: &TxAbort) {}
    fn peer_disconnected(&self, their_node_id: PublicKey) {
        if their_node_id == self.expected_pubkey {
            self.disconnected_flag.store(true, Ordering::SeqCst);
            self.pubkey_disconnected.clone().try_send(()).unwrap();
        }
    }
    fn peer_connected(
        &self,
        their_node_id: PublicKey,
        _init_msg: &Init,
        _inbound: bool,
    ) -> Result<(), ()> {
        if their_node_id == self.expected_pubkey {
            self.pubkey_connected.clone().try_send(()).unwrap();
        }
        Ok(())
    }
    fn handle_channel_reestablish(&self, _their_node_id: PublicKey, _msg: &ChannelReestablish) {}
    fn handle_error(&self, _their_node_id: PublicKey, _msg: &ErrorMessage) {}
    fn provided_node_features(&self) -> NodeFeatures {
        NodeFeatures::empty()
    }
    fn provided_init_features(&self, _their_node_id: PublicKey) -> InitFeatures {
        InitFeatures::empty()
    }
    fn get_chain_hashes(&self) -> Option<Vec<ChainHash>> {
        Some(vec![ChainHash::using_genesis_block(Network::Testnet)])
    }
    fn message_received(&self) {}
}
