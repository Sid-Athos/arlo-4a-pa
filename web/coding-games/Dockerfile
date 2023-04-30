FROM node:lts-alpine3.16 AS builder
WORKDIR /web/coding-games
COPY package*.json ./
COPY tsconfig.json ./
COPY vite.config.js ./
COPY src ./src
COPY index.html ./index.html
RUN npm install
RUN npm run build

FROM node:lts-alpine3.16
WORKDIR /web/coding-games
COPY package*.json ./
COPY vite.config.js ./
RUN npm install --only=production
COPY --from=builder /web/coding-games/dist ./
CMD ["npm", "run", "start"]
