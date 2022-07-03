# kvu

Simplest tool to update/add key-value pairs lines

```
┌──────────────────────────────┐                                             ┌──────────────────────────────┐
│ TOKEN=1ad543a3               │                                             │ TOKEN=7b1eacae               │
│ DB_URI=postgres://main/table │           ┌────────────────────┐            │ DB_URI=postgres://main/table │
│ DB_USERNAME=username         │───stdin──▶│ kvu TOKEN=7b1eacae │───stdout──▶│ DB_USERNAME=username         │
│ DB_PASSWORD=password         │           └────────────────────┘            │ DB_PASSWORD=password         │
└──────────────────────────────┘                                             └──────────────────────────────┘
```
