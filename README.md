# PATHFINDING-API-with-Docker
# REFERENCE:
The base of SAPF.rs algorithm cames from CMPT417 Course (single_agent_path_finding.py)
RabbitMQ and Dockerfiles are refferenced from Dr.Greg Baker
Other codes are refferenced in the source code

# A. What is the overall goal of the project (i.e. what does it do, or what problem is it solving)?
1. Pathfinding in web application

# B. Which languages did you use, and what parts of the system are implemented in each?
1. Javascript for web application and REST API client
2. Python for web server, REST API server, and RabbitMQ client
3. Rust for pathfinding application(complex calculation), and RabbitMQ server

# C. What methods did you use to communicate between languages?
1. REST(Flask) between javascript and Python
2. RabbitMQ between Python and Rust

# D. Exactly what steps should be taken to get the project working, after getting your code?
[This should start with vagrant up or docker-compose up and can include one or two commands to start components after that.]
1. Enter this command in terminal: docker-compose build && docker-compose up
2. CAUTION! I do not know why, but Rust is too slow to be built in docker. After finishing the build of RabbitMQ, docker stops for few minutes. Please wait until rust shows a "LISTENING" statement which means rust is ready for requests.
3. Visit "localhost" in the web
4. Follow the guide there

# E. What features should we be looking for when marking your project?
1. Pathfinding algorithm is a high cost searching algorithm; therefore, it is implemented in Rust which is faster than Python
2. Python connects two languages(JS and Rust) with short code and useful libraries(flask and rabbitmq) with small overhead
3. Python runs web application from JS in simple code
