This directory is just a rename of the `dist` directory spit out by Trunk. It's named `docs` so that it can be used as the deployment source for a GitHub pages site. The project does not warrant a CI/CD flow at this time. The following command was used to generate this directory:

```
trunk build --release --public-url /rubiks-simulator/ --dist docs
```
