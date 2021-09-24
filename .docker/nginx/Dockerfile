FROM nginx:latest

RUN apt-get update && apt-get install -y vim nano python-certbot-nginx --fix-missing

ADD ssl /etc/nginx/ssl
ADD nginx.conf /etc/nginx/nginx.conf

CMD ["nginx"]

EXPOSE 443
