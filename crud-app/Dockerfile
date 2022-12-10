# syntax=docker/dockerfile:1

FROM python:3.9

WORKDIR /app

RUN apt -y update
RUN apt -y upgrade
RUN apt install -y sqlite3 libsqlite3-dev


COPY ./requirements.txt ./requirements.txt

RUN pip install --no-cache-dir --upgrade -r requirements.txt

COPY . .

CMD uvicorn main:app --host 0.0.0.0 --port 8000