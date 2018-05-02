// Copyright 2018 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under The General Public License (GPL), version 3.
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied. Please review the Licences for the specific language governing
// permissions and limitations relating to use of the SAFE Network Software.

use super::ConnectionInfoResult;

use common::{CrustUser, Uid};
use std::net::SocketAddr;

/// Enum representing different events that will be sent over the asynchronous channel to the user
/// of this module.
#[derive(Debug)]
pub enum Event<UID: Uid> {
    /// Invoked when a bootstrap peer connects to us
    BootstrapAccept(UID, CrustUser),
    /// Invoked when we bootstrap to a new peer.
    BootstrapConnect(UID, SocketAddr),
    /// Invoked when we failed to connect to all bootstrap contacts.
    BootstrapFailed,
    /// Invoked when we are ready to listen for incomming connection. Contains
    /// the listening port.
    ListenerStarted(u16),
    /// Invoked when listener failed to start.
    ListenerFailed,
    /// Invoked as a result to the call of `Service::prepare_contact_info`.
    ConnectionInfoPrepared(ConnectionInfoResult<UID>),
    /// Invoked when connection to a new peer has been established.
    ConnectSuccess(UID),
    /// Invoked when connection to a new peer has failed.
    ConnectFailure(UID),
    /// Invoked when a peer disconnects or can no longer be contacted.
    LostPeer(UID),
    /// Invoked when a new message is received. Passes the message.
    NewMessage(UID, CrustUser, Vec<u8>),
    /// Invoked when trying to sending a too large data.
    WriteMsgSizeProhibitive(UID, Vec<u8>),
}
