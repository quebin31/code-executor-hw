resource "aws_api_gateway_rest_api" "code_executor" {
  name        = "CodeExecutorGateway"
  description = "Serverless for code executor"
}
