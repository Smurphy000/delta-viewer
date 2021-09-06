# Delta Lake Viewer

This project is currently in development using [Rocket](https://github.com/SergioBenitez/Rocket) for the backend and [Yew](https://github.com/yewstack/yew) for the frontend.

Additionally, this project is dependent upon [Delta](https://github.com/delta-io/delta) and the rust implementation [Delta-rs](https://github.com/delta-io/delta-rs)

## Goals

- Create an easy way to visually consume information in one or many delta lakes
- Provide supplemental information using delta table metadata such as table update frequency, schema evolution, etc
- Allow users to host the viewer application and connect to their own delta lakes
