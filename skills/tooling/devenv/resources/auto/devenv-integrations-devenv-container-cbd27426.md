# Devenv Container

You can use the [devenv
container](https://github.com/cachix/devenv/pkgs/container/devenv%2Fdevenv)
to run devenv commands on your preferred container-based system.

Any container-based environment like Gitlab CI, Kubernetes, Docker, is
supported.

- [Docker](#tab-panel-87)
- [GitLab CI](#tab-panel-88)
- [Kubernetes](#tab-panel-89)

```
docker run ghcr.io/cachix/devenv/devenv:latest devenv shell hello-world
```

Terminal window

```
devenv-job:  image: ghcr.io/cachix/devenv/devenv:latest  script: devenv shell hello-world
```

```
apiVersion: batch/v1kind: Jobmetadata:  name: devenv-jobspec:  template:    spec:      containers:        - name: devenv-job          image: ghcr.io/cachix/devenv/devenv:latest          command: ["devenv", "tasks", "run", "my-app:hello-world"]      restartPolicy: Never  backoffLimit: 4
```
