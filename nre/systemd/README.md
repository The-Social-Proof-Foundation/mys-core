# Run a MySocial Node using Systemd

Tested using:
- Ubuntu 20.04 (linux/amd64) on bare metal
- Ubuntu 22.04 (linux/amd64) on bare metal

## Prerequisites and Setup

1. Add a `mys` user and the `/opt/mys` directories

```shell
sudo useradd mys
sudo mkdir -p /opt/mys/bin
sudo mkdir -p /opt/mys/config
sudo mkdir -p /opt/mys/db
sudo mkdir -p /opt/mys/key-pairs
sudo chown -R mys:mys /opt/mys
```

2. Install the MySocial Node (mys-node) binary, two options:
    
- Pre-built binary stored in Amazon S3:
        
```shell
wget https://releases.mys.io/$MYS_SHA/mys-node
chmod +x mys-node
sudo mv mys-node /opt/mys/bin
```

- Build from source:

```shell
git clone https://github.com/MystenLabs/mys.git && cd mys
git checkout $MYS_SHA
cargo build --release --bin mys-node
mv ./target/release/mys-node /opt/mys/bin/mys-node
```

3. Copy your key-pairs into `/opt/mys/key-pairs/` 

If generated during the Genesis ceremony these will be at `MysExternal.git/mys-testnet-wave3/genesis/key-pairs/`

Make sure when you copy them they retain `mys` user permissions. To be safe you can re-run: `sudo chown -R mys:mys /opt/mys`

4. Update the node configuration file and place it in the `/opt/mys/config/` directory.

Add the paths to your private keys to validator.yaml. If you chose to put them in `/opt/mys/key-pairs`, you can use the following example: 

```
protocol-key-pair: 
  path: /opt/mys/key-pairs/protocol.key
worker-key-pair: 
  path: /opt/mys/key-pairs/worker.key
network-key-pair: 
  path: /opt/mys/key-pairs/network.key
```

5. Place genesis.blob in `/opt/mys/config/` (should be available after the Genesis ceremony)

6. Copy the mys-node systemd service unit file 

File: [mys-node.service](./mys-node.service)

Copy the file to `/etc/systemd/system/mys-node.service`.

7. Reload systemd with this new service unit file, run:

```shell
sudo systemctl daemon-reload
```

8. Enable the new service with systemd

```shell
sudo systemctl enable mys-node.service
```

## Connectivity

You may need to explicitly open the ports outlined in [MySocial for Node Operators](../mys_for_node_operators.md#connectivity) for the required MySocial Node connectivity.

## Start the node

Start the Validator:

```shell
sudo systemctl start mys-node
```

Check that the node is up and running:

```shell
sudo systemctl status mys-node
```

Follow the logs with:

```shell
journalctl -u mys-node -f
```

## Updates

When an update is required to the MySocial Node software the following procedure can be used. It is highly **unlikely** that you will want to restart with a clean database.

- assumes mys-node lives in `/opt/mys/bin/`
- assumes systemd service is named mys-node
- **DO NOT** delete the MySocial databases

1. Stop mys-node systemd service

```
sudo systemctl stop mys-node
```

2. Fetch the new mys-node binary

```shell
wget https://releases.mys.io/${MYS_SHA}/mys-node
```

3. Update and move the new binary:

```
chmod +x mys-node
sudo mv mys-node /opt/mys/bin/
```

4. start mys-node systemd service

```
sudo systemctl start mys-node
```
