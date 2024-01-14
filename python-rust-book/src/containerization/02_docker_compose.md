# docker-compose.yml

Even though we can run our API service with Docker, it's not very convenient. We have to build the Docker image and then run the Docker container. We can make this process a lot easier by using docker-compose. We will create a docker-compose.yml file to run our API service.
Create a new file called `docker-compose.yml` in the root of the project and add the following code to it:

```yaml
version: '3.8'

services:
  backend:
    build:
      context: .
      dockerfile: Dockerfile
    ports:
      - "8000:8000"
    expose:
      - 8000
    healthcheck:
      test: curl --fail backend:8000/healthz || exit 1
      interval: 30s
      retries: 5
      start_period: 20s
      timeout: 5s
    restart: unless-stopped
```

Now we are ready to run our API service with docker-compose! 🎉
