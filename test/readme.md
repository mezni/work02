{'TimePeriod': {'Start': '2023-11-06', 'End': '2023-11-07'}, 'Total': {'BlendedCost': {'Amount': '0', 'Unit': 'USD'}}, 'Groups': [], 'Estimated': False}



AWS_REGION="ca-central-1"
START_DATE="2023-12-05"
END_DATE="2023-12-06"


QUERY_PARAMS="{\"TimePeriod\":{\"Start\":\"$START_DATE\",\"End\":\"$END_DATE\"},\"Granularity\":\"MONTHLY\",\"Metrics\":[\"BlendedCost\"],\"GroupBy\":[{\"Type\":\"DIMENSION\",\"Key\":\"SERVICE\"}]}"


curl -X POST -H "Content-Type: application/x-amz-json-1.1" --data "$QUERY_PARAMS" https://ce.us-east-1.amazonaws.com
https://ce.us-east-1.amazonaws.com



curl -X POST -H "Content-Type: application/json" -d '{"Action":"GetCostAndUsage","Version":"2017-10-25" }' https://ce.us-east-1.amazonaws.com/





# Set your AWS credentials and region
export AWS_ACCESS_KEY_ID="your_access_key"
export AWS_SECRET_ACCESS_KEY="your_secret_key"
export AWS_REGION="your_aws_region"

# Set your start and end date
START_DATE="2023-12-01"
END_DATE="2023-12-31"

# Set your metrics and dimensions
METRICS="BlendedCost"
DIMENSIONS="SERVICE"

# Set your query parameters
QUERY_PARAMS='{"TimePeriod":{"Start":"'${START_DATE}'","End":"'${END_DATE}'"},"Granularity":"MONTHLY","Metrics":["'${METRICS}'"],"GroupBy":[{"Type":"DIMENSION","Key":"'${DIMENSIONS}'"}]}'

API_ENDPOINT="https://ce.${AWS_REGION}.amazonaws.com/"

# Perform the AWS Signature Version 4 signing
AWS_DATE=$(date -u +%Y%m%dT%H%M%SZ)
AWS_REGIONAL_DATE=$(echo -n $AWS_DATE | cut -c 1-8)
AWS_SERVICE="ce"
AWS_REQUEST_PAYLOAD=$(echo -n $QUERY_PARAMS)
AWS_CANONICAL_REQUEST="POST\n/\n\ncontent-type:application/x-amz-json-1.1\nhost:${AWS_SERVICE}.${AWS_REGION}.amazonaws.com\nx-amz-date:${AWS_DATE}\n\ncontent-type;host;x-amz-date\n$(echo -n $AWS_REQUEST_PAYLOAD | sha256sum | cut -c 1-64)"
AWS_STRING_TO_SIGN="AWS4-HMAC-SHA256\n${AWS_DATE}\n${AWS_REGIONAL_DATE}/${AWS_SERVICE}/aws4_request\n$(echo -n $AWS_CANONICAL_REQUEST | sha256sum | cut -c 1-64)"
AWS_SIGNATURE=$(echo -n $AWS_STRING_TO_SIGN | openssl dgst -sha256 -hmac ${AWS_SECRET_ACCESS_KEY} | cut -c 1-64)

# Make the API request using curl
curl -X POST -H "Content-Type: application/x-amz-json-1.1" -H "X-Amz-Date: ${AWS_DATE}" -H "Authorization: AWS4-HMAC-SHA256 Credential=${AWS_ACCESS_KEY_ID}/${AWS_REGIONAL_DATE}/${AWS_SERVICE}/aws4_request, SignedHeaders=content-type;host;x-amz-date, Signature=${AWS_SIGNATURE}" --data "${QUERY_PARAMS}" ${API_ENDPOINT}



AWS_DATE=$(date -u +%Y%m%dT%H%M%SZ)
QUERY_PARAMS='{"TimePeriod":{"Start":"'${START_DATE}'","End":"'${END_DATE}'"},"Granularity":"MONTHLY","Metrics":["'BlendedCost'"],"GroupBy":[{"Type":"DIMENSION","Key":"'SERVICE'"}]}'
API_ENDPOINT="https://ce.us-east-1.amazonaws.com/"
curl -X POST -H "Content-Type: application/x-amz-json-1.1" -H "X-Amz-Date: ${AWS_DATE}" -H "Authorization: AWS4-HMAC-SHA256 Credential=${AWS_ACCESS_KEY_ID}/${AWS_REGIONAL_DATE}/${AWS_SERVICE}/aws4_request, SignedHeaders=content-type;host;x-amz-date, Signature=${AWS_SIGNATURE}" --data "${QUERY_PARAMS}" ${API_ENDPOINT}



export AWS_ACCESS_KEY_ID="AKIAUWWMWB6LACI5COUO"
export AWS_SECRET_ACCESS_KEY="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"
export AWS_REGION="ca-central-1"

START_DATE="2023-12-01"
END_DATE="2023-12-31"

METRICS="BlendedCost"
DIMENSIONS="SERVICE"

QUERY_PARAMS='{"TimePeriod":{"Start":"'${START_DATE}'","End":"'${END_DATE}'"},"Granularity":"MONTHLY","Metrics":["'BlendedCost'"],"GroupBy":[{"Type":"DIMENSION","Key":"'LINKED_ACCOUNT'"},{"Type":"DIMENSION","Key":"'SERVICE'"]}'

API_ENDPOINT="https://ce.${AWS_REGION}.amazonaws.com/"

# Perform the AWS Signature Version 4 signing
AWS_DATE=$(date -u +%Y%m%dT%H%M%SZ)
AWS_REGIONAL_DATE=$(echo -n $AWS_DATE | cut -c 1-8)
AWS_SERVICE="ce"
AWS_REQUEST_PAYLOAD=$(echo -n $QUERY_PARAMS)
AWS_CANONICAL_REQUEST="POST\n/\n\ncontent-type:application/x-amz-json-1.1\nhost:${AWS_SERVICE}.${AWS_REGION}.amazonaws.com\nx-amz-date:${AWS_DATE}\n\ncontent-type;host;x-amz-date\n$(echo -n $AWS_REQUEST_PAYLOAD | sha256sum | cut -c 1-64)"
AWS_STRING_TO_SIGN="AWS4-HMAC-SHA256\n${AWS_DATE}\n${AWS_REGIONAL_DATE}/${AWS_SERVICE}/aws4_request\n$(echo -n $AWS_CANONICAL_REQUEST | sha256sum | cut -c 1-64)"
AWS_SIGNATURE=$(echo -n $AWS_STRING_TO_SIGN | openssl dgst -sha256 -hmac ${AWS_SECRET_ACCESS_KEY} | cut -c 1-64)

# Make the API request using curl
curl -X POST -H "Content-Type: application/x-amz-json-1.1" -H "X-Amz-Date: ${AWS_DATE}" -H "Authorization: AWS4-HMAC-SHA256 Credential=${AWS_ACCESS_KEY_ID}/${AWS_REGIONAL_DATE}/${AWS_SERVICE}/aws4_request, SignedHeaders=content-type;host;x-amz-date, Signature=${AWS_SIGNATURE}" --data "${QUERY_PARAMS}" ${API_ENDPOINT}

In this example: