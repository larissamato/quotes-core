# Quotes Core
It's a project built in Rust, which aims to create a fun and collaborative server where users can store and share memorable and funny quotes spoken at work. The project is a friendly challenge between friends and team members, using technologies such as Rocket, Diesel and Serde. Currently has PostgreSQL configuration via Docker for data management.

## 🚧 Project Status: In Development 🚧

This project is still a work in progress! I'm excited to share it once it's more developed, but feel free to take a look at what's been set up so far.


### Requirements

Before you can run the project, ensure you have the following installed on your system:

- [Rust](https://www.rust-lang.org/) (stable)
- [Docker](https://www.docker.com/)
- [Docker Compose](https://docs.docker.com/compose/)

### Cloning the Repository

1. Clone the repository:

   ```bash
   git clone https://github.com/your-username/quotes-core.git
   cd quotes-core
   ```

### Setting up PostgreSQL with Docker

```bash
docker-compose up -d
```

This will start the PostgreSQL database in the background. You can check the database status using:

```bash
docker-compose ps
```

### Running the Server

```bash
cargo run
```

This will start the Rocket server and connect it to the PostgreSQL instance.

### Database Migrations

We'll be using Diesel for database migrations. You can run:

```bash
diesel migration run
```

Make sure you have the proper configuration in `diesel.toml` and that the PostgreSQL container is running.
