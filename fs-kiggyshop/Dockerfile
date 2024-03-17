# syntax=docker/dockerfile:1


# Use an Alpine Linux base image for its speed and small size
FROM alpine:latest

# Install Nginx and Certbot then create a directory for the application
RUN apk add --no-cache nginx certbot certbot-nginx && mkdir -p /var/www/html

# Copy the dist directory from your frontend build
COPY ../frontend/dist /var/www/html

# Remove the default server definition and copy our nginx configuration
RUN rm /etc/nginx/conf.d/default.conf
COPY nginx.conf /etc/nginx/nginx.conf
COPY kiggyshop /etc/nginx/sites-available/kiggyshop

# Create a symbolic link to enable the site
RUN ln -s /etc/nginx/sites-available/kiggyshop /etc/nginx/sites-enabled/kiggyshop

# Expose ports 80 and 443 for HTTP and HTTPS traffic
EXPOSE 80 443

# Start Nginx in the foreground
CMD ["nginx", "-g", "daemon off;"]

