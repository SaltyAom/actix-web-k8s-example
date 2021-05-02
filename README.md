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

### Note
- Image is 2.98MB
- Build stage using `rust-alpine`.
- Build time is around 10-15 minute on MacBook Pro 2019 (i5 1.4GHz model).