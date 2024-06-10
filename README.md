# QR Shirts

This project is an Actix Web server for managing QR shirts. It includes a PostgreSQL database for storing shirt data.

## Prerequisites

Before you begin, ensure you have the following software installed on your local machine:

- Docker: [Installation Guide](https://docs.docker.com/get-docker/)
- Docker Compose: [Installation Guide](https://docs.docker.com/compose/install/)
- Rust and Cargo: [Installation Guide](https://www.rust-lang.org/tools/install)

## Getting Started

### 1. Clone the Repository

Clone this repository to your local machine:

```bash
git clone https://github.com/your-username/qr-shirts.git
cd qr-shirts
```

### 2. Set up the Envrionment

Create a `.env` file in the root directory with the following content:

```dotenv
DATABASE_URL=postgres://user:password@localhost:5432/qr_shirts
```

### 3. Start the PostgreSQL Database

Navigate to the docker directory and start the PostgreSQL database using Docker Compose:

```bash
cd docker
docker-compose up -d
```

This will start the PostgreSQL database in the background.

### 4. Run Actix Web Server

Navigate back to the root directory and run the server:

```bash
cargo run
```

This will start the Actix Web server. The server will be accessible at `http://localhost:8080`.

## Accessing the PostrgeSQL Database

You can access the PostgreSQL database using the psql command-line tool. First, get the container ID or name of the running PostgreSQL container:

```bash
docker ps
```

Then, use the following command to access the database:

```bash
docker exec -it <container_id> psql -U user -d qr_shirts
```

## Stopping the Service

To stop the PostgreSQL database, run:

```bash
cd docker
docker-compose down
```
