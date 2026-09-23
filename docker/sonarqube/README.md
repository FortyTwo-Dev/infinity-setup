# SonarQube (local)

Local SonarQube instance for code quality analysis.

## Requirements

- Docker and Docker Compose.

## Usage

```bash
cd docker/sonarqube
docker compose up -d
```

First boot can take a few minutes (database init + Elasticsearch). Wait until
`http://localhost:9000` responds.

- URL: http://localhost:9000
- Default credentials: `admin` / `admin` (you will be prompted to change it).

## Analyze a project

From the repository root (the SonarQube instance must be running):

```bash
docker run --rm --network host \
  -v "$(pwd):/usr/src" \
  sonarsource/sonar-scanner-cli:latest \
  -Dsonar.host.url=http://localhost:9000 \
  -Dsonar.token=sqp_0b2921a10439c0c0333dcab3f7d05bf748e727c4 \
  -Dsonar.projectKey=infinity-setup \
  -Dsonar.sources=src \
  -Dsonar.sourceEncoding=UTF-8
```

The token above is the local-only project token (`sqp_...`). It has no value
outside this local instance. Results are available at
`http://localhost:9000/dashboard?id=infinity-setup`. Project-level defaults live
in `sonar-project.properties` at the repo root.

## Stop

```bash
docker compose down            # stop, keep data volumes
docker compose down -v         # stop and remove data volumes
```

## Notes

- SonarQube uses an embedded Elasticsearch. On Linux, set the recommended host
  limits before starting (as root):

  ```bash
  sysctl -w vm.max_map_count=524288
  sysctl -w fs.file-max=131072
  ulimit -n 131072
  ulimit -u 8192
  ```

- The image is pinned to SonarQube **Community Build** (`26.9.0.129388-community`).
  The commercial editions are tagged `developer`, `enterprise`, and
  `datacenter-app`/`datacenter-search`. Long-Term Active (LTA) versions use
  tags like `2026-lta-developer`.

- Default database credentials are `sonar` / `sonar` (local only). Change them
  in `docker-compose.yml` if needed.
