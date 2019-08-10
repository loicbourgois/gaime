FROM node:10
RUN mkdir /usr/src/client
WORKDIR /usr/src/client
COPY package.json /usr/src/client
RUN npm install -g @angular/cli
RUN npm install
COPY . /usr/src/client
CMD ["ng", "serve"]
