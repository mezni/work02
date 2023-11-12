FROM python:3.10-alpine as generator-service
WORKDIR /code
COPY ./requirements.txt /code/requirements.txt
RUN pip install --no-cache-dir --upgrade -r /code/requirements.txt
COPY ./src/main.py /code
COPY ./src/logging.conf /code
COPY ./src/.env /code
COPY ./src/app /code/app
CMD ["uvicorn", "main:app", "--host", "0.0.0.0", "--port", "8000"]