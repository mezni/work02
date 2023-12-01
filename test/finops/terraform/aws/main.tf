provider "aws" {
  region = "ca-central-1"
}

#resource "aws_s3_bucket" "finops_bucket" {
#  bucket = "finops-bucket-1234"
#}

data "archive_file" "lambda_dependency_layer_zip" {
  type        = "zip"
  output_path = "finops-layer/finops_layer.zip"
  source_dir  = "finops-layer"
}

resource "aws_lambda_layer_version" "lambda_dependency_layer" {
  filename            = data.archive_file.lambda_dependency_layer_zip.output_path
  layer_name          = "finops_layer"
  compatible_runtimes = ["python3.11"]
  description         = "finops layer"
  source_code_hash    = data.archive_file.lambda_dependency_layer_zip.output_base64sha256
}


/*
# Create lambda
resource "aws_lambda_function" "finops_lambda" {
  function_name = "FinopsFunction"
  handler       = "index.handler"
  runtime       = "python3.11"
  role          = aws_iam_role.lambda_execution_role.arn
  filename      = "./code.zip"
}

resource "aws_iam_role" "lambda_execution_role" {
  name = "lambda_execution_role"

  assume_role_policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      },
      "Action": "sts:AssumeRole"
    }
  ]
}
EOF
}
*/
/*
resource "aws_lambda_function" "my_lambda_function" {
  function_name = "MyLambdaFunction"
  handler      = "index.handler"
  runtime      = "python3.8"
  role         = aws_iam_role.lambda_execution_role.arn
  filename     = "path/to/your/code.zip"
}

resource "aws_iam_role" "lambda_execution_role" {
  name = "lambda_execution_role"

  assume_role_policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      },
      "Action": "sts:AssumeRole"
    }
  ]
}
EOF
}

# Attach the AWSLambdaBasicExecutionRole policy to the Lambda execution role
resource "aws_iam_role_policy_attachment" "lambda_execution_role_attachment" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = aws_iam_role.lambda_execution_role.name
}
*/
