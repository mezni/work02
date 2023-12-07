#!/bin/bash

# Replace these values with your AWS credentials
AWS_ACCESS_KEY="AKIAUWWMWB6LACI5COUO"
AWS_SECRET_KEY="boXxViBNDrGIP2dCRSFb8rYT8PoQmH/nEBTi1hVg"
AWS_REGION="us-east-1"

# AWS Cost Explorer API information
AWS_SERVICE="ce"
AWS_ENDPOINT="ce.${AWS_REGION}.amazonaws.com"

# Request information
HTTP_METHOD="POST"
REQUEST_URI="/"
REQUEST_QUERY=""
REQUEST_BODY='{"TimePeriod": {"Start": "2023-12-01", "End": "2023-12-06"}, "Granularity": "DAILY", "Metrics": ["BlendedCost"], "GroupBy": [{"Type": "DIMENSION", "Key": "SERVICE"}]}'

# Generate timestamp
AWS_DATE=$(date -u +"%Y%m%dT%H%M%SZ")
DATE_STAMP=$(date -u +"%Y%m%d")

# Generate canonical request
CANONICAL_REQUEST=$(cat <<EOF
${HTTP_METHOD}
${REQUEST_URI}
${REQUEST_QUERY}
host:${AWS_ENDPOINT}
x-amz-date:${AWS_DATE}

host;x-amz-date
$(printf "%s" "${REQUEST_BODY}" | openssl dgst -sha256 -hex | awk '{print $2}')
EOF
)

# Generate string to sign
STRING_TO_SIGN=$(cat <<EOF
AWS4-HMAC-SHA256
${AWS_DATE}
${DATE_STAMP}/${AWS_REGION}/${AWS_SERVICE}/aws4_request
$(printf "%s" "${CANONICAL_REQUEST}" | openssl dgst -sha256 -hex | awk '{print $2}')
EOF
)

# Generate signing key
SIGNING_KEY=$(echo -n "AWS4${AWS_SECRET_KEY}" | \
 openssl sha256 -hex - | \
 cut -d' ' -f2 | \
 xxd -r -p | \
 openssl sha256 -hex - | \
 cut -d' ' -f2 | \
 xxd -r -p | \
 openssl sha256 -hex - | \
 cut -d' ' -f2 | \
 xxd -r -p)

echo  $SIGNING_KEY
# Generate signature
SIGNATURE=$(printf "%s" "${STRING_TO_SIGN}" | \
    openssl dgst -sha256 -hex -mac HMAC -macopt hexkey:"${SIGNING_KEY}" | \
    awk '{print $2}'
)
# Generate Authorization header
AUTH_HEADER="AWS4-HMAC-SHA256 Credential=${AWS_ACCESS_KEY}/${DATE_STAMP}/${AWS_REGION}/${AWS_SERVICE}/aws4_request, SignedHeaders=host;x-amz-date, Signature=${SIGNATURE}"

# Make the request using curl
curl -X ${HTTP_METHOD} \
    -H "host: ${AWS_ENDPOINT}" \
    -H "x-amz-date: ${AWS_DATE}" \
    -H "Authorization: ${AUTH_HEADER}" \
    -H "Content-Type: application/x-amz-json-1.1" \
    -d "${REQUEST_BODY}" \
    "https://${AWS_ENDPOINT}${REQUEST_URI}"
