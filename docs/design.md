## Design

Below is the design of the project

## Components

In order to make this project there should be some component at least for what I think first

-- Entry

- Environment (minikube, gcp...)

-- Engines & Docker

- Docker-compose parser
- Engine selector (docker, rkt)
- Image builder (by using docker engine) / Optional
- Image uploader ? (gcp, docker hub) / Optional

-- Kubernetes

- Create deployment (name of docker-compose services)
- Create services (when there is a port)
- Create default ingress ?


-- CLI

A cli for asking the user which kind of environment the people want to deploy

- minikube
- gcp
- amazon
- rancher


- User preference saver
- Namespace based (root project name or generate name with the original path of the docker-compose)

