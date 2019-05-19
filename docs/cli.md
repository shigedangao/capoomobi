## Init command

capoomobi init <args>

### Args

init <name>: Create a folder with a name where the Kubernetes yaml file will be located
init <name> <path>: Create a folder with a name where the Kubernetes yaml file will be located as well as using a custom path


--> Will create these folders
<path>/<name>/compose --> docker-compose copy for versioning purposes
<path>/<name>/kube --> folder which store the kubernetes generated files


## Verify command

capoomobi verify <docker-compose.yaml path>

Verify the validity of the docker-compose

## Generate command

capoomobi generate <docker-compose.yaml path>

Generate the Kubernetes files based on the docker-compose

The CLI will ask the user several questions

...Detecting 5 deployments
...Ask for how many pods for each deployment

## Revert command

capoomobi reverte <number>

Revert & generate a set of kubernetes files based on a docker-compose<version number> file.

## Help command

capoomobi help <command>

Will print the information of a command
