from flask import Flask, jsonify, request, render_template
from flask_restx import Api, Resource
from flask_cors import CORS, cross_origin

import pika
import uuid
import json



#<ggbaker@sfu.ca>
class DemoRpcClient(object):
    def __init__(self):
        self.connection = pika.BlockingConnection(pika.ConnectionParameters(host='rabbit'))
        self.channel = self.connection.channel()

        result = self.channel.queue_declare(queue='', exclusive=True)
        self.callback_queue = result.method.queue

        self.channel.basic_consume(
            queue=self.callback_queue,
            on_message_callback=self.on_response,
            auto_ack=True)

    def on_response(self, ch, method, props, body):
        if self.corr_id == props.correlation_id:
            self.response = json.loads(body.decode('utf-8'))

    def call(self, arg):
        self.response = None
        self.corr_id = str(uuid.uuid4())
        body = json.dumps(arg).encode('utf8')
        
        self.channel.basic_publish(
            exchange='',
            routing_key='rpc_queue',
            properties=pika.BasicProperties(reply_to=self.callback_queue, correlation_id=self.corr_id),
            body=body)
        while self.response is None:
            self.connection.process_data_events()
        return self.response



app = Flask(__name__)



#https://stackoverflow.com/questions/50417461/python-flask-return-html-content
@app.route('/', methods=['GET'])
def home():
  return render_template('table.html')


#https://justkode.kr/python/flask-restapi-1
@app.route("/find", methods=["POST"])
def post_example():
    """POST in server"""
    
    print(request.json)
    rpc = DemoRpcClient()
    response = rpc.call(request.json)
    print(response)


    return jsonify(message=response.get("result"))

if __name__ == "__main__":
    app.run(debug=True, host='0.0.0.0', port=80)