mkdir finops_layer
cd finops_layer
python3.11 -m venv venv
source ./venv/bin/activate
pip install -r ../requirements.txt 
deactivate
find . -name __pycache__ | xargs rm -r
mkdir python
cd python
cp -r ../venv/lib64/python3.11/site-packages/* .
cd ..
zip -r ../finops_layer.zip python