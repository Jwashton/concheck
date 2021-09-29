# concheck

Test TCP connections to multiple ports on an array of servers

## Usage

```bash
$ concheck ./inventory.yml
```

## Inventory format

```yml
---
roles:
  - name: webservers
    services:
      ssh: true
      http: true
      https: true
    servers:
      - my-blue-web
      - my-green-web

  - name: dbservers
    services:
      ssh: true
      postgresql: true
      other:
        4000: true
    servers:
      - my-db1
      - my-db2

  - name: public-names
    services:
      ssh: false
      http: true
      https: true
    servers:
      - example.org
```

## Known services

| Name         | Port |
|:-------------|-----:|
| `ssh`        |   22 |
| `http`       |   80 |
| `https`      |  443 |
| `mariadb`    | 3306 |
| `postgresql` | 5432 |

Feel free to add additional services via pull request, or check them manually using `other`.

```
  - name: dbservers
    services:
      ssh: true
      postgresql: true
      other:
        4000: true
    servers:
      - my-db1
      - my-db2
```
