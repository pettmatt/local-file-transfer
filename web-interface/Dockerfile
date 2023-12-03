FROM node:20-alpine
WORKDIR /usr/src/web-interface

COPY package*.json ./
RUN npm install

COPY . .
RUN npm run build

EXPOSE 5173
CMD ["npm", "start"]