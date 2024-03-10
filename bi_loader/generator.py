import uuid, random
import pandas as pd


def read_config_file(file_name):
    output = []
    df = pd.read_csv(file_name)
    records = df.to_dict(orient="records")
    for rec in records:
        tags = {}
        for tag in rec["tags"].split("|"):
            tags[tag.split("=")[0]] = tag.split("=")[1]
        rec["tags"] = tags
        output.append(rec)
    return output


def write_config_file(records, file_name):
    output = []
    for rec in records:
        tags = []
        for k, v in rec["tags"].items():
            tags.append(k + "=" + v)
        rec["tags"] = "|".join(tags)
        meta = []
        for k, v in rec["meta"].items():
            meta.append(k + "=" + v)
        rec["meta"] = "|".join(meta)
        output.append(rec)
    df_output = pd.DataFrame(output)
    df_output.to_csv(file_name, index=False)


imput_file_name = "accounts.csv"
accounts = read_config_file(imput_file_name)
