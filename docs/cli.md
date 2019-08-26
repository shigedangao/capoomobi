## Init command

capoomobi init <args>

### Args

init <name>: Create a folder with a name where the Kubernetes yaml file will be located
init <name> <path>: Create a folder with a name where the Kubernetes yaml file will be located as well as using a custom path

--> Will create these folders
<path>/<name>/compose --> docker-compose copy for versioning purposes
<path>/<name>/kube --> folder which store the kubernetes generated files

--> Will create a config file where the path of each project is store e.g

{
  projects: [
    {
      name: "foo",
      path: "/foo/bar"
    },
    {
      name: "Miam",
      path: "/miam/bao"
    }
  ],
  "current": "foo"
}


So that we can switch from project to project w/o any headache

## Verify command

capoomobi verify <docker-compose.yaml path>

Verify the validity of the docker-compose

## Project command

capoomobi init <main> <args>

### List project availables

capoomobi project list

### Change current project

capoomobi project switch <project_name>

### Delete project

capoomobi project delete <project_name>

## Help command

capoomobi help <command>

Will print the information of a command
