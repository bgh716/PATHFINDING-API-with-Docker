FROM python:3.8

RUN pip3 install flask
RUN pip3 install flask_restx
RUN pip3 install flask_cors
RUN pip3 install pika
RUN pip3 install uuid

WORKDIR /app
COPY . .

CMD python3 REST_API.py