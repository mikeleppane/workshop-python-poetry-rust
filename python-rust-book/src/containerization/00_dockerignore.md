# .dockerignore

Since Docker copies all files from the building folder, we will to add the `.dockerignore` file to the project root that contains the patterns of paths to avoid copying these files. It's useful, for example, to skip the target building folder, because it may contain large files that are not needed in the final image.

```dockerignore
**/target/
**/Cargo.lock

.dockerignore
.git
.gitignore
.vscode
**/__pycache__
```

```admonish info title="Docker's .dockerignore file"
You can read more about Docker's .dockerignore file [here](https://docs.docker.com/build/building/context/#dockerignore-files).
```
