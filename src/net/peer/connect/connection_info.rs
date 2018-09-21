// Copyright 2016 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement.  This, along with the Licenses can be
// found in the root directory of this project at LICENSE, COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.
//
// Please review the Licences for the specific language governing permissions and limitations
// relating to use of the SAFE Network Software.

use bytes::Bytes;
use future_utils::bi_channel::UnboundedBiChannel;
use futures::sync::oneshot;
use priv_prelude::*;

#[derive(Debug)]
/// Connection information specifically used for rendezvous connections carried by `p2p` crate.
pub struct P2pConnectionInfo {
    // opaque data representing our rendezvous connection info which `p2p` is able to make sense of
    pub our_info: Bytes,
    // data channel used to send remote peer's connection information
    pub rendezvous_channel: UnboundedBiChannel<Bytes>,
    // future that resolves to p2p connection when it's done
    pub connection_rx: oneshot::Receiver<Result<PaStream, RendezvousConnectError>>,
}

/// Contact info generated by a call to `Service::prepare_contact_info`.
#[derive(Debug)]
pub struct PrivConnectionInfo {
    #[doc(hidden)]
    pub connection_id: u64,
    #[doc(hidden)]
    pub for_direct: Vec<PaAddr>,
    // P2P connection info is optional in case when p2p `rendezvous_connect` fails.
    // In such cases we can still proceed with direct connections, but we must keep track
    // that P2P connections should not be attempted.
    #[doc(hidden)]
    pub p2p_conn_info: Option<P2pConnectionInfo>,
    #[doc(hidden)]
    pub our_uid: PublicEncryptKey,
    #[doc(hidden)]
    pub our_pk: PublicEncryptKey,
    #[doc(hidden)]
    pub our_sk: SecretEncryptKey,
}

/// Contact info used to connect to another peer.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PubConnectionInfo {
    #[doc(hidden)]
    pub connection_id: u64,
    #[doc(hidden)]
    pub for_direct: Vec<PaAddr>,
    #[doc(hidden)]
    pub p2p_conn_info: Option<Vec<u8>>,
    #[doc(hidden)]
    pub pub_key: PublicEncryptKey,
    #[doc(hidden)]
    pub uid: PublicEncryptKey,
}

impl PubConnectionInfo {
    /// Returns the `UID` of the node that created this connection info.
    pub fn id(&self) -> PublicEncryptKey {
        self.pub_key
    }
}

impl PrivConnectionInfo {
    /// Use private connection info to create public connection info that can be shared with the
    /// peer.
    pub fn to_pub_connection_info(&self) -> PubConnectionInfo {
        let p2p_conn_info = self
            .p2p_conn_info
            .as_ref()
            .and_then(|conn_info| Some(conn_info.our_info.to_vec()));
        PubConnectionInfo {
            connection_id: self.connection_id,
            for_direct: self.for_direct.clone(),
            uid: self.our_uid,
            p2p_conn_info,
            pub_key: self.our_pk,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod priv_connection_info {
        use super::*;

        mod to_pub_connection_info {
            use super::*;
            use future_utils::bi_channel::unbounded;

            #[test]
            fn when_p2p_conn_info_is_none_it_sets_none_in_public_conn_info_too() {
                let (our_pk, our_sk) = gen_encrypt_keypair();
                let our_uid = our_pk;
                let priv_conn_info = PrivConnectionInfo {
                    connection_id: 123,
                    our_uid,
                    for_direct: vec![],
                    p2p_conn_info: None,
                    our_pk,
                    our_sk,
                };

                let pub_conn_info = priv_conn_info.to_pub_connection_info();

                assert!(pub_conn_info.p2p_conn_info.is_none());
            }

            #[test]
            fn when_p2p_conn_info_is_not_none_it_is_stored_as_vector_in_public_conn_info() {
                let (rendezvous_channel, _) = unbounded();
                let (_, connection_rx) = oneshot::channel();
                let p2p_conn_info = Some(P2pConnectionInfo {
                    our_info: Bytes::from(vec![1, 2, 3]),
                    rendezvous_channel,
                    connection_rx,
                });
                let (our_pk, our_sk) = gen_encrypt_keypair();
                let our_uid = our_pk;
                let priv_conn_info = PrivConnectionInfo {
                    connection_id: 123,
                    our_uid,
                    for_direct: vec![],
                    p2p_conn_info,
                    our_pk,
                    our_sk,
                };

                let pub_conn_info = priv_conn_info.to_pub_connection_info();

                assert_eq!(pub_conn_info.p2p_conn_info, Some(vec![1, 2, 3]));
            }
        }
    }
}
