FROM node:10
RUN mkdir /usr/src/server_node
WORKDIR /usr/src/server_node
COPY package.json /usr/src/server_node
RUN npm install
COPY . /usr/src/server_node
CMD ["npm", "start"]
