# Useful Commands
## Docker images
### Remove conditionally
```sh
docker rmi $(docker images | grep -v 'aws\|node\'client' | awk {'print $3'})
```
Remove all images that are not under the repository with aws, node, or client.
Source: [http://stackoverflow.com/a/19711913]

## Docker containers
### Remove stopped containers
```sh
docker rm $(docker ps -a -q)
```
Source: [http://jimhoskins.com/2013/07/27/remove-untagged-docker-images.html]
