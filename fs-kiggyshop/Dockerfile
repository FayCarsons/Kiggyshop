# syntax=docker/dockerfile:1

# Dockerfile for main app Nginx server
FROM nginx:latest 

RUN apt-get update && apt-get install -y certbot

VOLUME /etc/letsencrypt/

COPY frontend/dist /var/www/html

RUN rm /etc/nginx/nginx.conf
COPY fs-kiggyshop/nginx.conf /etc/nginx/nginx.conf
COPY fs-kiggyshop/kiggyshop /etc/nginx/sites-available/kiggyshop

RUN mkdir /etc/nginx/sites-enabled && ln -s /etc/nginx/sites-available/kiggyshop /etc/nginx/sites-enabled/kiggyshop

# Expose ports 80 and 443 for HTTP and HTTPS traffic
EXPOSE 80 443

# Start Nginx in the foreground
CMD ["nginx", "-g", "daemon off;"]

