## Docker-compose.yaml

Docker-compose.yaml is the basis for deploying the application. Sometimes we don't want to deploy everything. Thus it might be useful for a developer or a devops to specify what he want to ignore

In order to support this feature we'll introducing the annotation on the docker-compose.yml

## Annoations list

- #@Ignore: Ignore the service
- #@Port: Override the ports values that'll be assign to the Kuberntes deployment & services files
- #@ExposeClusterIP: Will only expose to the ClusterIP