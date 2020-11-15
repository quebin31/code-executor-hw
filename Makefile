aws_region="us-east-1"
aws_register="174655635967.dkr.ecr.$(aws_region).amazonaws.com/code-executor"

docker-image:
	@docker build -t code-executor .

tag-image:
	@docker tag code-executor:latest $(aws_register)/code-executor:latest

push-image:
	@docker push $(aws_register)/code-executor:latest

create-cluster:
	@eksctl create cluster \
		--name code-executor \
		--version 1.18 \
		--region $(aws_region) \
		--fargate 

delete-cluster:
	@eksctl delete cluster \
		--region $(aws_region) \
		--name code-executor

deploy:
	@kubectl apply -f service.yaml
