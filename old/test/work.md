docker run -p 8888:8888 -v $(pwd):/home/jovyan/work jupyter/minimal-notebook

cost_df = pd.read_csv('work/finsops_horizoninnovations_aws_20231211.csv')
cost_df['mois']=cost_df['periode'].map(lambda x: x[:7]) 

grouped_single = cost_df.groupby(['mois','compte']).agg({'cout': ['sum']})
grouped_single.columns = ['cout']
grouped_single = grouped_single.reset_index()
grouped_single.head()

fig = plt.figure()
ax1 = fig.add_subplot(111)

ax1.scatter(grouped_single.loc[grouped_single['compte'] == 311653278753]['mois'], grouped_single.loc[grouped_single['compte'] == 311653278753]["cout"], s=10, c='b', marker="s", label='first')
ax1.scatter(grouped_single.loc[grouped_single['compte'] == 322653477654]['mois'],grouped_single.loc[grouped_single['compte'] == 322653477654]["cout"], s=10, c='r', marker="o", label='second')
plt.legend(loc='upper left')
plt.show()

import boto3

start_date = '2022-07-01'
end_date = '2022-08-30'


client = boto3.client('ce')

tags_response = None

try:
    tags_response = client.list_cost_allocation_tags(
        Status='Inactive',  # 'Active'|'Inactive',
        # TagKeys=[
        #     'Key',
        # ],
        Type='UserDefined',  # 'AWSGenerated'|'UserDefined',
        # NextToken='string',
        # MaxResults=100,
    )
except Exception as e:
    print(e)

cost_allocation_tags = tags_response['CostAllocationTags']
print(cost_allocation_tags)

print("-"*5+' Input TagValues with comma separation '+"-"*5)

for cost_allocation_tag in cost_allocation_tags:

    tag_key = cost_allocation_tag['TagKey']
    tag_type = cost_allocation_tag['Type']
    tag_status = cost_allocation_tag['Status']

    tag_values = str(input(
        f'TagKey: {tag_key}, Type: {tag_type}, Status: {tag_status} -> '))

    if tag_values == "":
        continue
    tag_values_parsed = tag_values.strip().split(',')
    if tag_values_parsed == []:
        continue

    cost_usage_response = None
    try:
        cost_usage_response = client.get_cost_and_usage(
            TimePeriod={
                'Start': start_date,
                'End': end_date
            },
            Metrics=['AmortizedCost'],
            Granularity='MONTHLY',  # 'DAILY'|'MONTHLY'|'HOURLY',
            Filter={
                'Tags': {
                    'Key': tag_key,
                    'Values': tag_values_parsed,
                    'MatchOptions': [
                        'EQUALS'  # 'EQUALS'|'ABSENT'|'STARTS_WITH'|'ENDS_WITH'|'CONTAINS'|'CASE_SENSITIVE'|'CASE_INSENSITIVE',
                    ]
                },
            },
            # GroupBy=[
            #     {
            #         'Type': 'SERVICE',  # 'DIMENSION'|'TAG'|'COST_CATEGORY',  # AZ, INSTANCE_TYPE, LEGAL_ENTITY_NAME, INVOICING_ENTITY, LINKED_ACCOUNT, OPERATION, PLATFORM, PURCHASE_TYPE, SERVICE, TENANCY, RECORD_TYPE , and USAGE_TYPE
            #         'Key': 'string',
            #     },
            # ],
        )
        print(cost_usage_response)
    except Exception as e:
        print(e)

