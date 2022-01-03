#!/usr/bin/env bash
bundle exec jekyll clean
bundle exec jekyll build

deploy_env=${1:-PREVIEW}
# Force change by default, use inc as 2nd parameter for incremental update
if [ -z ${2} ]; then
  force="-force"
else
  force=""
fi
# Uppercase the arg
deploy_env=${deploy_env^^}

# Read in the secrets
source .env
echo "Deploying to ${deploy_env} $force"
bucket=${deploy_env}_BUCKET
declare ${deploy_env}_BUCKET

key=${deploy_env}_KEY
declare ${deploy_env}_KEY

secret=${deploy_env}_SECRET
declare ${deploy_env}_SECRET

cdn=${deploy_env}_CDN
declare ${deploy_env}_CDN

export AWS_ACCESS_KEY_ID=${!key}
export AWS_SECRET_ACCESS_KEY=${!secret}

s3deploy -bucket ${!bucket} -region $REGION  -public-access -source _site -distribution-id ${!cdn} ${force}