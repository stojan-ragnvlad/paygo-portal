It would be cool if the twig project would be wired to a single docker volume.
  This volume would hold all of the source files for each process that would 
be run on a given virtual machine.  The only problem is how would we support 
dynamically creating Docker files...  We would probably need to store a 
Dockerfile.twig file in any project folders that required dynamic docker 
builds.  Then, the twig step would build the appropriate Dockerfile and store 
it on the host machine (not in a volume) so that docker on the host machine 
could use that file.  But, this gets tricky when a project requires multiple 
docker files.  How could multiple docker files be stored in a single folder?  
So, I think the twig step should literally just create an entirely new project 
folder with a dynamically created docker compose file to run the current 
version of the project.  The only issue with this is that local development 
might become quite a pain...  In the case of local development, maybe each 
source folder should have a docker file in it for local development purposes 
and should run "unit tests"!

Run:

```
docker build -t paygo-portal/twig:latest .
```

and:

```
docker run -dit --rm --name twig paygo-portal/twig:latest
```

