# Simple actix-web k8s
Simple Hello World web written in Actix running on k8s using Nginx Ingress.

## Prequisted
- Docker
- Kubernetes
- Nginx Ingress

## Getting Started
`ImagePullPolicy` is set to never, you've to build docker image yourself.

1. Create Docker Image
```bash
cd src
./dockerize.sh
```

2. Run deployment
```bash
# at /k8s
./up.sh
```

3. To stop
```bash
# at /k8s
./down.sh
```

#### Chaning package name
Match `name` in `cargo.toml` and Dockerfile by replacing `hello-actix` with your package name.

### Note
- Image is 611.1 KB
- Build stage using `rust-alpine`.
- Build time is around 5 minute on Ryzen 5 3500X, 16GB DDR4 3266MHZ