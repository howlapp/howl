# Infrastructure

## Setup

Install the local-path provisioner:

```bash
$ minikube kubectl -- apply -f https://raw.githubusercontent.com/rancher/local-path-provisioner/v0.0.24/deploy/local-path-storage.yaml
```

Install Consul to the cluster:

```bash
$ minikube kubectl -- apply -k "github.com/hashicorp/consul-api-gateway/config/crd?ref=v0.4.0"
$ helm install consul hashicorp/consul --set global.name=consul --create-namespace --namespace macrostack --values charts/consul.yaml
```

## Usage

View the Consul UI:

```bash
$ minikube kubectl -- port-forward service/consul-server 8500:8500 -n macrostack
```
