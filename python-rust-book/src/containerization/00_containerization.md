# Containerization

As a final step, we will package our API service in a Docker container. We will use a multi-stage build to keep the size of the final image as small as possible. The base image for the final image will be `python:3.12-bookworm`. We will also create a docker-compose.yml file to run our API service.
