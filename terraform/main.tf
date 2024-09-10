provider "aws" {
  region = "us-east-1"
}

resource "random_string" "id" {
  length  = 8
  special = false
  upper   = false
  numeric  = false
}

resource "aws_s3_bucket" "bucket" {
  bucket        = "kops-state-store-${random_string.id.result}"
  force_destroy = true
  tags = {
    Project = "twitter-phase-2"
  }
}
resource "aws_s3_bucket_versioning" "bucket" {
  bucket = "kops-state-store-${random_string.id.result}"
  versioning_configuration {
    status = "Enabled"
  }
}


resource "aws_s3_bucket_public_access_block" "bucket_block" {
  bucket                  = aws_s3_bucket.bucket.id
  block_public_acls       = true
  block_public_policy     = true
  ignore_public_acls      = true
  restrict_public_buckets = true
}

output "bucket_name" {
  value = aws_s3_bucket.bucket.bucket
}
