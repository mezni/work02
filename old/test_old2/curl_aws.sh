# Set your AWS credentials and region
export AWS_ACCESS_KEY_ID=""
export AWS_SECRET_ACCESS_KEY=""
export AWS_REGION="us-east-1"

# Set your AWS service and endpoint
AWS_SERVICE="ce"
AWS_ENDPOINT="https://ce.${AWS_REGION}.amazonaws.com/"

# Set your request parameters
START_DATE="2023-12-01"
END_DATE="2023-12-06"
METRICS="BlendedCost"
DIMENSIONS="SERVICE"

# Set the API request payload
REQUEST_PAYLOAD='{"TimePeriod":{"Start":"'${START_DATE}'","End":"'${END_DATE}'"},"Granularity":"MONTHLY","Metrics":["'${METRICS}'"],"GroupBy":[{"Type":"DIMENSION","Key":"'${DIMENSIONS}'"}]}'

# Set the current timestamp
AWS_DATE=$(date -u +"%Y%m%dT%H%M%SZ")
AWS_REGIONAL_DATE=$(date -u +"%Y%m%d")

# Generate the canonical request hash
CANONICAL_REQUEST_HASH=$(echo -n -e "POST\n/\n\ncontent-type:application/x-amz-json-1.1\nhost:${AWS_SERVICE}.${AWS_REGION}.amazonaws.com\nx-amz-date:${AWS_DATE}\n\ncontent-type;host;x-amz-date\n$(echo -n -e $REQUEST_PAYLOAD | sha256sum | cut -d ' ' -f 1)" | sha256sum | cut -d ' ' -f 1)

# Generate the string to sign
STRING_TO_SIGN=$(echo -n -e "AWS4-HMAC-SHA256\n${AWS_DATE}\n${AWS_REGIONAL_DATE}/${AWS_SERVICE}/aws4_request\n${CANONICAL_REQUEST_HASH}" | openssl sha256 -hex | cut -d ' ' -f 2)

# Generate the signing key
SIGNING_KEY=$(printf "%s" "AWS4${AWS_SECRET_ACCESS_KEY}" | xxd -p -c 256 | { cat -; echo; } | openssl sha256 -hex -mac HMAC -macopt hexkey:$(printf "%s" $AWS_REGIONAL_DATE | xxd -p -c 256) | cut -d ' ' -f 2)

# Generate the signature
SIGNATURE=$(printf "%s" $STRING_TO_SIGN | openssl sha256 -hex -mac HMAC -macopt hexkey:$SIGNING_KEY | cut -d ' ' -f 2)

# Construct the Authorization header
AUTHORIZATION_HEADER="AWS4-HMAC-SHA256 Credential=${AWS_ACCESS_KEY_ID}/${AWS_REGIONAL_DATE}/${AWS_SERVICE}/aws4_request, SignedHeaders=content-type;host;x-amz-date, Signature=${SIGNATURE}"

# Make the API request using curl
echo curl -v -X POST -H "Content-Type: application/x-amz-json-1.1" -H "X-Amz-Date: ${AWS_DATE}" -H "Authorization: $AUTHORIZATION_HEADER" --data "$REQUEST_PAYLOAD" $AWS_ENDPOINT
