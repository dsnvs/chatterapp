use std::{io};
use futures::{AsyncRead, AsyncWrite, AsyncWriteExt};
use libp2p::{request_response::{ProtocolName, RequestResponseCodec}, core::upgrade::{read_length_prefixed, write_length_prefixed},};


#[derive(Clone, Debug)]
pub struct GenericProtocol;

#[derive(Clone, Debug, Default)]
pub struct GenericCodec;

pub type IncomingRequest = Vec<u8>;
pub type OutgoingResponse = Vec<u8>;

impl ProtocolName for GenericProtocol {
    fn protocol_name(&self) -> &[u8] {
        "/Generic/1".as_bytes()
    }
}


// Max Request Size is 1024
// This codec expects data to already be serialized

#[async_trait::async_trait]
impl RequestResponseCodec for GenericCodec {
    type Protocol = GenericProtocol;
    type Request = IncomingRequest;
    type Response = OutgoingResponse;

    async fn read_request<T>(&mut self, _protocol: &Self::Protocol, io: &mut T) -> io::Result<Self::Request>
    where
        T: AsyncRead + Unpin + Send,
    {
        let request_in_bytes = read_length_prefixed(io, 1024).await?;
        Ok(request_in_bytes)
    }

    async fn read_response<T>(&mut self, _protocol: &Self::Protocol, io: &mut T) -> io::Result<Self::Response>
    where
        T: AsyncRead + Unpin + Send,
    {
        let response_in_bytes = read_length_prefixed(io, 1024).await?;
        Ok(response_in_bytes)
    }

    async fn write_request<T>(&mut self, _: &Self::Protocol, io: &mut T, req: Self::Request) -> io::Result<()>
    where
        T: AsyncWrite + Unpin + Send,
    {
        write_length_prefixed(io, &req).await?;
        io.close().await
    }

    async fn write_response<T>(&mut self, _: &Self::Protocol, io: &mut T, res: Self::Response) -> io::Result<()>
    where
        T: AsyncWrite + Unpin + Send,
    {
        write_length_prefixed(io, &res) .await?;
        io.close().await
    }
}