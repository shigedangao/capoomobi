# ▓ Capoo ⣿ Mobi ▓

[![pipeline status](https://gitlab.com/MarcInthaamnouay/capoomobi/badges/master/pipeline.svg)](https://gitlab.com/MarcInthaamnouay/capoomobi/commits/master)

Capoomobi is a small project which helps you to convert your docker-compose.yaml file to a valid set of kubernetes manifest files.

# Installation

Binary will be available in a few days. In the meantime you can clone this repository and build the project by yourself. 

# Build the project

The project is made with Rust as such if you wish to contribute you'll need to install it on the first hand
Run the project by using the command below

```shell
cargo run <command> <args>
```

# Commands available

## Init

Create command allow you to init a new project

```shell
cargo run init <name> <path of your project>

## example

cargo run init my-nginx ../nginx
```

## Generate

Generate command generate a set of kubernetes manifests. The command will generate these manifests for each services:
- controller.yaml
- service.yaml

In order to generate a set of manifests you'll need to configure a small configuration file on your folder which contain the docker-compose.yaml. This file should be name ```confiture.json```. This file helps the command to know what to generate.

```json
{
  "confitures": [
    {
      "name": "<name of a docker-compose service>",
      "deployment": {
        "replicas": <number>,
        "controller": "either: Deployment|StatefulSet|DaemonSet"
      },
      "service": {
        "kind": "either: NodePort (expose outside of the cluster) | ClusterIP (not expose outside of the cluster)",
        "nodeport": <number (only use for nodeport)>
      }
    }
  ],
  // optional
  "ingress": {
    "ip": <ip given by a provider>
    "services": [
      {
        "name": "name of the docker-compose service",
        "path": "path expose to the world"
      }
    ]
  }
}
```

### Generate a configuration w/o the ingress configuration file

```shell
cargo run generate <path of your project>
```

### Generate the ingress configuration file

```shell
cargo run generate <path of your project> --ingress
```

### Print the manifest (output mode)

```shell
cargo run generate <path of your project> --print
```

## Project

Project command allow you to switch, delete, list between projects. The command are below

```shell
capoomobi project current -> display current project
capoomobi project list --> display list of project
capoomobi project switch --> change project to an other
capoomobi project delete --> delete a project
```

## Help

Get some help on how to use the cli

```shell
capoomobi help
capoomobi help init
capoomobi help generate
capoomobi help project
```