# paygo-portal
Public Implementation of the Paygo Portal Infrastructure

## Installing Docker Desktop

Download the proper version for your machine or environment at the following
link:

https://docs.docker.com/desktop/release-notes/

## Installing Kubernetes

### Linux

#### kubectl

For version 1.30.0 of kubectl, run the following command from the home
directory of your terminal:

```curl -LO https://dl.k8s.io/release/v1.30.0/bin/linux/amd64/kubectl```

Then, run the following command from the home directory of your terminal to
make the binary executable:

```chmod +x kubectl```

Next, create a `~/bin` directory by running the following command from the home
directory of your terminal:

```mkdir bin```

Then, add the `bin` directory to your terminal path by adding the following
code to the end of your `~/.bashrc` file:

```
export PATH="$PATH:/home/USERNAME/bin"
```

Next, move the `kubectl` file into the `~/bin` directory using the following
command from the home directory of your terminal:

```mv kubectl bin```

Finally, close and re-open your terminal session and run the following command
to ensure `kubectl` is accessible anywhere in your terminal:

```kubectl version```

Which should output:

```
Client Version: v1.30.0
```

#### minikube

Follow the following link instructions:

https://minikube.sigs.k8s.io/docs/start/
