CLIENT_ID="your_client_id"
CLIENT_SECRET="your_client_secret"

SUBSCRIPTION_ID="XXXX-464b-4b87-acbe-YYYY"
TENANT_ID="XXXX-9959-4d0a-a454-YYYY"

START_DATE="2023-12-01"
END_DATE="2023-12-06"

ACCESS_TOKEN=$(az account get-access-token --output json --resource https://management.azure.com | jq -r .accessToken)
API_ENDPOINT="https://management.azure.com/subscriptions/${SUBSCRIPTION_ID}/providers/Microsoft.CostManagement/query?api-version=2019-11-01"

QUERY_PAYLOAD='{"type":"Usage","timeframe":"Custom","timePeriod":{"start": "'${START_DATE}'","end": "'${END_DATE}'"},"dataset":{"granularity": "Daily","aggregation": {"totalCost": {"name": "PreTaxCost","function": "Sum"}}}}'

curl -X POST -H "Content-Type: application/json" -H "Authorization: Bearer ${ACCESS_TOKEN}" --data "${QUERY_PAYLOAD}" ${API_ENDPOINT}
