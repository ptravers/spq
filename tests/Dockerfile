FROM python:3.8.1-buster

WORKDIR /app/

COPY tests/requirements-test.txt /app/requirements-test.txt

RUN pip install --upgrade pip
RUN pip install -r requirements-test.txt

RUN mkdir /app/reports

COPY tests/integration_tests/ /app/tests/
COPY tests/entrypoint.sh /app/entrypoint.sh

RUN touch /app/tests/proto/__init__.py
RUN touch /app/tests/__init__.py

RUN chmod +x /app/entrypoint.sh

ENV PYTHONPATH /app/tests

# The below command is overidden in docker-compose.yml
CMD pytest -s -p no:warnings /app/tests/
