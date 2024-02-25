FROM node:lts-gallium as build

WORKDIR /build
ADD ./front-end /build/
RUN npm install && npm run build

FROM nginx

COPY --from=build /build/dist/  /usr/share/nginx/html/
COPY ./nginx/default.conf /etc/nginx/conf.d/default.conf