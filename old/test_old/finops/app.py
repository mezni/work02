#!/usr/bin/env python3

import aws_cdk as cdk

from finops.finops_stack import FinopsStack


app = cdk.App()
FinopsStack(app, "FinopsStack")

app.synth()
