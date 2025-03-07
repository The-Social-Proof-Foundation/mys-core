# MySocial Edge Proxy

The MySocial Edge Proxy is a simple proxy service that routes read and execution requests to different fullnode endpoints based on configuration. The purpose is to provide a single entrypoint for all RPC requests, and maintain a consistent connection to the read and execution fullnodes, ensuring the fastest possible client experience for RPC requests.

## Deployment

At the time of writing, the MySocial Edge Proxy is best run as a Kubernetes Deployment. The following guide will walk you through deploying the `mys-edge-proxy` in k8s.

## Guide to Deploying `mys-edge-proxy` in Kubernetes

This guide will walk you through deploying the `mys-edge-proxy` Rust service on a Kubernetes cluster using a deployment configuration and an accompanying service.

### Prerequisites

- Ensure you have `kubectl` installed and configured to connect to your Kubernetes cluster.
- Have `mys-edge-proxy.yaml`, `benchmark-svc.yaml`, and the ConfigMap `mys-edge-proxy-config` prepared.

### Overview of Components

1. **Deployment**: Defines the `mys-edge-proxy` service deployment in Kubernetes.
2. **Service**: Exposes the `mys-edge-proxy` service to other services within the cluster.
3. **ConfigMap**: Provides configuration details for `mys-edge-proxy`.

### Step 1: Set Up the ConfigMap

The ConfigMap provides runtime configurations for `mys-edge-proxy`.

#### `mys-edge-proxy-config.yaml`

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: mys-edge-proxy-config
  namespace: benchmark-rpc-testnet
data:
  proxy.yaml: |
    ---
    listen-address: "0.0.0.0:8080"
    metrics-address: "0.0.0.0:9184"

    execution-peer:
      # Fullnode address for executing requests, this is created by a ServiceExport resource, available in gke fleet clusters:
      # https://cloud.google.com/kubernetes-engine/docs/how-to/multi-cluster-services#registering_a_service_for_export
      address: "http://euw2-testnet-benchmark-service.benchmark-rpc-testnet.svc.clusterset.local:9000"

    read-peer:
      # K8s service address for routing read traffic
      address: "http://mys-node-benchmark.benchmark-rpc-testnet.svc.cluster.local:9000"
```

Apply the ConfigMap:

```bash
kubectl apply -f mys-edge-proxy-config.yaml
```

### Step 2: Deploy `mys-edge-proxy` Deployment

The deployment configuration runs a single instance of `mys-edge-proxy` and mounts the configuration from the ConfigMap.

#### `mys-edge-proxy.yaml`

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: mys-edge-proxy
  namespace: benchmark-rpc-testnet
  labels:
    app: mys-edge-proxy
    cluster: mys-fleet-usw1
    network: testnet
spec:
  replicas: 1
  selector:
    matchLabels:
      app: mys-edge-proxy
  template:
    metadata:
      labels:
        app: mys-edge-proxy
      annotations:
        prometheus.io/path: /metrics
        prometheus.io/port: '9184'
        prometheus.io/scrape: 'true'
    spec:
      containers:
        - name: mys-edge-proxy
          image: mysten/mys-tools:mainnet
          imagePullPolicy: IfNotPresent
          command:
            - /opt/mys/bin/mys-edge-proxy
            - --config=/config/proxy.yaml
          env:
            - name: RUST_LOG
              value: debug
          ports:
            - containerPort: 8080
              protocol: TCP
          volumeMounts:
            - name: config-volume
              mountPath: /config
      volumes:
        - name: config-volume
          configMap:
            name: mys-edge-proxy-config
```

Apply the deployment:

```bash
kubectl apply -f mys-edge-proxy.yaml
```

### Step 3: Create the Service

The Service routes traffic to the `mys-edge-proxy` deployment. It uses `ClientIP` session affinity to maintain connection consistency for each client.

#### `benchmark-svc.yaml`

```yaml
apiVersion: v1
kind: Service
metadata:
  name: mys-node-benchmark
  namespace: benchmark-rpc-testnet
  annotations:
    cloud.google.com/neg: '{"ingress":true}'
spec:
  type: ClusterIP
  selector:
    app: mys-node-benchmark
  ports:
    - port: 9000
      targetPort: 9000
      protocol: TCP
  sessionAffinity: ClientIP
  sessionAffinityConfig:
    clientIP:
      timeoutSeconds: 10800
```

Apply the service:

```bash
kubectl apply -f benchmark-svc.yaml
```

### Step 4: Verify Deployment and Service

After applying all the configurations, check the deployment and service status:

```bash
kubectl get deployments -n benchmark-rpc-testnet
kubectl get services -n benchmark-rpc-testnet
```

Confirm that the `mys-edge-proxy` pod is running:

```bash
kubectl get pods -n benchmark-rpc-testnet -l app=mys-edge-proxy
```

### Summary

Now that this has been deployed, ingress traffic can be pointed at the edge-proxy pods instead of the fullnode pods directly.

## Troubleshooting / Debugging

If you find any issues with the MySocial Edge Proxy or would like to request a feature, please open an issue in the [mys repository](https://github.com/MystenLabs/mys/issues/new).

## Local Development

To run the MySocial Edge Proxy locally and issue test JSON-RPC requests:

1. Create or modify your local config YAML ("mys-edge-proxy-config.yaml") as needed, an example is provided in the repo.
2. In one terminal, run:

   ```bash
   RUST_LOG=debug cargo run -- --config=mys-edge-proxy-config.yaml
   ```

   This will start up the proxy listening on the configured port (e.g. 127.0.0.1:8080).

3. In another terminal, send a test JSON RPC request using curl. For example:

   ```bash
   curl --silent --location --request POST '127.0.0.1:8080' \
   --header 'Content-Type: application/json' \
   --data-raw '{
       "jsonrpc": "2.0",
       "id": 1,
       "method": "mys_getLatestCheckpointSequenceNumber",
       "params": []
   }'
   ```

   You should see debug logs in your first terminal and a JSON response in the second.
