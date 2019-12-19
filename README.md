# CapooMobi

Capoomobi is a project where I'm learning Rust ! Capoomobi is a helper tool allowing you to generate a Kubernetes boilerplate files from your docker-compose files

## Idea

Ever wondering how hard is to translate a simple to a set of complex docker-compose to a set of Kubernetes objects ? Well this project aim to render this process automatic.

Deploying your docker-compose to Kubernetes with ease

## Examples

- Create a project

```shell
capoomobi init rabbit ../rabbit
```

- Help command

```shell
capoomobi help
```

- Generate command

```shell
capoomobi generate <path to docker-compose.yaml>
capoomobi generate <path to docker-compose.yaml> (--print | --ingress)
```

- Project command

```shell
capoomobi project current -> display current project
capoomobi project list --> display list of project
capoomobi project switch --> change project to an other
capoomobi project delete --> delete a project
```

A docker-compose.yaml file is available in the ```example folder```