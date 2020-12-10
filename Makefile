version := $(shell cat Cargo.toml | grep -E "^version = .*$$" | cut -d= -f2 | sed 's/[" ]//g')
gcp_pid := "cloud-executor"
gcp_region := "us-central1"
gcp_registry := "gcr.io/$(gcp_pid)"
aws_registry := "768088100333.dkr.ecr.us-east-1.amazonaws.com"

include .env

# =================================================
# Build, tag and push the standalone image to a 
# registry
# =================================================
build-standalone:
	@docker build -t code-executor-standalone:$(version) -f Standalone.Dockerfile .

tag-standalone:
	@docker tag code-executor-standalone:$(version) $(gcp_registry)/code-executor-standalone:$(version)

push-standalone:
	@docker push $(gcp_registry)/code-executor-standalone:$(version)


# =================================================
# Build, tag and push the lambda image to a 
# registry
# =================================================
build-lambda:
	@docker build -t code-executor-lambda:$(version) -f Lambda.Dockerfile .

tag-lambda:
	@docker tag code-executor-lambda:$(version) $(aws_registry)/code-executor-lambda:$(version)

push-lambda:
	@docker push $(aws_registry)/code-executor-lambda:$(version)


# =================================================
# Apply and delete deployment and service (manual)
# =================================================
apply-deploy:
	@kubectl apply -f k8s/deployment.yaml

apply-service:
	@sed "s/\$$SERVICE_IP/$(SERVICE_IP)/" k8s/service.in.yaml > service.yaml
	@kubectl apply -f service.yaml
	@rm service.yaml

del-service:
	@kubectl delete service code-executor-service

del-deploy:
	@kubectl delete deployment code-executor-deployment
