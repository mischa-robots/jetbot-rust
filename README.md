# jetbot-rust

Basic Jetbot (with Jetson Nano) written in Rust lang running in a docker container based on [dustynv/onnxruntime:r32.7.1](https://github.com/dusty-nv/jetson-containers/tree/master/packages/onnxruntime) .

To use this repository for your robot project, give it first a star, then fork it to your Github account.

[Docker installation](https://docs.docker.com/engine/install/ubuntu/) is required, check also the [Jetson Nano docs](https://developer.nvidia.com/embedded/learn/tutorials/jetson-container), please.

Also make sure that you have the latest L4T version installed on your Jetson Nano (32.7.1)

## Setup project on your Jetbot

Clone your repository to your Jetbot:

```
git clone git@github.com:mischa-robots/jetbot-rust.git
```

Then start the docker container:

```
cd jetbot-rust
docker compose up
```

## Access and develop your Jetbot from your local machine

Read [VSCode Remote SSH](https://code.visualstudio.com/remote/advancedcontainers/develop-remote-host) to learn how you can access the Jetbot from your local Desktop VSCode instance and develop directly on the Jetbot.

### License: GNU AGPL

    Copyright (C) 2024 Mischa (Michael Schaefer)

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as published
    by the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.