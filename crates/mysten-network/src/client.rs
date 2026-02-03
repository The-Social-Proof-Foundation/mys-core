// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use crate::{
    config::Config,
    multiaddr::{Multiaddr, Protocol, parse_dns, parse_ip4, parse_ip6},
};
use eyre::{Context, Result, eyre};
use hyper_util::client::legacy::connect::dns::Name;
use std::{
    collections::HashMap,
    fmt,
    future::Future,
    io,
    net::{SocketAddr, ToSocketAddrs},
    pin::Pin,
    sync::{Arc, Mutex},
    task::{self, Poll},
    time::Instant,
    vec,
};
use tokio::task::JoinHandle;
use tokio_rustls::rustls::ClientConfig;
use tonic::transport::Uri;
use tonic_rustls::Channel;
use tower::Service;
use tracing::trace;

pub async fn connect(address: &Multiaddr, tls_config: ClientConfig) -> Result<Channel> {
    let channel = endpoint_from_multiaddr(address, tls_config)?
        .connect()
        .await?;
    Ok(channel)
}

pub fn connect_lazy(address: &Multiaddr, tls_config: ClientConfig) -> Result<Channel> {
    let channel = endpoint_from_multiaddr(address, tls_config)?.connect_lazy();
    Ok(channel)
}

pub(crate) async fn connect_with_config(
    address: &Multiaddr,
    tls_config: ClientConfig,
    config: &Config,
) -> Result<Channel> {
    let channel = endpoint_from_multiaddr(address, tls_config)?
        .apply_config(config)
        .connect()
        .await?;
    Ok(channel)
}

pub(crate) fn connect_lazy_with_config(
    address: &Multiaddr,
    tls_config: ClientConfig,
    config: &Config,
) -> Result<Channel> {
    let channel = endpoint_from_multiaddr(address, tls_config)?
        .apply_config(config)
        .connect_lazy();
    Ok(channel)
}

fn endpoint_from_multiaddr(addr: &Multiaddr, tls_config: ClientConfig) -> Result<MyEndpoint> {
    let mut iter = addr.iter();

    let channel = match iter.next().ok_or_else(|| eyre!("address is empty"))? {
        Protocol::Dns(_) => {
            let (dns_name, tcp_port, http_or_https) = parse_dns(addr)?;
            let uri = format!("{http_or_https}://{dns_name}:{tcp_port}");
            MyEndpoint::try_from_uri(uri, tls_config)?
        }
        Protocol::Ip4(_) => {
            let (socket_addr, http_or_https) = parse_ip4(addr)?;
            let uri = format!("{http_or_https}://{socket_addr}");
            MyEndpoint::try_from_uri(uri, tls_config)?
        }
        Protocol::Ip6(_) => {
            let (socket_addr, http_or_https) = parse_ip6(addr)?;
            let uri = format!("{http_or_https}://{socket_addr}");
            MyEndpoint::try_from_uri(uri, tls_config)?
        }
        unsupported => return Err(eyre!("unsupported protocol {unsupported}")),
    };

    Ok(channel)
}

struct MyEndpoint {
    uri: String,
    tls_config: ClientConfig,
    config: Option<Config>,
}

impl MyEndpoint {
    fn new(uri: String, tls_config: ClientConfig) -> Self {
        Self {
            uri,
            tls_config,
            config: None,
        }
    }

    fn try_from_uri(uri: String, tls_config: ClientConfig) -> Result<Self> {
        // Validate URI
        uri.parse::<Uri>()
            .with_context(|| format!("unable to create Uri from '{uri}'"))?;
        Ok(Self::new(uri, tls_config))
    }

    fn apply_config(mut self, config: &Config) -> Self {
        self.config = Some(config.clone());
        self
    }

    fn connect_lazy(self) -> Channel {
        // Use tonic_rustls::Channel API similar to consensus/core
        let mut builder = tonic_rustls::Channel::from_shared(self.uri.clone())
            .expect("URI should be valid");
        
        // Apply config if available
        if let Some(ref config) = self.config {
            if let Some(timeout) = config.connect_timeout {
                builder = builder.connect_timeout(timeout);
            }
            if let Some(window_size) = config.http2_initial_stream_window_size {
                builder = builder.initial_stream_window_size(Some(window_size));
            }
            if let Some(window_size) = config.http2_initial_connection_window_size {
                builder = builder.initial_connection_window_size(Some(window_size));
            }
            if let Some(keepalive) = config.http2_keepalive_interval {
                builder = builder.http2_keep_alive_interval(keepalive);
            }
            if let Some(timeout) = config.http2_keepalive_timeout {
                builder = builder.keep_alive_timeout(timeout);
            }
            if let Some(keepalive) = config.tcp_keepalive {
                builder = builder.tcp_keepalive(Some(keepalive));
            }
        }
        
        builder
            .tls_config(self.tls_config)
            .expect("TLS config should be valid")
            .connect_lazy()
    }

    async fn connect(self) -> Result<Channel> {
        // Use tonic_rustls::Channel API similar to consensus/core
        let mut builder = tonic_rustls::Channel::from_shared(self.uri.clone())
            .map_err(|e| eyre!("invalid URI '{}': {}", self.uri, e))?;
        
        // Apply config if available
        if let Some(ref config) = self.config {
            if let Some(timeout) = config.connect_timeout {
                builder = builder.connect_timeout(timeout);
            }
            if let Some(window_size) = config.http2_initial_stream_window_size {
                builder = builder.initial_stream_window_size(Some(window_size));
            }
            if let Some(window_size) = config.http2_initial_connection_window_size {
                builder = builder.initial_connection_window_size(Some(window_size));
            }
            if let Some(keepalive) = config.http2_keepalive_interval {
                builder = builder.http2_keep_alive_interval(keepalive);
            }
            if let Some(timeout) = config.http2_keepalive_timeout {
                builder = builder.keep_alive_timeout(timeout);
            }
            if let Some(keepalive) = config.tcp_keepalive {
                builder = builder.tcp_keepalive(Some(keepalive));
            }
        }
        
        let channel = builder
            .tls_config(self.tls_config)
            .map_err(|e| eyre!("invalid TLS config: {}", e))?
            .connect()
            .await
            .map_err(|e| eyre!("failed to connect: {}", e))?;
        Ok(channel)
    }
}

type CacheEntry = (Instant, Vec<SocketAddr>);

/// A caching resolver based on hyper_util GaiResolver
#[derive(Clone)]
pub struct CachingResolver {
    cache: Arc<Mutex<HashMap<Name, CacheEntry>>>,
}

type SocketAddrs = vec::IntoIter<SocketAddr>;

pub struct CachingFuture {
    inner: JoinHandle<Result<SocketAddrs, io::Error>>,
}

impl CachingResolver {
    pub fn new() -> Self {
        CachingResolver {
            cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl Default for CachingResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl Service<Name> for CachingResolver {
    type Response = SocketAddrs;
    type Error = io::Error;
    type Future = CachingFuture;

    fn poll_ready(&mut self, _cx: &mut task::Context<'_>) -> Poll<Result<(), io::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, name: Name) -> Self::Future {
        let blocking = {
            let cache = self.cache.clone();
            tokio::task::spawn_blocking(move || {
                let entry = cache.lock().unwrap().get(&name).cloned();

                if let Some((when, addrs)) = entry {
                    trace!("cached host={:?}", name.as_str());

                    if when.elapsed().as_secs() > 60 {
                        trace!("refreshing cache for host={:?}", name.as_str());
                        // Start a new task to update the cache later.
                        tokio::task::spawn_blocking(move || {
                            if let Ok(addrs) = (name.as_str(), 0).to_socket_addrs() {
                                let addrs: Vec<_> = addrs.collect();
                                trace!("updating cached host={:?}", name.as_str());
                                cache
                                    .lock()
                                    .unwrap()
                                    .insert(name, (Instant::now(), addrs.clone()));
                            }
                        });
                    }

                    Ok(addrs.into_iter())
                } else {
                    trace!("resolving host={:?}", name.as_str());
                    match (name.as_str(), 0).to_socket_addrs() {
                        Ok(addrs) => {
                            let addrs: Vec<_> = addrs.collect();
                            cache
                                .lock()
                                .unwrap()
                                .insert(name, (Instant::now(), addrs.clone()));
                            Ok(addrs.into_iter())
                        }
                        res => res,
                    }
                }
            })
        };

        CachingFuture { inner: blocking }
    }
}

impl fmt::Debug for CachingResolver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("CachingResolver")
    }
}

impl Future for CachingFuture {
    type Output = Result<SocketAddrs, io::Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner).poll(cx).map(|res| match res {
            Ok(Ok(addrs)) => Ok(addrs),
            Ok(Err(err)) => Err(err),
            Err(join_err) => {
                if join_err.is_cancelled() {
                    Err(io::Error::new(io::ErrorKind::Interrupted, join_err))
                } else {
                    panic!("background task failed: {:?}", join_err)
                }
            }
        })
    }
}

impl fmt::Debug for CachingFuture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("CachingFuture")
    }
}

impl Drop for CachingFuture {
    fn drop(&mut self) {
        self.inner.abort();
    }
}
