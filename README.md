# paygo-portal
Public Implementation of the Paygo Portal Infrastructure

## Creating an Application

1. Start up an Ubuntu Virtual Machine in a cloud environment.

2. Connect to the virtual machine via SSH in a terminal or browser terminal.

3. Download the necessary code into the virtual machine by running the
following:

```
curl \
  -LO\
  https://github.com/stojan-ragnvlad/paygo-portal/archive/refs/heads/main.zip
```

4. Install the `zip` utility by running the following:

```
sudo apt install zip
```

5. Install Docker by doing the following:

Docker is available in the default repositories. You can install via apt:

```
sudo apt install docker.io
```

Then add yourself to the docker group:

```
sudo usermod -a -G docker $(whoami)
```

Restart the docker service and relogin, reboot, or start a new login shell.

## Installing Docker Desktop

Download the proper version for your machine or environment at the following
link:

https://docs.docker.com/desktop/release-notes/

## Installing Kubernetes

### Linux

#### kubectl

For version 1.30.0 of kubectl, run the following command from the home
directory of your terminal:

```
curl -LO https://dl.k8s.io/release/v1.30.0/bin/linux/amd64/kubectl
```

Then, run the following command from the home directory of your terminal to
make the binary executable:

```
chmod +x kubectl
```

Next, create a `~/bin` directory by running the following command from the home
directory of your terminal:

```
mkdir bin
```

Then, add the `bin` directory to your terminal path by adding the following
code to the end of your `~/.bashrc` file:

```
export PATH="$PATH:/home/USERNAME/bin"
```

Next, move the `kubectl` file into the `~/bin` directory using the following
command from the home directory of your terminal:

```
mv kubectl bin
```

Finally, close and re-open your terminal session and run the following command
to ensure `kubectl` is accessible anywhere in your terminal:

```
kubectl version
```

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

```
minikube-linux-amd64.tar.gz
```

Open a terminal in the `~/Downloads` directory by running the following from
anywhere in your terminal:

```
cd ~/Downloads
```

Extract the minikube files by running the following command from the
`~/Downloads` directory of your terminal:

```
tar -xvzf minikube-linux-amd64.tar.gz
```

Move and rename the `minikube-linux-amd64` executable file into the `~/bin`
directory by running the following command from the downloads directory
of your terminal:

```
mv minikube-linux-amd64 ~/bin/minikube
```

Running the following command from anywhere in your terminal:

```
minikube version
```

Should output the following text:

```
minikube version: v1.33.0
```

To start a `minikube` cluster, run the following code from anywhere in your
terminal:

```
minikube start
```

## Logging into Docker Hub

### Creating a GPG Key Pair

From anywhere in your terminal, run the following command to create a key:

```
gpg --generate-key
```

Follow the terminal prompts.

Copy the GPG ID from the line of text that looks like the following:

```
gpg: key XXXXXXXXXXXXXXXX marked as ultimately trusted
```

where `XXXXXXXXXXXXXXXX` is the GPG ID.

### Initializing pass

Run the following command from anywhere in your terminal:

```
pass init XXXXXXXXXXXXXXXX
```

where `XXXXXXXXXXXXXXXX` is the GPG ID.

### Log into Docker on Your Machine

Run the following command from anywhere in your terminal:

```
docker login
```

Follow the terminal prompts.

## Updating Docker Images in Docker Hub

To locally build the docker image, run the following command from the
directory of this read me file in your terminal:

```
docker build -t YOUR-DOCKER-HUB-USERNAME/IMAGE_DIRECTORY_NAME:latest .
```

To update the docker image in the registry, run the following command:

```
docker push YOUR-DOCKER-HUB-USERNAME/IMAGE_DIRECTORY_NAME:latest
```

## Starting the Cluster on a Local Machine

Open a terminal in the same directory as this read me file.

Change your directory to the `static-web-server/` directory.

Run the following command to create the deployment:

```
kubectl create -f pod.yaml
```

Run the following command to check the status of the created deployment:

```
kubectl get deployments
```

The output of the previous command should look like the following after around
a minute or so:

```
static-web-server-deployment   1/1     1            1           4m24s
```

Run the following command to port forward a localhost port to the deployment,
where LOCAL_PORT is a local port like 8082 that you could view in the web
browser:

```
kubectl port-forward deployment/static-web-server-deployment LOCAL_PORT:80
```

## Postgres Notes:

Run this to temporarily disable terminal history saving:

```
set +o history
```

Run this to install the proper command line tool.

```
curl -OL https://github.com/lukasmartinelli/pgfutter/releases/download/v1.2/pgfutter_linux_amd64
```

Pull the docker image:

```
sudo docker pull postgres:16.3-alpine3.20
```

Then:

```
sudo docker run -d -i --name some-postgres -h 127.0.0.1 -p 8081:5432 -e POSTGRES_PASSWORD=mysecretpassword postgres:16.3-alpine3.20
```

```
sudo docker exec -it some-postgres sh
```

```
psql -U postgres
```

```
CREATE DATABASE nutrition;
```

```
\c nutrition;
```

```
curl -OL https://fdc.nal.usda.gov/fdc-datasets/FoodData_Central_csv_2024-04-18.zip
```

Then:

```
pgfutter --host "localhost" --db "myDatabase" --port "5432" --user "postgres" --pw "mySecretPassword" csv myCSVfile.csv
```

```
pip install csvkit psycopg2-binary
```

```
csvsql --db "postgresql://hostname:port/nutrition?user=postgres&password=password" --insert --create-if-not-exists --db-schema "public" acquisition_samples.csv
```

https://docs.docker.com/engine/install/ubuntu/#install-from-a-package

