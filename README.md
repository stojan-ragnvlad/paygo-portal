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

#### kind

Go to the following web page in a web browser and follow the instructions:

https://kind.sigs.k8s.io/docs/user/quick-start/#installing-from-release-binaries

#### minikube

Go to the following GitHub page in a web browser:

https://github.com/kubernetes/minikube/releases/tag/v1.33.0

Download the file from the `Assets` section of this web page that has the
following filename:

```minikube-linux-amd64.tar.gz```

Open a terminal in the `~/Downloads` directory by running the following from
anywhere in your terminal:

```cd ~/Downloads```

Extract the minikube files by running the following command from the
`~/Downloads` directory of your terminal:

```tar -xvzf minikube-linux-amd64.tar.gz```

Move and rename the `minikube-linux-amd64` executable file into the `~/bin`
directory by running the following command from the downloads directory
of your terminal:

```mv minikube-linux-amd64 ~/bin/minikube```

Running the following command from anywhere in your terminal:

```minikube version```

Should output the following text:

```minikube version: v1.33.0```

To start a `minikube` cluster, run the following code from anywhere in your
terminal:

```minikube start```
